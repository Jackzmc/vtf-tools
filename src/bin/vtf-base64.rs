use std::env;
use std::fs::File;
use std::io::Read;
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
    if !path.exists() {
        eprintln!("File does not exist");
        std::process::exit(1)
    }
    let mut file = File::open(path)?;
    let mut buf = Vec::with_capacity(700000);
    file.read_to_end(&mut buf)?;

    let vtf = vtf::from_bytes(&mut buf)?;
    let image = vtf.highres_image.decode(0)?;

    let rgba = image.to_rgba();
    buf.clear();
    let encoder = image::png::PNGEncoder::new(&mut buf);
    encoder.encode(
        &rgba, 
        rgba.width(), rgba.height(),
        image::ColorType::RGBA(8)
    )?;

    let encoded = base64::encode_config(&buf, base64::STANDARD_NO_PAD);

    print!("{}", encoded);
    Ok(())
}