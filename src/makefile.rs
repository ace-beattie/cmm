use crate::filesys::create_relative_path;

use std::io;
use std::io::prelude::*;
use std::{fs::File, path::PathBuf};

pub fn create_makefile(
    source_paths: &Vec<PathBuf>,
    out_dir: &PathBuf,
    cur_dir: &PathBuf,
    outfile_name: &str,
    place_out_in_root: bool,
    use_cplusplus: bool,
) -> io::Result<()> {
    let fd = File::create("makefile").unwrap();

    let mut buffer = std::io::BufWriter::new(fd);

    let source_paths_rel: Vec<PathBuf> = source_paths
        .iter()
        .map(|source_path| create_relative_path(source_path, &cur_dir))
        .collect();

    let out_dir_rel = create_relative_path(&out_dir, &cur_dir);
    let out_dir_rel_string = out_dir_rel.to_string_lossy();

    write!(buffer, "# This file was generated by CMM\n\n")?;

    write!(buffer, "debug: CFLAGS += -g\ndebug: all\n\n")?;

    let obj_paths: Vec<String> = source_paths_rel
        .iter()
        .map(|source_path| {
            let obj_path: PathBuf = [
                out_dir_rel.as_os_str(),
                source_path.with_extension("o").file_name().unwrap(),
            ]
            .iter()
            .collect();

            let obj_path_string = obj_path.to_str().unwrap().to_string();

            obj_path_string
        })
        .collect();

    let all_objs = obj_paths.join(" ");

    let outfile_path;

    outfile_path = if place_out_in_root {
        outfile_name.to_string()
    } else {
        let outfile_path = out_dir_rel.join(outfile_name);
        outfile_path.to_string_lossy().to_string()
    };

    write!(
        buffer,
        "all: {} {}\n\t{} $(CFLAGS) {} -o {}\n\n",
        out_dir_rel_string,
        all_objs,
        if use_cplusplus { "g++" } else { "gcc" },
        all_objs,
        outfile_path
    )?;

    for (i, object_file) in obj_paths.iter().enumerate() {
        let source_path_relative_string = source_paths_rel[i].to_string_lossy();

        write!(
            buffer,
            "{}: {} {}\n\t {} $(CFLAGS) -c {} -o {}\n\n",
            object_file,
            out_dir_rel_string,
            source_path_relative_string,
            if source_path_relative_string.ends_with(".cpp") {
                "g++"
            } else {
                "gcc"
            },
            source_path_relative_string,
            object_file
        )?;
    }

    write!(
        buffer,
        "{}:\n\tmkdir {}\n\n",
        out_dir_rel_string, out_dir_rel_string
    )?;

    write!(buffer, "clean:\n\trm -rf {}\n\n", out_dir_rel_string)?;

    write!(buffer, "run: all\n\t{}\n\n", outfile_path)?;

    write!(buffer, ".PHONY: all clean run")?;

    buffer.flush()?;

    Ok(())
}
