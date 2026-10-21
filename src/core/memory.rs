use sysinfo::{MemoryRefreshKind, RefreshKind, System};
use crate::models::MemoryStats;

pub struct MemoryCollector {
    sys: System,
}

impl MemoryCollector {
    pub fn new() -> Self {
        // sysinfo ram swap
        let sys = System::new_with_specifics(
            RefreshKind::new().with_memory(MemoryRefreshKind::everything()),
        );
        Self { sys }
    }

    pub fn fetch(&mut self) -> MemoryStats {
        self.sys.refresh_memory();

        let total_bytes = self.sys.total_memory();
        let used_bytes = self.sys.used_memory();
        let swap_total_bytes = self.sys.total_swap();
        let swap_used_bytes = self.sys.used_swap();

        MemoryStats {
            total_bytes,
            used_bytes,
            swap_total_bytes,
            swap_used_bytes,
        }
    }
}