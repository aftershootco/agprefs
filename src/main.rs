use agprefs::parser::agprefs;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    for path in std::env::args().skip(1) {
        let string = std::fs::read_to_string(&path)?;
        let agp = agprefs(string.as_str()).unwrap();
        // dbg!(x.get("libraryToLoad20").unwrap());
        for i in agp.iter() {
            println!("\x1b[32m{} :: \x1b[31m{:#?}", i.1.name, i.1.value);
        }
    }
    Ok(())
}
