use std::env;
mod formatter;
mod install;
mod utilities;

/// Discription : Command Line Tool Functionlity.
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "-h" | "--help" => utilities::help(),
            "-v" | "--version" => utilities::version(),
            "-f" | "format" => utilities::format(&args[2..]),
            "install" => install::run(&args[2..]),
            "-r" | "--repl" => utilities::repl(),
            argument if argument.starts_with('-') => {
                eprintln!("Unknown option '{argument}'. Run `vedic --help` for usage.");
                std::process::exit(64);
            }
            path => utilities::run_file(path),
        }
    } else {
        utilities::help();
    }
}
