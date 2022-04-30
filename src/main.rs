use std::io;
use std::env;
use std::fs;
use analysis::Analysis;
use analysis::report::Report;

mod analysis;

fn main() {
    let args: Vec<String> = env::args().collect();
    let report = match args.get(1) {
        Some(filename) => process_file(filename),
        None => process_stdin(),
    };

    eprintln!("=================================");
    println!("{:#?}",report);
    eprintln!("=================================");
}

fn process_stdin() -> Report {
    eprintln!("Reading from STDIN");
    let mut analysis = Analysis::new();
    
    for line in io::stdin().lines() {
        match line {
            Ok(ref l) => analysis.process_log(&l),
            Err(error) => eprintln!("error: {}", error),
        }
    }

    analysis.report
}

fn process_file(filename: &str) -> Report {
    eprintln!("Reading from file: {:?}", &filename);
    let contents = fs::read_to_string(filename)
                        .expect("There was an error reading the file");
    let mut analysis = Analysis::new();
    analysis.process_log(&contents.to_string());
    analysis.report
}
