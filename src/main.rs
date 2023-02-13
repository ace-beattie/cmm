mod cli;
mod filesys;
mod makefile;

use filesys::get_source;
use makefile::create_makefile;
use std::{env, path::PathBuf};

use cli::get_args;

fn main() {
    let args = get_args();

    let curr_dir = env::current_dir().unwrap();

    let dir = match args.directory {
        Some(dir) => PathBuf::from(dir),
        None => env::current_dir().unwrap(),
    };

    let build_dir = PathBuf::from(&args.build_dir);

    let source_files: Vec<PathBuf> = get_source(args.cplusplus, &dir);

    match create_makefile(
        &source_files,
        &build_dir,
        &curr_dir,
        &args.outfile,
        args.current_dir,
    ) {
        Ok(_) => println!("Makefile created successfully."),
        Err(e) => panic!("Error creating makefile: {}", e),
    }
}
