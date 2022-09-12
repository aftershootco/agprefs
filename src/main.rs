use agprefs::Agpref;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    for path in std::env::args().skip(1) {
        let string = std::fs::read_to_string(&path)?;
        let agprefs = Agpref::from_str(&string)?;
        for agpref in agprefs.iter() {
            println!("\x1b[32m{} :: \x1b[31m{:#?}", agpref.0, agpref.1);
        }
    }
    Ok(())
}
