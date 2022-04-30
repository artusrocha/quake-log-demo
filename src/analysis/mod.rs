use regex::Regex;
use report::Report;

pub mod report;
//mod event;

pub struct Analysis {
    pub report: Report,
    rgx: Regex,
}

impl Analysis {

    pub fn new() -> Analysis {
        let pattern = r"(?P<time>\d+:\d+) (?P<action>Kill): (.*): (?P<killer>.*) killed (?P<victim>.*) by (?P<cause>\w+)";
        Analysis {
            report: Report::new(),
            rgx: Regex::new( pattern ).unwrap(),
        }
    }

    pub fn process_log(&mut self, data: &str) {
        self.rgx.captures_iter(data)
            .for_each(|cap| self.report.plus(&cap["killer"], &cap["victim"]));
    }
}
