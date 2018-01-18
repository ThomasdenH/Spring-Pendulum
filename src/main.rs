extern crate gnuplot;
extern crate rayon;
extern crate rand;

mod system;

use system::System;
use gnuplot::{Figure, PlotOption, AxesCommon};
use rayon::prelude::*;
use std::process::Command;

fn main() {
    let c = 1.0 / 3.0;
    let d = 0.001;
    let h_min = -7.0 / 4.0;
    let h = h_min + 2.0;

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
