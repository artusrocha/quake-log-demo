
use strum::ParseError;

use crate::analysis::report::{Report, death_types::DeathType};
use std::{str::{FromStr}};


#[test]
fn should_add_one_player() {
    let mut report = Report::new();
    report.add_player("Foo".to_owned());
    assert_eq!(report.players[0],"Foo");
}

#[test]
fn should_add_one_kill_entry() {
    let mut report = Report::new();
    report.plus("killer_foo", "victim_bar", "MOD_ROCKET");
    assert_eq!(report.players[0],"victim_bar");
    assert_eq!(report.players[1],"killer_foo");
    assert_eq!(report.kills["killer_foo"], 1);
}

#[test]
fn should_add_two_kill_entry() -> Result<(), ParseError> {
    let mut report = Report::new();
    report.plus("killer_foo", "victim_bar", "MOD_ROCKET");
    report.plus("killer_fizz", "victim_foo", "MOD_ROCKET");
    assert_eq!(report.kills["killer_foo"], 1);
    assert_eq!(report.kills["killer_fizz"], 1);
    assert_eq!(report.kills_by_means[&DeathType::from_str("MOD_ROCKET")?], 2);
    Ok(())
}
