use sysinfo::Disks;
use crate::models::{DiskInfo, StorageStats};

pub struct StorageCollector {
    disks: Disks,
}

impl StorageCollector {
    pub fn new() -> Self {
        let disks = Disks::new_with_refreshed_list();
        Self { disks }
    }

    pub fn fetch(&mut self) -> StorageStats {
    self.disks.refresh();

    let disk_list = self
        .disks
        .iter()
        .filter(|disk| {
            let mount = disk.mount_point().to_string_lossy();
            let fs = disk.file_system().to_string_lossy();
        })
        .map(|disk| DiskInfo {
            name: disk.name().to_string_lossy().to_string(),
            mount_point: disk.mount_point().to_string_lossy().to_string(),
            file_system: disk.file_system().to_string_lossy().to_string(),
            total_bytes: disk.total_space(),
            available_bytes: disk.available_space(),
        })
        .collect();

    StorageStats { disks: disk_list }
}
}