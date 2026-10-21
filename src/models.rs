
/// cpu
#[derive(Debug, Clone, Default)]
pub struct CpuStats {
    pub usage_percentage: f32,
    pub frequency_mhz: u64,
    pub temperature_celsius: Option<f32>,
    pub cores_usage: Vec<f32>, // porcentaje de cada nucleo
}

/// ram y swap
#[derive(Debug, Clone, Default)]
pub struct MemoryStats {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub swap_total_bytes: u64,
    pub swap_used_bytes: u64,
}

impl MemoryStats {
    pub fn used_gb(&self) -> f32 {
        self.used_bytes as f32 / 1024.0 / 1024.0 / 1024.0
    }

    pub fn total_gb(&self) -> f32 {
        self.total_bytes as f32 / 1024.0 / 1024.0 / 1024.0
    }

    pub fn usage_percentage(&self) -> f32 {
        if self.total_bytes == 0 {
            return 0.0;
        }
        (self.used_bytes as f32 / self.total_bytes as f32) * 100.0
    }
}

/// resumen para tarjeta 0
#[derive(Debug, Clone, Default)]
pub struct SummaryStats {
    pub cpu_percentage: f32,
    pub ram_percentage: f32,
    pub battery_percentage: Option<u8>,
    pub cpu_temp: Option<f32>,
}