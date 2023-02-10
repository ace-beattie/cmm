use glob::glob;
use std::{env, path::PathBuf};

/// Returns a relative path from the given directory to the given path.
///
/// # Arguments
///
/// * `abs_path` - The absolute path to the file.
/// * `rel_dir` - The directory to create the relative path from.
///
/// # Returns
/// A relative path from the given directory to the given path.
pub fn create_relative_path(abs_path: &PathBuf, rel_dir: &PathBuf) -> PathBuf {
    let mut new_path = PathBuf::from("./");

    if abs_path.is_relative() {
        return abs_path.clone();
    }

    let mut curr_directory = rel_dir.canonicalize().unwrap().clone();

    println!(
        "Converting {} into relative with current directory: {}",
        abs_path.to_string_lossy(),
        curr_directory.to_string_lossy()
    );

    while !abs_path
        .iter()
        .any(|x| x == curr_directory.file_name().unwrap_or("".as_ref()))
    {
        curr_directory.pop();
        new_path.push("..");
    }

    new_path.push(abs_path.strip_prefix(&curr_directory).unwrap());

    new_path
}

/// Returns an absolute path for the given path.
///
/// # Arguments
///
/// * `path` - The path to make absolute.
///
/// # Returns
/// An absolute path to the given path.
pub fn _create_absolute_path(path: &PathBuf) -> PathBuf {
    if !path.is_relative() {
        return path.clone();
    }

    let mut new_path = env::current_dir().unwrap();

    for x in path.components() {
        match x {
            std::path::Component::Normal(x) => new_path.push(x),
            std::path::Component::ParentDir => drop(new_path.pop()),
            _ => (),
        }
    }

    new_path
}
pub fn get_source(cplusplus: bool, dir: &PathBuf) -> Vec<PathBuf> {
    let mut source_paths: Vec<PathBuf> = Vec::new();

    let mut extensions = vec!["c"];

    if cplusplus {
        extensions.push("cpp");
    }

    for extension in extensions.iter() {
        let mut path = dir.canonicalize().unwrap();
        path.push("**");
        path.push(format!("*.{}", extension));

        let mut files = glob(path.to_str().unwrap()).unwrap();

        while let Some(file) = files.next() {
            let file = file.unwrap().canonicalize().unwrap();

            source_paths.push(file);
        }
    }

    source_paths
}
