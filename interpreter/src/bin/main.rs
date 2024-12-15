use clap::Parser;
use std::{fs::File, io::prelude::*, process::ExitCode};

use interpreter::*;

#[derive(Parser)]
#[command(version)]
struct Cli {
    /// FactoryScript program to execute
    file: Option<String>,

    /// Print benchmarking information after completion
    #[arg(short, long)]
    benchmark: bool,

    /// Increase debug logging level, can be supplied multiple times
    #[arg(short = 'd', long = "debug", action = clap::ArgAction::Count)]
    debug_level: u8,

    /// Disable colored terminal output
    #[arg(long = "no-color")]
    no_color: bool,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    // setting global options
    unsafe {
        DEBUG_LEVEL = cli.debug_level;
        COLOR_OUTPUT = !cli.no_color;
        debug!(1, "Debug level:\t{}", DEBUG_LEVEL);
    }

    // reading file
    let file_name: String = match cli.file {
        Some(s) => s,
        None => {
            print_cli_err!("No file provided");
            return ExitCode::FAILURE;
        }
    };
    let mut file = match File::open(&file_name) {
        Ok(f) => f,
        Err(e) => {
            print_cli_err!("Failed to open file \"{}\": {}", file_name, e);
            return ExitCode::FAILURE;
        }
    };
    debug!(2, "Opened file");
    let mut file_contents = String::new();
    let bytes_read = match file.read_to_string(&mut file_contents) {
        Ok(b) => b,
        Err(e) => {
            print_cli_err!("Failed to read file \"{}\": {}", file_name, e);
            return ExitCode::FAILURE;
        }
    };
    debug!(2, "Read {} bytes", bytes_read);
    debug!(
        1,
        "Input file:\t{} ({})",
        file_name,
        if bytes_read < 1000 {
            format!("{} B", bytes_read)
        } else {
            format!("{:.2} KB", (bytes_read as u32) as f32 / 1000f32)
        }
    );

    match run(&file_contents, cli.benchmark) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            print_cli_err!("{}", e.pretty_msg(&file_contents));
            ExitCode::FAILURE
        }
    }
}
