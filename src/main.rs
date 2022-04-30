use regex::Regex;
use std::collections::HashMap;


fn main() {
    
    let log_line = 
"8:04 Kill: 2 5 7: Dono da Bola killed Assasinu Credi by MOD_ROCKET_SPLASH
7:02 Item: 4 weapon_rocketlauncher
7:02 Kill: 1022 3 22: <world> killed Isgalamido by MOD_TRIGGER_HURT
7:02 Item: 5 weapon_shotgun
7:03 Item: 5 ammo_shells
7:03 Kill: 4 2 6: Zeh killed Dono da Bola by MOD_ROCKET
7:08 Item: 4 item_health_large
7:09 Item: 2 weapon_rocketlauncher
7:10 Item: 3 weapon_rocketlauncher
7:11 Item: 4 item_armor_shard
7:11 Item: 4 item_armor_shard
7:11 Item: 4 item_armor_shard
7:11 Item: 4 item_armor_combat
7:12 Kill: 2 5 7: Dono da Bola killed Assasinu Credi by MOD_ROCKET_SPLASH
7:12 Item: 2 weapon_rocketlauncher
7:12 Kill: 4 2 7: Zeh killed Dono da Bola by MOD_ROCKET_SPLASH
7:13 Item: 4 weapon_rocketlauncher
7:14 Item: 4 item_armor_shard
7:14 Item: 4 item_armor_shard
7:14 Item: 4 item_armor_shard
7:16 Item: 2 weapon_rocketlauncher
7:17 Item: 5 weapon_rocketlauncher
7:18 Item: 5 ammo_rockets
7:21 Item: 3 ammo_bullets
7:21 Kill: 2 2 7: Dono da Bola killed Dono da Bola by MOD_ROCKET_SPLASH
7:23 Kill: 4 3 7: Zeh killed Isgalamido by MOD_ROCKET_SPLASH
";

    let report = process_log(&log_line);
    println!("=================================");
    println!("{:#?}",report);
    println!("=================================");
}

fn process_log(data: &str) -> Report {

    let re = get_kill_event_regex();

    let mut report = Report::new();
    
    re.captures_iter(data)
        .map(|cap| Event::new(&cap) )
        .for_each(|event| report.plus(&event));
    
    report
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

fn get_kill_event_regex() -> Regex {
    let pattern = r"(?P<time>\d+:\d+) (?P<action>Kill): (.*): (?P<killer>.*) killed (?P<victim>.*) by (?P<cause>\w+)";
    Regex::new( pattern ).unwrap()
}