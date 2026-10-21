use sysinfo::System;
use std::env;
use crate::models::SystemInfoStats;

pub struct SystemInfoCollector;

impl SystemInfoCollector {
    pub fn new() -> Self {
        Self
    }

    pub fn fetch(&mut self) -> SystemInfoStats {
        let os_name = System::name().unwrap_or_else(|| "Linux".to_string());
        let kernel_version = System::kernel_version().unwrap_or_else(|| "Desconocido".to_string());
        let host_name = System::host_name().unwrap_or_else(|| "archlinux".to_string());
        let user_name = env::var("USER").unwrap_or_else(|_| "Usuario".to_string());
        let uptime_seconds = System::uptime();

        SystemInfoStats {
            os_name,
            kernel_version,
            host_name,
            user_name,
            uptime_seconds,
        }
    }
}