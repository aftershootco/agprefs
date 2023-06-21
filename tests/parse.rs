pub const ASSETS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/assets");
use agprefs::Agpref;
#[test]
pub fn one() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/assets/1.agprefs"
    ));
    Agpref::parse(input).unwrap();
}

#[test]
pub fn two() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/assets/2.agprefs"
    ));
    Agpref::parse(input).unwrap();
}

#[test]
pub fn nikhil() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/assets/nikhil.agprefs"
    ));
    Agpref::parse(input).unwrap();
}

#[test]
pub fn windows() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/assets/windows.agprefs"
    ));
    let x = Agpref::parse(input).unwrap();
    let output = Agpref::to_str(&x).unwrap();
    std::fs::write("x.agprefs", &output).unwrap();
    assert_eq!(&input, &output);

    let recents = x
        .values
        .get_struct()
        .unwrap()
        .get("recentLibraries20")
        .unwrap()
        .get_string()
        .unwrap();
    let agprefs = Agpref::parse(recents).unwrap();
    let ss: Vec<&str> = agprefs
        .values
        .get_values()
        .unwrap()
        .iter()
        .flat_map(|p| p.get_string())
        .collect();
    assert_eq!(
        ["C:\\Users\\harsh\\Pictures\\Lightroom\\Lightroom Catalog.lrcat"].as_slice(),
        &ss
    )
}

#[test]
pub fn db() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/assets/db.agprefs"
    ));
    let x = Agpref::parse(input).unwrap();
    let output = Agpref::to_str(&x).unwrap();
    assert_eq!(input, output);
}

#[test]
pub fn metadata() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/assets/metadata"
    ));
    let x = Agpref::parse(input).unwrap();
    let output = Agpref::to_str(&x).unwrap();
    assert_eq!(input, output);
}
#[test]
pub fn fail() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/assets/failure"));
    Agpref::parse(input).unwrap_err();
}

#[cfg(feature = "serde")]
#[test]
pub fn serialize_metadata() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/assets/metadata"
    ));
    let agpref = Agpref::parse(input).unwrap();
    let out = serde_json::to_string(&agpref).unwrap();
    let agpref2 = serde_json::from_str(&out).unwrap();
    assert_eq!(agpref, agpref2);
}
