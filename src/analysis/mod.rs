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
        	         Some(r) => r.plus(&cap["killer"], &cap["victim"], &cap["cause"]),
        	         None => eprintln!("Error: kill without game"),
        	       }
            	});
        }
    }
}

#[cfg(test)]
mod test {
    use crate::analysis::Analysis;

    #[test]
    fn should_create_reports_with_zero_len() {
        let analisys = Analysis::new();
        assert_eq!(analisys.reports.len(), 0);
    }

    #[test]
    fn should_create_reports_with_one_len() {
        let game_log = [
            "0:00 ------------------------------------------------------------",
            "0:00 InitGame: ",
            "8:04 Kill: 2 5 7: Foo killed Bar by MOD_ROCKET_SPLASH",
            "20:37 ShutdownGame:" ].join("\n");
        let mut analisys = Analysis::new();
        analisys.process_log(&game_log);
        assert_eq!(analisys.reports.len(), 1);
    }

    #[test]
    fn should_create_reports_with_two_len() {
        let game_log = [
            "0:00 ------------------------------------------------------------",
            "0:00 InitGame: ",
            "8:04 Kill: 2 5 7: Bar killed Foo by MOD_ROCKET_SPLASH",
            "20:37 ShutdownGame:",
            "0:00 ------------------------------------------------------------",
            "0:00 InitGame: ",
            "8:04 Kill: 2 5 7: Foo killed Bar by MOD_ROCKET_SPLASH",
            "20:37 ShutdownGame:" ].join("\n");
        let mut analisys = Analysis::new();
        analisys.process_log(&game_log);
        assert_eq!(analisys.reports.len(), 2);
    }
}