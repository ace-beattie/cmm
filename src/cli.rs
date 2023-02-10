use clap::Parser;

/// A simple CLI for generating a makefile from a C or C++ project.
#[derive(Parser, Debug)]
#[command(
    author = "Ace Beattie",
    version = "0.1.0",
    about = "A simple CLI for generating a makefile from a C or C++ project."
)]
pub struct Args {
    /// Output file name
    #[arg(short, long, default_value = "a.out")]
    pub outfile: String,

    /// The directory to search for C/C++ files
    #[arg()]
    pub directory: Option<String>,

    /// Build directory
    #[arg(short, long, default_value = "./out/")]
    pub build_dir: String,

    /// Use C++ compiler.
    #[arg(short = '+', long = "c++")]
    pub cplusplus: bool,

    /// Place outfile in current directory
    #[arg(short = 'r', long = "root")]
    pub current_dir: bool,
}

pub fn get_args() -> Args {
    Args::parse()
}
