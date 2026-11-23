use std::env;
mod formatter;
mod utilities;

/// Discription : Command Line Tool Functionlity.
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "-h" | "--help" => utilities::help(),
            "-v" | "--version" => utilities::version(),
            "-f" | "format" => utilities::format(&args[2..]),
            // "-i" | "install" => utilities::install(args),
            "-r" | "--repl" => utilities::repl(),
            _ => utilities::run(args),
        }
    } else {
        utilities::help();
    }
}
