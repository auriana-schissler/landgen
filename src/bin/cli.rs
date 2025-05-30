use std::mem::size_of;
use clap::Parser;
use landgen::args::Args;
use landgen::file::write_to_file;
use landgen::render::render_map;

fn main() {
    if size_of::<usize>() < size_of::<u64>() {
        eprint!("This program requires 64-bit processing and is not compatible on this processor architecture. ");
        return;
    }
    let args = Args::parse();

    let state = render_map(&args);
    
    let _ = write_to_file(state.clone());
}
