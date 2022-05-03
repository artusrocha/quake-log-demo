use std::collections::HashMap;
use serde::Serialize;
use std::str::FromStr;
use strum_macros::EnumString;

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


#[derive(Hash, Debug, PartialEq, Eq, EnumString, Serialize)]
#[allow(non_camel_case_types)]
enum DeathType {
  MOD_UNKNOWN,
  MOD_SHOTGUN,
  MOD_GAUNTLET,
  MOD_MACHINEGUN,
  MOD_GRENADE,
  MOD_GRENADE_SPLASH,
  MOD_ROCKET,
  MOD_ROCKET_SPLASH,
  MOD_PLASMA,
  MOD_PLASMA_SPLASH,
  MOD_RAILGUN,
  MOD_LIGHTNING,
  MOD_BFG,
  MOD_BFG_SPLASH,
  MOD_WATER,
  MOD_SLIME,
  MOD_LAVA,
  MOD_CRUSH,
  MOD_TELEFRAG,
  MOD_FALLING,
  MOD_SUICIDE,
  MOD_TARGET_LASER,
  MOD_TRIGGER_HURT,
  MOD_NAIL,
  MOD_CHAINGUN,
  MOD_PROXIMITY_MINE,
  MOD_KAMIKAZE,
  MOD_JUICED,
  MOD_GRAPPLE
}
