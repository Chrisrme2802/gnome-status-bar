use pulse::context::{Context, Flag as ContextFlag};
use pulse::mainloop::standard::Mainloop;
use pulse::volume::VolumeLinear;
use std::cell::RefCell;
use std::rc::Rc;

use crate::models::{AppAudioStream, AudioDevice, AudioStats};

pub struct AudioCollector;

impl AudioCollector {
    pub fn new() -> Self {
        Self
    }

    pub fn fetch(&mut self) -> AudioStats {
        let mut stats = AudioStats::default();

        let mut mainloop = match Mainloop::new() {
            Some(m) => m,
            None => return stats,
        };

        let mut context = match Context::new(&mainloop, "gnome-status-bar") {
            Some(c) => c,
            None => return stats,
        };

        if context.connect(None, ContextFlag::NOFLAGS, None).is_err() {
            return stats;
        }

        loop {
            mainloop.iterate(false);
            match context.get_state() {
                pulse::context::State::Ready => break,
                pulse::context::State::Failed | pulse::context::State::Terminated => return stats,
                _ => {}
            }
        }

        let stats_ref = Rc::new(RefCell::new(AudioStats::default()));
        let introspect = context.introspect();

        let default_sink_name = Rc::new(RefCell::new(String::new()));
        let default_source_name = Rc::new(RefCell::new(String::new()));

        let sink_name_clone = Rc::clone(&default_sink_name);
        let source_name_clone = Rc::clone(&default_source_name);

        introspect.get_server_info(move |info| {
            if let Some(name) = &info.default_sink_name {
                *sink_name_clone.borrow_mut() = name.to_string();
            }
            if let Some(name) = &info.default_source_name {
                *source_name_clone.borrow_mut() = name.to_string();
            }
        });
        mainloop.iterate(true);

        let stats_clone = Rc::clone(&stats_ref);
        let def_sink = default_sink_name.borrow().clone();

        introspect.get_sink_info_list(move |res| {
            if let pulse::callbacks::ListResult::Item(info) = res {
                let name = info.name.as_ref().map(|s| s.to_string()).unwrap_or_default();
                let is_default = name == def_sink;
                let vol_linear = VolumeLinear::from(info.volume.avg()).0;
                let volume = (vol_linear * 100.0).round().min(100.0) as u8;

                let device = AudioDevice {
                    id: info.index,
                    name: name.clone(),
                    description: info.description.as_ref().map(|s| s.to_string()).unwrap_or(name),
                    volume,
                    is_muted: info.mute,
                    is_default,
                };

                let mut s = stats_clone.borrow_mut();
                if is_default {
                    s.output_volume = volume;
                    s.is_output_muted = info.mute;
                }
                s.output_devices.push(device);
            }
        });
        mainloop.iterate(true);

        let stats_clone = Rc::clone(&stats_ref);
        let def_source = default_source_name.borrow().clone();

        introspect.get_source_info_list(move |res| {
            if let pulse::callbacks::ListResult::Item(info) = res {
                let name = info.name.as_ref().map(|s| s.to_string()).unwrap_or_default();
                if name.contains("monitor") {
                    return;
                }

                let is_default = name == def_source;
                let vol_linear = VolumeLinear::from(info.volume.avg()).0;
                let volume = (vol_linear * 100.0).round().min(100.0) as u8;

                let device = AudioDevice {
                    id: info.index,
                    name: name.clone(),
                    description: info.description.as_ref().map(|s| s.to_string()).unwrap_or(name),
                    volume,
                    is_muted: info.mute,
                    is_default,
                };

                let mut s = stats_clone.borrow_mut();
                if is_default {
                    s.input_volume = volume;
                    s.is_input_muted = info.mute;
                }
                s.input_devices.push(device);
            }
        });
        mainloop.iterate(true);

        // --- D. Mezclador por Aplicación (Sink Inputs) ---
        let stats_clone = Rc::clone(&stats_ref);

        introspect.get_sink_input_info_list(move |res| {
            if let pulse::callbacks::ListResult::Item(info) = res {
                let vol_linear = VolumeLinear::from(info.volume.avg()).0;
                let volume = (vol_linear * 100.0).round().min(100.0) as u8;

                // Extraemos el nombre de la app y su icono desde las propiedades de Pulse
                let app_name = info
                    .proplist
                    .get_str("application.name")
                    .unwrap_or_else(|| "Aplicación".to_string());

                let icon_name = info
                    .proplist
                    .get_str("application.icon_name")
                    .unwrap_or_else(|| "audio-card".to_string());

                let stream = AppAudioStream {
                    id: info.index,
                    app_name,
                    icon_name,
                    volume,
                    is_muted: info.mute,
                };

                stats_clone.borrow_mut().app_streams.push(stream);
            }
        });
        mainloop.iterate(true);

        // Desconectamos el contexto de forma limpia
        context.disconnect();

        let final_stats = stats_ref.borrow().clone();
        final_stats
    }

    // --- MÉTODOS DE CONTROL PARA LA UI DE GTK 4 ---

    /// Cambia el volumen de salida principal (0 - 100)
    pub fn set_output_volume(volume_percent: u8) {
        let vol_str = format!("{}%", volume_percent);
        let _ = std::process::Command::new("pactl")
            .args(["set-sink-volume", "@DEFAULT_SINK@", &vol_str])
            .output();
    }

    /// Alterna el silencio (mute) de la salida principal
    pub fn toggle_output_mute() {
        let _ = std::process::Command::new("pactl")
            .args(["set-sink-mute", "@DEFAULT_SINK@", "toggle"])
            .output();
    }

    /// Cambia el volumen de una aplicación específica por su ID/Index
    pub fn set_app_volume(stream_id: u32, volume_percent: u8) {
        let vol_str = format!("{}%", volume_percent);
        let _ = std::process::Command::new("pactl")
            .args(["set-sink-input-volume", &stream_id.to_string(), &vol_str])
            .output();
    }

    /// Cambia el volumen del micrófono principal (0 - 100)
    pub fn set_input_volume(volume_percent: u8) {
        let vol_str = format!("{}%", volume_percent);
        let _ = std::process::Command::new("pactl")
            .args(["set-source-volume", "@DEFAULT_SOURCE@", &vol_str])
            .output();
    }
}