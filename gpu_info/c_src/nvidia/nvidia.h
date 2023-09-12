#include <nvml.h>
#include <stdint.h>

#define MAX_NAME_LENGTH 96
#define MAX_DEVICE_COUNT 10
#define MAX_PROCESS_COUNT 100

typedef struct {
    char name[MAX_NAME_LENGTH];
    uint32_t deviceId;
    nvmlPciInfo_t pciInfo;
    uint64_t totalMemory;
    uint32_t maxPowerConsumption;
    uint32_t maxGpuFrequency;
    uint32_t maxMemoryFrequency;
} DeviceStaticInfo;

typedef struct {
    uint32_t currentGpuFrequency;
    uint32_t currentMemoryFrequency;
    uint32_t gpuTemperature;
    uint32_t fanSpeedPercentage;
    uint32_t currentPowerConsumption;
    uint64_t rxBytesRate; 
    uint64_t txBytesRate; 
    double gpuUsagePercentage;
    uint64_t usedMemory;
    nvmlProcessInfo_t processes[MAX_PROCESS_COUNT];
    uint32_t processCount;
} DeviceDynamicInfo;
