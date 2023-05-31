use std::io;
use std::io::Write;
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
    
    Examples: 
    ➤ to run a vedic script
      vedic script.ved
    
    ➤ to start the vedic repl
      vedic -r
",
        env!("CARGO_PKG_VERSION")
    );
}

// ❯ this command will install the panchang.ved vedic script from given url
// vedic -i https://git.com/panchang.ved

// pub fn format(_args: Vec<String>) {
//     println!("Format Feature is not available yet.");
//     process::exit(0);
// }

// pub fn install(_args: Vec<String>) {
//     println!("Install Feature is not available yet.");
//     process::exit(0);
// }

pub fn version() {
    println!("Vedic v{}", env!("CARGO_PKG_VERSION"));
}

pub fn run(args: Vec<String>) {
    if args[1].starts_with('-') {
        eprintln!("Invalid argument.");
    } else {
        run_file(&args[1]);
    }
}

pub fn run_file(path: &str) {
    if path.ends_with(".ved") {
        let mut aadhaar = Aadhaar::new();
        aadhaar.prarambha();
        let sc = match Sourcecode::new(path) {
            Ok(content) => content,
            Err(dosa) => {
                eprintln!("Unable to read file : {dosa}");
                process::exit(0);
            }
        };

        if let Err(dosa) = aadhaar.vyakhyati(sc) {
            match dosa {
                Dosa::SagkalanaKaleDosa => process::exit(65),
                Dosa::AnusthanaKaleDosa => process::exit(70),
            }
        }
    } else {
        eprintln!("Invalid file extension. Only .ved files are allowed.");
        process::exit(0);
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
