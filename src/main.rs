use agprefs::Agpref;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    for path in std::env::args().skip(1) {
        let string = std::fs::read_to_string(&path)?;
        let agprefs = Agpref::from_str(&string)?;
        assert_eq!(agprefs.to_str()?, string);
    }
    Ok(())
}


