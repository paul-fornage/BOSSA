#ifndef _BOSSAOBSERVER_H
#define _BOSSAOBSERVER_H

#include "Info.h" // Includes lib.rs.h needed for ObserverCallback
#include "Flasher.h"

using ObserverCallback = void(*)(const std::string &message);
using ProgressCallback = void(*)(int current, int total);

class BossaObserver : public FlasherObserver
{
public:
    BossaObserver(ObserverCallback print, ProgressCallback progress) : _lastTicks(-1) {
        printFunc = print;
        progressFunc = progress;
    }
    virtual ~BossaObserver() {}

    virtual void onStatus(const char *message, ...);
    virtual void onProgress(int num, int div);
private:
    int _lastTicks;
    ObserverCallback printFunc;
    ProgressCallback progressFunc;
};

std::unique_ptr<FlasherObserver> new_bossa_observer_with_progress(ObserverCallback print, ProgressCallback progress);
std::unique_ptr<FlasherObserver> new_bossa_observer(ObserverCallback print);

#endif // _BOSSAOBSERVER_H
