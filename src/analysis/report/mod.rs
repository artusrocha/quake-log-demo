use std::collections::HashMap;
use serde::Serialize;
use std::str::FromStr;

use self::death_types::DeathType;

mod death_types;

#[cfg(test)]
mod test;

static WORLD: &str = "<world>";

#[derive(Serialize, Debug)]
pub struct Report {
    total_kills: u16,
    players: Vec<String>,
    kills: HashMap<String,u16>,
    kills_by_means: HashMap<DeathType,u16>,
}

impl Report {

    pub fn new() -> Report {
        Report {
            total_kills: 0,
            players: vec![],
            kills: HashMap::new(),
            kills_by_means: HashMap::new(),
        }
    }

    pub fn plus(&mut self, killer: &str, victim: &str, cause: &str) {
        self.total_kills += 1;
        self.add_player( victim.to_owned() );

        if killer != WORLD {
	        self.add_player( killer.to_owned() );
    	    *self.kills.entry( killer.to_owned() ).or_default() += 1;
    	}
    	
        match DeathType::from_str( cause ) {
            Ok(death_type) => *self.kills_by_means.entry( death_type ).or_default() += 1,
            Err(e) => eprintln!("Error DeathType not found: {:#?}", e),
        }
    }

    fn add_player(&mut self, player: String) {
        if ! self.players.contains( &player ) {
            self.players.push( player )
        }    	
    }
}

