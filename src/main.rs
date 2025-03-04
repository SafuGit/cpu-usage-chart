use charming::{
    component::{Axis, Title},
    datatype::DataPoint,
    renderer::ImageRenderer,
    series::Line,
    Chart,
};
use resvg::{tiny_skia::Pixmap, usvg};
use std::{fs, process::Command, thread, time::Duration};
use sysinfo::System;
use usvg::{Options, Transform, Tree};

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    println!("Hello, world!");
    memory(&mut sys);
    render_cpu_chart(&mut sys);
}

fn render_cpu_chart(mut sys: &mut System) {
    let cpu_data = cpu(&mut sys);

    let chart = Chart::new()
        .title(Title::new().text("CPU USAGE"))
        .x_axis(Axis::new().name("Time (s)"))
        .y_axis(Axis::new().name("CPU Usage (%)"))
        .series(Line::new().name("CPU Usage").data(cpu_data));

    let svg_path = "cpu_usage.svg";
    let png_path = "cpu_usage.png";

    let mut renderer = ImageRenderer::new(1000, 1000);
    if let Err(e) = renderer.save(&chart, svg_path) {
        eprintln!("Error saving chart: {}", e);
        return;
    }

    println!("Chart saved successfully as {}", svg_path);

    let svg_data = fs::read_to_string("./cpu_usage.svg").expect("Failed to read SVG file");
    if let Some(pixmap) = svg_to_png(&svg_data) {
        pixmap.save_png(png_path).expect("Failed to save PNG");
    }

    open_image(png_path);
}

fn memory(sys: &mut System) {
    // let total_memory_gb = sys.total_memory() as f64 / 1_000_000_000.0;
    let total_used_memory_gb = sys.used_memory() as f64 / 1_000_000_000.0;
    println!("{:.3}", total_used_memory_gb);
}

fn cpu(sys: &mut System) -> Vec<DataPoint> {
    let mut cpu_data: Vec<DataPoint> = vec![];
    for n in 1..=15 {
        sys.refresh_cpu_all();
        thread::sleep(Duration::from_millis(500));
        let total_cpu_usage = sys.global_cpu_usage();
        cpu_data.push(DataPoint::from(vec![n as f32, total_cpu_usage]));
        println!("Total CPU usage: {:.2}%", total_cpu_usage);
    }
    // println!("cpu data vector {:?}", cpu_data);
    cpu_data
}

fn svg_to_png(svg_data: &str) -> Option<Pixmap> {
    // println!("svg data: {}", svg_data);
    let opt = Options::default();
    let rtree = Tree::from_str(svg_data, &opt).ok()?;

    let pixmap_size = rtree.size();
    let mut pixmap = Pixmap::new(pixmap_size.width() as u32, pixmap_size.height() as u32)?;

    pixmap.fill(resvg::tiny_skia::Color::WHITE);

    resvg::render(&rtree, Transform::identity(), &mut pixmap.as_mut());

    Some(pixmap)
}

fn open_image(img_path: &str) {
    #[cfg(target_os = "windows")]
    Command::new("cmd")
        .arg("/C")
        .arg("start")
        .arg(img_path)
        .spawn()
        .unwrap();

    #[cfg(target_os = "linux")]
    Command::new("xdg-open").arg(img_path).spawn().unwrap();

    #[cfg(target_os = "macos")]
    Command::new("open").arg(img_path).spawn().unwrap();
}
