#include "Info.h"

void flasherinfo2flasherinfors(const FlasherInfo &info, FlasherInfoRs &rsInfo)
{
    rsInfo.name = info.name;
    rsInfo.chipId = info.chipId;
    rsInfo.extChipId = info.extChipId;
    rsInfo.version = info.version;
    rsInfo.address = info.address;
    rsInfo.numPages = info.numPages;
    rsInfo.pageSize = info.pageSize;
    rsInfo.totalSize = info.totalSize;
    rsInfo.numPlanes = info.numPlanes;

    rsInfo.security = info.security;
    rsInfo.bootFlash = info.bootFlash;
    rsInfo.bod = info.bod;
    rsInfo.bor = info.bor;

    rsInfo.canBootFlash = info.canBootFlash;
    rsInfo.canBod = info.canBod;
    rsInfo.canBor = info.canBor;
    rsInfo.canChipErase = info.canChipErase;
    rsInfo.canWriteBuffer = info.canWriteBuffer;
    rsInfo.canChecksumBuffer = info.canChecksumBuffer;

    for (auto lockRegion : info.lockRegions) {
        rsInfo.lockRegions.push_back(lockRegion);
    }
    for (auto elem : info.uniqueId) {
        rsInfo.uniqueId.push_back(elem);
    }
}
