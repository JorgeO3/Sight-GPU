#include <stdio.h>
#include "nvidia.h"

int main()
{
    unsigned int deviceCount = get_device_count();
    DeviceStaticInfo *infoArray = fetch_all_static_device_info(&deviceCount);
    DeviceDynamicInfo *dynamicInfoArray = fetch_all_dynamic_device_info(&deviceCount);

    printf("Hello World\n");
    printf("Device count: %u\n", deviceCount);
    for (unsigned int i = 0; i < deviceCount; i++)
    {
        printf("Device %u\n", i);
        printf("Name: %s\n", infoArray[i].name);
        printf("Device ID: %u\n", infoArray[i].deviceId);
        printf("PCI Bus ID: %u\n", infoArray[i].pciInfo.bus);
        printf("PCI Device ID: %u\n", infoArray[i].pciInfo.device);
        printf("PCI Domain ID: %u\n", infoArray[i].pciInfo.domain);
        printf("Memory total: %llu\n", infoArray[i].memoryInfo.total);
        printf("Max power consumption: %u\n", infoArray[i].maxPowerConsumption);
        printf("Max GPU frequency: %u\n", infoArray[i].maxGpuFrequency);
        printf("Max memory frequency: %u\n", infoArray[i].maxMemoryFrequency);
        printf("Current GPU frequency: %u\n", dynamicInfoArray[i].currentGpuFrequency);
        printf("Current memory frequency: %u\n", dynamicInfoArray[i].currentMemoryFrequency);
        printf("GPU temperature: %u\n", dynamicInfoArray[i].gpuTemperature);
        printf("Fan speed percentage: %u\n", dynamicInfoArray[i].fanSpeedPercentage);
        printf("Current power consumption: %u\n", dynamicInfoArray[i].currentPowerConsumption);
        printf("RX bytes rate: %llu\n", dynamicInfoArray[i].rxBytesRate);
        printf("TX bytes rate: %llu\n", dynamicInfoArray[i].txBytesRate);
        printf("GPU usage percentage: %f\n", dynamicInfoArray[i].gpuUsagePercentage);
        printf("Used memory: %llu\n", dynamicInfoArray[i].usedMemory);
        printf("Process count: %u\n", dynamicInfoArray[i].processCount);
        for (unsigned int j = 0; j < dynamicInfoArray[i].processCount; j++)
        {
            printf("Process %u\n", j);
            printf("PID: %u\n", dynamicInfoArray[i].computeProcesses[j].pid);
            printf("Used memory: %llu\n", dynamicInfoArray[i].computeProcesses[j].usedGpuMemory);
        }
        for (unsigned int j = 0; j < dynamicInfoArray[i].processCount; j++)
        {
            printf("Process %u\n", j);
            printf("PID: %u\n", dynamicInfoArray[i].graphicsProcesses[j].pid);
            printf("Used memory: %llu\n", dynamicInfoArray[i].graphicsProcesses[j].usedGpuMemory);
        }
    }
    return 0;
}
