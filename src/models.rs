
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

/// power
#[derive(Debug, Clone, Default)]
pub struct PowerStats {
    pub capacity_percentage: u8,
    pub status: String,
    pub energy_full: u64,
    pub energy_full_design: u64,
}

/// storage
#[derive(Debug, Clone, Default)]    
pub struct DiskInfo {
    pub name: String,
    pub mount_point: String,
    pub file_system: String,
    pub total_bytes: u64,
    pub available_bytes: u64,
}

/// sistema
#[derive(Debug, Clone, Default)]
pub struct SystemInfoStats {
    pub os_name: String,
    pub kernel_version: String,
    pub host_name: String,
    pub user_name: String,
    pub uptime_seconds: u64,
}

/// network
#[derive(Debug, Clone, Default)]
pub struct NetworkStats {
    pub interface_name: String,
    pub rx_bytes_per_sec: u64,
    pub tx_bytes_per_sec: u64
    pub total_received_bytes: u64,
    pub total_transmitted_bytes: u64,
}

/// memory methods
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

/// power methods
impl PowerStats {
    pub fn health_percentage(&self) -> f32 {
        if self.energy_full_design == 0 {
            return 100.0;
        }
        (self.energy_full as f32 / self.energy_full_design as f32) * 100.0
    }
}

/// storage methods
impl DiskInfo {
    pub fn used_bytes(&self) -> u64 {
        self.total_bytes.saturating_sub(self.available_bytes)
    }

    pub fn used_gb(&self) -> f32 {
        self.used_bytes() as f32 / 1024.0 / 1024.0 / 1024.0
    }

    pub fn total_gb(&self) -> f32 {
        self.total_bytes as f32 / 1024.0 / 1024.0 / 1024.0
    }

    pub fn usage_percentage(&self) -> f32 {
        if self.total_bytes == 0 {
            return 0.0;
        }
        (self.used_bytes() as f32 / self.total_bytes as f32) * 100.0
    }
}

/// system methods
impl SystemInfoStats {
    pub fn formatted_uptime(&self) -> String {
        let days = self.uptime_seconds / 86400;
        let hours = (self.uptime_seconds % 86400) / 3600;
        let minutes = (self.uptime_seconds % 3600) / 60;

        if days > 0 {
            format!("{}d {}h {}m", days, hours, minutes)
        } else if hours > 0 {
            format!("{}h {}m", hours, minutes)
        } else {
            format!("{}m", minutes)
        }
    }
}

impl NetworkStats {
    pub fn format_rx(&self) -> String {
        Self::format_speed(self.rx_bytes_per_sec)
    }

    pub fn format_tx(&self) -> String {
        Self::format_speed(self.tx_bytes_per_sec)
    }

    fn format_speed(bytes_per_sec: u64) -> String {
        let kb = bytes_per_sec as f32 / 1024.0;
        let mb = kb / 1024.0;

        if mb >= 1.0 {
            format!("{:.2} MB/s", mb)
        } else {
            format!("{:.1} KB/s", kb)
        }
    }
}

///  discos
#[derive(Debug, Clone, Default)]
pub struct StorageStats {
    pub disks: Vec<DiskInfo>,
}

/// resumen para tarjeta 0
#[derive(Debug, Clone, Default)]
pub struct SummaryStats {
    pub cpu_percentage: f32,
    pub ram_percentage: f32,
    pub battery_percentage: Option<u8>,
    pub cpu_temp: Option<f32>,
}