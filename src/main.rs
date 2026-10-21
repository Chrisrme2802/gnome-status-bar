pub mod models;
pub mod core {
    pub mod cpu;
    pub mod memory;
    pub mod power;
    pub mod storage;
}

use crate::core::storage::StorageCollector;

fn main() {
    let mut storage = StorageCollector::new();
    let stats = storage.fetch();

    println!("--- Almacenamiento y Discos ---");
    for disk in stats.disks {
        println!(
            "Punto de montaje: {} ({})",
            disk.mount_point, disk.file_system
        );
        println!(
            "  Uso: {:.2} GB / {:.2} GB ({:.1}%)",
            disk.used_gb(),
            disk.total_gb(),
            disk.usage_percentage()
        );
        println!("----------------------------------");
    }
}