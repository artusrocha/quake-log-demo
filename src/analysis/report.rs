use std::collections::HashMap;

#[derive(Debug)]
pub struct Report {
    total_kills: i32,
    players: Vec<String>,
    kills: HashMap<String,i32>,
}

impl Report {

    pub fn new() -> Report {
        Report {
            total_kills: 0,
            players: vec![],
            kills: HashMap::new(),
        }
    }

    pub fn plus(&mut self, killer_str: &str, victim_str: &str) {
        let killer = killer_str.to_owned();
        let victim = victim_str.to_owned();

        self.total_kills += 1;
        if ! self.players.contains( &killer ) {
            self.players.push( killer.clone() )
        }
        if ! self.players.contains( &victim ) {
            self.players.push( victim )
        }

        *self.kills.entry( killer ).or_default() += 1;
    }
}