extern crate gnuplot;
extern crate rayon;
extern crate rand;

mod system;

use system::System;
use gnuplot::{Figure, PlotOption, AxesCommon, AutoOption};
use rayon::prelude::*;
use std::process::Command;

fn main() {
    let c = 1.0 / 3.0;
    let d = 0.01;
    let h_min = -7.0 / 4.0;
    let h = h_min + 0.1;

    make_poincare_map(h, c, d);
    make_lyapunov_map(h, c, d);
}

/// Create a Lyapunov exponent heat map
fn make_lyapunov_map(h: f64, c: f64, d: f64) {

    let y_start = -5.0;
    let y_end = 0.0;
    let py_start = -1.5;
    let py_end = 1.5;

    let size_y = 100;
    let size_py = 100;

    let matrix = (0..size_y * size_py).into_par_iter().map(|i| {
        let y_coord = i / size_py;
        let py_coord = i % size_py;
        let y = y_start + (y_coord as f64 / size_y as f64) * (y_end - y_start);
        let py = py_start + (py_coord as f64 / size_py as f64) * (py_end - py_start);

        if let Some(mut sys) = System::new(0.0, y, py, h, c, d) {
            sys.get_exponent()
        } else {
            0.0
        }
    }).collect::<Vec<f64>>();

    let filename = format!("output/heatmap_H_{}", h);
    let filename_png = format!("output/heatmap_H_{}.png", h);
    let mut fig = Figure::new();
    fig.axes2d()
        .set_title(&format!("Heatmap of \\lambda (\\bar{{H}} = {})", h), &[])
        .set_x_label(&"py", &[])
        .set_y_label(&"y", &[])
        .set_x_range(AutoOption::Fix(py_start), AutoOption::Fix(py_end))
        .set_y_range(AutoOption::Fix(y_start), AutoOption::Fix(y_end))
        .set_cb_range(AutoOption::Fix(-0.002), AutoOption::Fix(0.05))
        .image(&matrix, size_y, size_py,
               Some((py_start, y_start, py_end, y_end)), &[]);
    fig.echo_to_file(&filename);

    let command = format!("set terminal png size 1000,1000;\
                     set output '{}';\
                     load '{}';\
                     gnuplot exit;", &filename_png, &filename);
    Command::new("gnuplot")
        .args(&["-e", &command])
        .output()
        .expect("Could not save file");

    println!("{}", filename);
}

/// Create a Poincaré map with random orbits.
fn make_poincare_map(h: f64, c: f64, d: f64) {
    let points = (0..50).into_par_iter()
        .flat_map(|_| System::random(h, c, d)
            .filter(|s| s.has_crossed(0.0))
            .take(5_000)
            .map(|s| (s.py(), s.y()))
            .collect::<Vec<(f64, f64)>>())
        .collect::<Vec<(f64, f64)>>();

    let py = points.par_iter().map(|f| f.0).collect::<Vec<f64>>();
    let y = points.par_iter().map(|f| f.1).collect::<Vec<f64>>();

    let filename = format!("output/plot_H_{}", h);
    let filename_png = format!("output/plot_H_{}.png", h);
    let mut fig = Figure::new();
    fig.axes2d()
        .set_title(&format!("Poincaré map of py vs. y (\\bar{{H}} = {})", h), &[])
        .set_x_label(&"py", &[])
        .set_y_label(&"y", &[])
        .points(&py, &y, &[PlotOption::PointSymbol('.')]);
    fig.echo_to_file(&filename);

    let command = format!("set terminal png size 1000,1000;\
                     set output '{}';\
                     load '{}';\
                     gnuplot exit;", &filename_png, &filename);
    Command::new("gnuplot")
        .args(&["-e", &command])
        .output()
        .expect("Could not save file");

    println!("Saved file as {}", filename);
}
