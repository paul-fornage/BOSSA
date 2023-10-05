#include <cstdarg>
#include <stdio.h>

#include "BossaObserver.h"

void
BossaObserver::onStatus(const char *message, ...)
{
    va_list ap;

    va_start(ap, message);
    vprintf(message, ap);
    va_end(ap);
}

void
BossaObserver::onProgress(int num, int div)
{
    int ticks;
    int bars = 30;

    ticks = num * bars / div;

    if (ticks == _lastTicks)
        return;

    printf("\r[");
    while (ticks-- > 0)
    {
        putchar('=');
        bars--;
    }
    while (bars-- > 0)
    {
        putchar(' ');
    }
    printf("] %d%% (%d/%d pages)", num * 100 / div, num, div);
    fflush(stdout);

    _lastTicks = 0;
}

std::unique_ptr<FlasherObserver> new_bossa_observer()
{
    return std::unique_ptr<FlasherObserver>(new BossaObserver());
}
