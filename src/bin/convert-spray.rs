use image::DynamicImage;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::vec::Vec;
use vtf::Error;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 2 {
        eprintln!("Usage: convert-spray <path to .dat file> [dest]");
        std::process::exit(1)
    }

    let path = Path::new(&args[1]);
    let path = Path::new(&args[1]);
    let mut buf = Vec::new();
    let image = vtf_tools::get_vtf_image(&path, &mut buf)?;

    let destination: String = match args.len() {
        3 => args[2].to_owned(),
        _ => {
            Path::new(&args[1]).with_extension("png").to_string_lossy().to_string()
        }
    };
    // rgb and rgba images we can save directly, for other formats we convert to rgba
    match image {
        DynamicImage::ImageRgb8(_) | DynamicImage::ImageRgba8(_) => image.save(destination)?,
        DynamicImage::ImageBgra8(_) => image.to_rgba().save(destination)?,
        _ => image.to_rgb().save(destination)?,
    };
    
    Ok(())
}