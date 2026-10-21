use sysinfo::Networks;
use crate::models::NetworkStats;

pub struct NetworkCollector {
    networks: Networks,
}

impl NetworkCollector {
    pub fn new() -> Self {
        let networks = Networks::new_with_refreshed_list();
        Self { networks }
    }

    pub fn fetch(&mut self) -> NetworkStats {
        // Refresca las métricas de tráfico en las interfaces
        self.networks.refresh();

        let mut primary_stats = NetworkStats::default();

        // Buscamos la interfaz principal activa (Wi-Fi o Ethernet)
        // Ignoramos la interfaz de loopback 'lo'
        for (interface_name, network) in &self.networks {
            if interface_name == "lo" {
                continue;
            }

            // Si encuentra una interfaz activa transmitiendo/recibiendo datos
            let rx = network.received();
            let tx = network.transmitted();

            if rx > 0 || tx > 0 || primary_stats.interface_name.is_empty() {
                primary_stats = NetworkStats {
                    interface_name: interface_name.clone(),
                    rx_bytes_per_sec: rx,
                    tx_bytes_per_sec: tx,
                    total_received_bytes: network.total_received(),
                    total_transmitted_bytes: network.total_transmitted(),
                };
            }
        }

        primary_stats
    }
}