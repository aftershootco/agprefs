pub const ASSETS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/assets");
use agprefs::Agpref;
use std::path::Path;
#[test]
pub fn one() {
    let file = Path::new(ASSETS).join("1.agprefs");
    let input = std::fs::read_to_string(file).unwrap();
    Agpref::from_str(&input).unwrap();
}

#[test]
pub fn two() {
    let file = Path::new(ASSETS).join("2.agprefs");
    let input = std::fs::read_to_string(file).unwrap();
    Agpref::from_str(&input).unwrap();
}

#[test]
pub fn nikhil() {
    let file = Path::new(ASSETS).join("nikhil.agprefs");
    let input = std::fs::read_to_string(file).unwrap();
    Agpref::from_str(&input).unwrap();
}

#[test]
pub fn windows() {
    let file = Path::new(ASSETS).join("windows.agprefs");
    let input = std::fs::read_to_string(file).unwrap();
    Agpref::from_str(&input).unwrap();
}

#[test]
pub fn db() {
    let file = Path::new(ASSETS).join("db.agprefs");
    let input = std::fs::read_to_string(file).unwrap();
    Agpref::from_str(&input).unwrap();
}

#[test]
pub fn metadata() {
    let file = Path::new(ASSETS).join("metadata");
    let input = std::fs::read_to_string(file).unwrap();
    Agpref::from_str(&input).unwrap();
}
