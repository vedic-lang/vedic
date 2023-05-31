use std::env;
mod utilities;

/// Discription : Command Line Tool Functionlity.
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "-h" | "--help" => utilities::help(),
            "-v" | "--version" => utilities::version(),
            // "-f" | "--format" => utilities::format(args),
            // "-i" | "--install" => utilities::install(args),
            "-r" | "--repl" => utilities::repl(),
            _ => utilities::run(args),
        }
    } else {
        utilities::help();
    }
}
