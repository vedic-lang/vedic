use std::io;
use std::io::Write;
use std::path::Path;
use std::process;

use vedic_core::aadhaar::Aadhaar;
use vedic_core::dosasucana::Dosa;
use vedic_core::fetchsource::Sourcecode;
// use vedic_core::debug;

pub fn help() {
    println!(
        "
    Vedic v{}

    Vedic is a programming language that is designed to be easy to learn and use.

    Developed by: Pt. Prashant Tripathi

    Usage: vedic [--option] [path/to/script.ved]

    Options:
    -h, --help      Show this vedic cli help message
    -v, --version   Print version number
    -r, --repl      Run the vedic repl
    install [name]  Install an indexed library, or restore project dependencies
    format <path>   Format a Vedic source file or every Vedic file in a directory

    Examples:
    ➤ to run a vedic script
      vedic script.ved

    ➤ to start the vedic repl
      vedic -r

    ➤ to format a source file or directory recursively
      vedic format script.ved
      vedic format ./examples
",
        env!("CARGO_PKG_VERSION")
    );
}

pub(crate) fn is_vedic_file(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|extension| extension.to_str()),
        Some("v" | "ved" | "veda" | "vedic")
    )
}

pub fn format(args: &[String]) {
    if args.len() != 1 {
        eprintln!("Usage: vedic format <file-or-directory>");
        process::exit(64);
    }

    match crate::formatter::format_target(Path::new(&args[0])) {
        Ok(paths) => {
            for path in &paths {
                println!("Formatted {}", path.display());
            }
            println!("Formatted {} Vedic file(s).", paths.len());
        }
        Err(error) => {
            eprintln!("Formatting failed: {error}");
            process::exit(1);
        }
    }
}

pub fn version() {
    println!("Vedic v{}", env!("CARGO_PKG_VERSION"));
}

pub fn run_file(path: &str) {
    if path.ends_with(".v")
        || path.ends_with(".ved")
        || path.ends_with(".veda")
        || path.ends_with(".vedic")
    {
        let sc = match Sourcecode::new(path) {
            Ok(content) => content,
            Err(error) => {
                eprintln!("Cannot read '{path}': {error}");
                process::exit(66);
            }
        };

        let mut aadhaar = Aadhaar::new();
        aadhaar.prarambha();
        if let Err(dosa) = aadhaar.vyakhyati(sc) {
            match dosa {
                Dosa::SagkalanaKaleDosa => process::exit(65),
                Dosa::AnusthanaKaleDosa => process::exit(70),
            }
        }
    } else {
        eprintln!("Unsupported file extension for '{path}'. Use .v, .ved, .veda, or .vedic.");
        process::exit(64);
    }
}

pub fn repl() {
    let mut aadhaar = Aadhaar::new();
    println!(
        "Welcome to Vedic v{}
REPL session

❯ Type #निर्गम or #exit to Exit the REPL.
",
        env!("CARGO_PKG_VERSION")
    );
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Unable to read line from the REPL");
        match line.trim() {
            "#निर्गम" | "#exit" => {
                println!("Exiting the REPL...");
                break;
            }
            "" => continue,
            _ => {
                aadhaar
                    .vyakhyati(Sourcecode {
                        code: line,
                        path: "script",
                    })
                    .ok();
            }
        }
    }
}
