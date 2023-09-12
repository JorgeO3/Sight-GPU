#include <stdio.h>
#include <nvml.h>
#include "your_header_file.h" // Asume que tienes un archivo de encabezado con las estructuras y declaraciones.

unsigned int get_device_count() {
    unsigned int count = 0;
    nvmlReturn_t result;

    result = nvmlInit();
    if (result != NVML_SUCCESS) {
        fprintf(stderr, "Error al inicializar NVML: %s\n", nvmlErrorString(result));
        return count; // Retorna 0 en caso de error.
    }

    nvmlDeviceGetCount(&count);

    nvmlShutdown();

    return count;
}

StaticDeviceInfo* fetch_all_static_device_info(unsigned int *deviceCount) {
    nvmlReturn_t result;
    nvmlDevice_t device;

    *deviceCount = get_device_count();

    if (*deviceCount == 0) {
        return NULL;
    }

    StaticDeviceInfo *infoArray = malloc(sizeof(StaticDeviceInfo) * (*deviceCount));
    if (!infoArray) {
        fprintf(stderr, "Error al asignar memoria para infoArray\n");
        return NULL;
    }

    result = nvmlInit();
    if (result != NVML_SUCCESS) {
        fprintf(stderr, "Error al inicializar NVML: %s\n", nvmlErrorString(result));
        free(infoArray);
        return NULL;
    }

    for (unsigned int i = 0; i < *deviceCount; i++) {
        result = nvmlDeviceGetHandleByIndex(i, &device);
        if (result != NVML_SUCCESS) {
            fprintf(stderr, "Error al obtener manejador de dispositivo: %s\n", nvmlErrorString(result));
            continue;
        }

        // Llena cada estructura con datos
        nvmlDeviceGetName(device, infoArray[i].name, sizeof(infoArray[i].name));
        nvmlDeviceGetMinorNumber(device, &(infoArray[i].deviceId));
        nvmlDeviceGetMaxPcieGeneration(device, &(infoArray[i].pcieGeneration));
        nvmlDeviceGetMemoryInfo(device, &(infoArray[i].memoryInfo));
        nvmlDeviceGetPowerManagementLimit(device, &(infoArray[i].maxPowerUsage));
        nvmlDeviceGetMaxClockInfo(device, NVML_CLOCK_GRAPHICS, &(infoArray[i].maxGpuFrequency));
        nvmlDeviceGetMaxClockInfo(device, NVML_CLOCK_MEM, &(infoArray[i].maxMemoryFrequency));
    }

    nvmlShutdown();

    return infoArray;
}

DynamicDeviceInfo* fetch_all_dynamic_device_info(unsigned int *deviceCount) {
    nvmlReturn_t result;
    nvmlDevice_t device;

    *deviceCount = get_device_count();

    if (*deviceCount == 0) {
        return NULL;
    }

    DynamicDeviceInfo *infoArray = malloc(sizeof(DynamicDeviceInfo) * (*deviceCount));
    if (!infoArray) {
        fprintf(stderr, "Error al asignar memoria para infoArray\n");
        return NULL;
    }

    result = nvmlInit();
    if (result != NVML_SUCCESS) {
        fprintf(stderr, "Error al inicializar NVML: %s\n", nvmlErrorString(result));
        free(infoArray);
        return NULL;
    }

    for (unsigned int i = 0; i < *deviceCount; i++) {
        result = nvmlDeviceGetHandleByIndex(i, &device);
        if (result != NVML_SUCCESS) {
            fprintf(stderr, "Error al obtener manejador de dispositivo: %s\n", nvmlErrorString(result));
            continue;
        }

        // Llena cada estructura con datos
        nvmlDeviceGetClockInfo(device, NVML_CLOCK_GRAPHICS, &(infoArray[i].currentGpuFrequency));
        nvmlDeviceGetClockInfo(device, NVML_CLOCK_MEM, &(infoArray[i].currentMemoryFrequency));
        nvmlDeviceGetTemperature(device, NVML_TEMPERATURE_GPU, &(infoArray[i].gpuTemperature));
        nvmlDeviceGetFanSpeed(device, &(infoArray[i].fanUsage));
        nvmlDeviceGetPowerUsage(device, &(infoArray[i].currentPowerUsage));
        nvmlDeviceGetUtilizationRates(device, &(infoArray[i].utilization));
        nvmlDeviceGetMemoryInfo(device, &(infoArray[i].memoryInfo));
    }

    nvmlShutdown();

    return infoArray;
}

void free_static_info_array(StaticDeviceInfo *infoArray) {
    if (infoArray) {
        free(infoArray);
    }
}

void free_dynamic_info_array(DynamicDeviceInfo *infoArray) {
    if (infoArray) {
        free(infoArray);
    }
}
