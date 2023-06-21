use agprefs::Agpref;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    for path in std::env::args().skip(1) {
        let s = std::fs::read_to_string(&path)?;
        let agprefs = Agpref::parse(&s)?;
        #[cfg(feature = "composer")]
        println!("{}", agprefs.to_str()?);
    }
    Ok(())
}
