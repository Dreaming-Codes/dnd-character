use std::fs;

use dnd_character::Character;
use serde_json::json;

#[test]
fn schema_compat_0_13_17() {
    let file = fs::read_to_string("./tests/schema_0_13_17.json")
        .expect("Failed to read 0.13.17 schema file");

    let dnd_character: Character =
        serde_json::from_str(&file).expect("Failed to parse 0.13.17 schema");
}
