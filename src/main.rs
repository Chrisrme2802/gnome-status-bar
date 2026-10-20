use libadwaita::prelude::*;
use libadwaita::{Application, ApplicationWindow, HeaderBar};
use gtk4::{Box, Label, Orientation};
use sysinfo::System;

fn main() {
    // inicializar la aplicación de Libadwaita
    let app = Application::builder()
        .application_id("dev.gael.HealthBar")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    // 1. Obtener métricas del sistema
    let mut sys = System::new_all();
    sys.refresh_cpu();
    sys.refresh_memory();

    let cpu_usage = sys.global_cpu_info().cpu_usage();
    let total_mem = sys.total_memory() as f32 / 1024.0 / 1024.0; // En GB
    let used_mem = sys.used_memory() as f32 / 1024.0 / 1024.0;   // En GB

    // 2. Crear los widgets de GTK 4
    let header = HeaderBar::new();

    let label_cpu = Label::builder()
        .label(&format!("CPU: {:.1}%", cpu_usage))
        .css_classes(["title-2"])
        .build();

    let label_mem = Label::builder()
        .label(&format!("RAM: {:.2} GB / {:.2} GB", used_mem, total_mem))
        .css_classes(["body"])
        .build();

    let content_box = Box::new(Orientation::Vertical, 12);
    content_box.set_margin_top(16);
    content_box.set_margin_bottom(16);
    content_box.set_margin_start(16);
    content_box.set_margin_end(16);

    content_box.append(&label_cpu);
    content_box.append(&label_mem);

    let main_box = Box::new(Orientation::Vertical, 0);
    main_box.append(&header);
    main_box.append(&content_box);

    // 3. Crear la ventana nativa
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Salud del Sistema")
        .default_width(320)
        .default_height(180)
        .content(&main_box)
        .build();

    window.present();
}