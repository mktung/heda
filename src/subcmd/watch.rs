use std::path::Path;

pub mod derive;
pub mod plot;
pub mod sort;

pub fn run_watch(_config_path: &Path) {
    println!("todo watch");
    derive::run_derive();
    sort::run_sort();
    plot::run_plot();
}
