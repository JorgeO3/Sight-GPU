#include <stdio.h>
#include <stdlib.h>
#include <nvml.h>
#include "nvidia.h"

unsigned int get_device_count()
{
    unsigned int count = 0;
    nvmlReturn_t result;

    result = nvmlInit();
    if (result != NVML_SUCCESS)
    {
        fprintf(stderr, "Error al inicializar NVML: %s\n", nvmlErrorString(result));
        return count; // Retorna 0 en caso de error.
    }

    nvmlDeviceGetCount(&count);
    nvmlShutdown();

    return count;
}

DeviceStaticInfo *fetch_all_static_device_info(unsigned int *deviceCount)
{
    nvmlReturn_t result;
    nvmlDevice_t device;

    *deviceCount = get_device_count();
    if (*deviceCount == 0)
    {
        return NULL;
    }

    DeviceStaticInfo *infoArray = malloc(sizeof(DeviceStaticInfo) * (*deviceCount));
    if (!infoArray)
    {
        fprintf(stderr, "Error al asignar memoria para infoArray\n");
        return NULL;
    }

    result = nvmlInit();
    if (result != NVML_SUCCESS)
    {
        fprintf(stderr, "Error al inicializar NVML: %s\n", nvmlErrorString(result));
        free(infoArray);
        return NULL;
    }

    for (unsigned int i = 0; i < *deviceCount; i++)
    {
        result = nvmlDeviceGetHandleByIndex(i, &device);
        if (result != NVML_SUCCESS)
        {
            fprintf(stderr, "Error al obtener manejador de dispositivo: %s\n", nvmlErrorString(result));
            continue;
        }

        // Llenar cada estructura con datos
        nvmlDeviceGetName(device, infoArray[i].name, sizeof(infoArray[i].name));
        nvmlDeviceGetMinorNumber(device, &(infoArray[i].deviceId));
        nvmlDeviceGetPciInfo(device, &(infoArray[i].pciInfo));
        nvmlDeviceGetMemoryInfo(device, &(infoArray[i].memoryInfo));
        nvmlDeviceGetPowerManagementLimit(device, &(infoArray[i].maxPowerConsumption));
        nvmlDeviceGetMaxClockInfo(device, NVML_CLOCK_GRAPHICS, &(infoArray[i].maxGpuFrequency));
        nvmlDeviceGetMaxClockInfo(device, NVML_CLOCK_MEM, &(infoArray[i].maxMemoryFrequency));
        nvmlDeviceGetUtilizationRates(device, NULL);
    }

    nvmlShutdown();
    return infoArray;
}

DeviceDynamicInfo *fetch_all_dynamic_device_info(unsigned int *deviceCount)
{
    nvmlReturn_t result;
    nvmlDevice_t device;

    *deviceCount = get_device_count();
    if (*deviceCount == 0)
    {
        return NULL;
    }

    DeviceDynamicInfo *infoArray = malloc(sizeof(DeviceDynamicInfo) * (*deviceCount));
    if (!infoArray)
    {
        fprintf(stderr, "Error al asignar memoria para infoArray\n");
        return NULL;
    }

    result = nvmlInit();
    if (result != NVML_SUCCESS)
    {
        fprintf(stderr, "Error al inicializar NVML: %s\n", nvmlErrorString(result));
        free(infoArray);
        return NULL;
    }

    for (unsigned int i = 0; i < *deviceCount; i++)
    {
        result = nvmlDeviceGetHandleByIndex(i, &device);
        if (result != NVML_SUCCESS)
        {
            fprintf(stderr, "Error al obtener manejador de dispositivo: %s\n", nvmlErrorString(result));
            continue;
        }

        // Llenar cada estructura con datos
        nvmlDeviceGetClockInfo(device, NVML_CLOCK_GRAPHICS, &(infoArray[i].currentGpuFrequency));
        nvmlDeviceGetClockInfo(device, NVML_CLOCK_MEM, &(infoArray[i].currentMemoryFrequency));
        nvmlDeviceGetTemperature(device, NVML_TEMPERATURE_GPU, &(infoArray[i].gpuTemperature));
        nvmlDeviceGetFanSpeed(device, &(infoArray[i].fanSpeedPercentage));
        nvmlDeviceGetPowerUsage(device, &(infoArray[i].currentPowerConsumption));
        nvmlUtilization_t utilization;
        nvmlDeviceGetUtilizationRates(device, &utilization);
        infoArray[i].gpuUsagePercentage = utilization.gpu;

        nvmlMemory_t memory;
        nvmlDeviceGetMemoryInfo(device, &memory);
        infoArray[i].usedMemory = memory.used;
        nvmlDeviceGetComputeRunningProcesses(device, &(infoArray[i].processCount), infoArray[i].processes);
    }

    nvmlShutdown();
    return infoArray;
}