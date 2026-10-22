use std::fs;
use crate::models::GpuStats;

pub struct GpuCollector;

impl GpuCollector {
    pub fn new() -> Self {
        Self
    }

    pub fn fetch(&mut self) -> GpuStats {
        let name = Self::read_gpu_name().unwrap_or_else(|| "GPU Integrada / Genérica".to_string());
        let usage_percentage = Self::read_gpu_usage();
        let temperature_celsius = Self::read_gpu_temp();

        GpuStats {
            name,
            usage_percentage,
            memory_used_mb: None,
            memory_total_mb: None,
            temperature_celsius,
        }
    }

    fn read_gpu_name() -> Option<String> {
        let path = "/sys/class/drm/card0/device/product_name";
        fs::read_to_string(path).ok().map(|s| s.trim().to_string())
    }

    fn read_gpu_usage() -> Option<f32> {
        let paths = [
            "/sys/class/drm/card0/device/gpu_busy_percent", // AMD
            "/sys/class/drm/card0/load",                    // Intel
        ];

        for path in paths {
            if let Ok(content) = fs::read_to_string(path) {
                if let Ok(val) = content.trim().parse::<f32>() {
                    return Some(val);
                }
            }
        }
        None
    }

    fn read_gpu_temp() -> Option<f32> {
        let path = "/sys/class/hwmon/hwmon0/temp1_input";
        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(milli_c) = content.trim().parse::<f32>() {
                return Some(milli_c / 1000.0);
            }
        }
        None
    }
}