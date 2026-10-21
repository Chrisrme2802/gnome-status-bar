use sysinfo::{CpuRefreshKind, RefreshKind, System};
use crate::models::CpuStats;

pub struct CpuCollector {
    sys: System,
}

impl CpuCollector {
    pub fn new() -> Self {
        // sysinfo cpu
        let sys = System::new_with_specifics(
            RefreshKind::new().with_cpu(CpuRefreshKind::everything()),
        );
        Self { sys }
    }

    pub fn fetch(&mut self) -> CpuStats {
        // sysinfo necesario refrescar dos veces o dejar pasar un intervalo para calcular el delta real de uso de CPU
        self.sys.refresh_cpu_all();

        let usage_percentage = self.sys.global_cpu_info().cpu_usage();
        
        let cores_usage: Vec<f32> = self
            .sys
            .cpus()
            .iter()
            .map(|cpu| cpu.cpu_usage())
            .collect();

        let frequency_mhz = self
            .sys
            .cpus()
            .first()
            .map(|cpu| cpu.frequency())
            .unwrap_or(0);

        CpuStats {
            usage_percentage,
            frequency_mhz,
            temperature_celsius: None,
            cores_usage,
        }
    }
}