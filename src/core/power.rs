use std::fs;
use crate::models::PowerStats;

pub struct PowerCollector;

impl PowerCollector {
    pub fn new() -> Self {
        Self
    }

    pub fn fetch(&mut self) -> PowerStats {
        let capacity_percentage = Self::read_capacity().unwrap_or(0);
        let status = Self::read_status().unwrap_or_else(|| "Unknown".to_string());
        let energy_full = Self::read_u64_file("/sys/class/power_supply/BAT0/energy_full")
            .or_else(|| Self::read_u64_file("/sys/class/power_supply/BAT0/charge_full"))
            .unwrap_or(0);

        let energy_full_design = Self::read_u64_file("/sys/class/power_supply/BAT0/energy_full_design")
            .or_else(|| Self::read_u64_file("/sys/class/power_supply/BAT0/charge_full_design"))
            .unwrap_or(0);

        PowerStats {
            capacity_percentage,
            status,
            energy_full,
            energy_full_design,
        }
    }

    fn read_capacity() -> Option<u8> {
        let content = fs::read_to_string("/sys/class/power_supply/BAT0/capacity").ok()?;
        content.trim().parse::<u8>().ok()
    }

    fn read_status() -> Option<String> {
        let content = fs::read_to_string("/sys/class/power_supply/BAT0/status").ok()?;
        Some(content.trim().to_string())
    }

    fn read_u64_file(path: &str) -> Option<u64> {
        let content = fs::read_to_string(path).ok()?;
        content.trim().parse::<u64>().ok()
    }
}