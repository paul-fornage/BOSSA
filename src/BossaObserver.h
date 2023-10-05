#ifndef _BOSSAOBSERVER_H
#define _BOSSAOBSERVER_H

#include "Flasher.h"

class BossaObserver : public FlasherObserver
{
public:
    BossaObserver() : _lastTicks(-1) {}
    virtual ~BossaObserver() {}

    virtual void onStatus(const char *message, ...);
    virtual void onProgress(int num, int div);
private:
    int _lastTicks;
};

std::unique_ptr<FlasherObserver> new_bossa_observer();

#endif // _BOSSAOBSERVER_H
