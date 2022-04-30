use std::io;
use std::env;
use std::fs;
use regex::Regex;
use std::collections::HashMap;


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

struct Analysis {
    report: Report,
    rgx: Regex,
}

impl Analysis {

    fn new() -> Analysis {
        let pattern = r"(?P<time>\d+:\d+) (?P<action>Kill): (.*): (?P<killer>.*) killed (?P<victim>.*) by (?P<cause>\w+)";
        Analysis {
            report: Report::new(),
            rgx: Regex::new( pattern ).unwrap(),
        }
    }

    fn process_log(&mut self, data: &str) {
        self.rgx.captures_iter(data)
            .map(|cap| Event::new(&cap) )
            .for_each(|event| self.report.plus(&event));
    }
}

#[derive(Debug)]
struct Report {
    total_kills: i32,
    players: Vec<String>,
    kills: HashMap<String,i32>,
}

impl Report {

    fn new() -> Report {
        Report {
            total_kills: 0,
            players: vec![],
            kills: HashMap::new(),
        }
    }

    fn plus(&mut self, event: &Event) {
        self.total_kills = self.total_kills + 1;
        if ! self.players.contains(&event.killer) {
            self.players.push(event.killer.to_string())
        }
        if ! self.players.contains(&event.victim) {
            self.players.push(event.victim.to_owned())
        }

        *self.kills.entry( event.killer.to_owned() ).or_default() += 1;
    }
}

#[derive(Debug)]
struct Event {
    time: String,
    action: String,
    killer: String,
    victim: String,
    cause: String,
}

impl Event {
    fn new(cap: &regex::Captures) -> Event {
        Event {
            time: cap["time"].to_string(), 
            action: cap["action"].to_string(), 
            killer: cap["killer"].to_string(), 
            victim: cap["victim"].to_string(), 
            cause: cap["cause"].to_string(),
        }
    }
}
