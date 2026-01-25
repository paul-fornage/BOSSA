#include <cstdarg>
#include <cstdio>
#include <stdio.h>

#include "BossaObserver.h"

std::string format(const char *fmt, va_list args)
{
    char buf[256];
    const auto r = std::vsnprintf(buf, sizeof buf, fmt, args);

    if (r < 0)
    {
        // conversion failed
        return {};
    }

    const size_t len = r;
    if (len < sizeof buf)
    {
        // we fit in the buffer
        return { buf, len };
    }

    std::string s(len, '\0');
    std::vsnprintf((char*)s.data(), len+1, fmt, args);

    return s;
}

void BossaObserver::onStatus(const char *message, ...)
{
    va_list args;
    va_start(args, message);
    printFunc(format(message, args));
    va_end(args);
}

void BossaObserver::onProgress(int num, int div)
{
    if (progressFunc) {
        progressFunc(num, div);
    } else {
		int ticks;
   		int bars = 30;

        ticks = num * bars / div;

        if (ticks == _lastTicks)
            return;

        printFunc("\r[");
        while (ticks-- > 0)
        {
            printFunc("=");
            bars--;
        }
        while (bars-- > 0)
        {
            printFunc(" ");
        }
        onStatus("] %d%% (%d/%d pages)", num * 100 / div, num, div);

        _lastTicks = 0;
	}
}

std::unique_ptr<FlasherObserver> new_bossa_observer_with_progress(ObserverCallback print, ProgressCallback progress)
{
    return std::unique_ptr<FlasherObserver>(new BossaObserver(print, progress));
}

std::unique_ptr<FlasherObserver> new_bossa_observer(ObserverCallback print) 
{
	return new_bossa_observer_with_progress(print, nullptr);    
}

