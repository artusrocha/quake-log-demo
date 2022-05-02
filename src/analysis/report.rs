use std::collections::HashMap;
use serde::Serialize;

static WORLD: &str = "<world>";

#[derive(Serialize, Debug)]
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

    pub fn plus(&mut self, killer: &str, victim: &str) {
        self.total_kills += 1;
        self.add_player( victim.to_owned() );

        if killer != WORLD {
	        self.add_player( killer.to_owned() );
    	    *self.kills.entry( killer.to_owned() ).or_default() += 1;
    	}
    }

    fn add_player(&mut self, player: String) {
        if ! self.players.contains( &player ) {
            self.players.push( player )
        }    	
    }
}
