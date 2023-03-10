use rf1::run;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    match run() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("ERROR: {e}");
        }
    }
    Ok(())
}
