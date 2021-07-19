use std::env;
use std::path::Path;
use std::vec::Vec;
use vtf::Error;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: vtf-to-base64 <path to .dat/.vtf file>");
        std::process::exit(1)
    }

    let path = Path::new(&args[1]);
    let encoded = vtf_tools::get_vtf_as_base64_png(path)?;

    print!("{}", encoded);
    Ok(())
}