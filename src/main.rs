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

    eprintln!("================================");
    eprintln!("");

    println!("{}", serde_json::to_string(&report).unwrap());
}

fn process_stdin() -> Vec<Report> {
    eprintln!("Reading from STDIN");
    let mut analysis = Analysis::new();
    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(n_bytes_readed) => {
                if n_bytes_readed == 0 {
                    eprintln!("End Of Input");
                    break
                }
                analysis.process_log(&input);
            }
            Err(error) => eprintln!("error: {}", error),
        }
        input.clear()
    }

    analysis.reports
}

fn process_file(filename: &str) -> Vec<Report> {
    eprintln!("Reading from file: {:?}", &filename);
    let contents = fs::read_to_string(filename)
                        .expect("There was an error reading the file");
    let mut analysis = Analysis::new();
    analysis.process_log(&contents.to_string());
    analysis.reports
}
