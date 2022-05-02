use regex::Regex;
use report::Report;

pub mod report;

pub struct Analysis {
    pub reports: Vec<Report>,
    games_rgx: Regex,
    kills_rgx: Regex,
}

impl Analysis {

    pub fn new() -> Analysis {
        let games_pattern = r"(\d+:\d+ InitGame: .*)";
        let kills_pattern = r"(?P<time>\d+:\d+) (?P<action>Kill): (.*): (?P<killer>.*) killed (?P<victim>.*) by (?P<cause>\w+)";
        Analysis {
            reports: vec![],
            games_rgx: Regex::new( games_pattern ).unwrap(),
            kills_rgx: Regex::new( kills_pattern ).unwrap(),
        }
    }

    pub fn process_log(&mut self, data: &str) {
        for line in data.lines() {
        	if self.games_rgx.is_match(&line) {
	        	self.reports.push( Report::new() );        		
        	}

	        self.kills_rgx.captures_iter( &line )
    	        .for_each(|cap| {
        	       match self.reports.last_mut() {
        	         Some(r) => r.plus(&cap["killer"], &cap["victim"]),
        	         None => eprintln!("Error: kill without game"),
        	       }
            	});
        }
    }
}
