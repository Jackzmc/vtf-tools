use std::fs::File;
use std::path::Path;
use std::io::Read;

pub fn get_vtf_image(path: &Path, buf: &mut Vec<u8>) -> Result<image::DynamicImage, vtf::Error> {
    if !path.exists() {
        eprintln!("File does not exist");
        std::process::exit(1)
    }
    let mut file = File::open(path)?;
    //let mut buf = Vec::new();
    file.read_to_end(buf)?;

    let vtf = vtf::from_bytes(buf)?;
    vtf.highres_image.decode(0)
}

pub fn get_vtf_as_base64_png(path: &Path) -> Result<String, vtf::Error> {
    let mut buf = Vec::new();
    let image = get_vtf_image(&path, &mut buf)?;

    let rgba = image.into_rgba();
    buf.clear();
    let encoder = image::png::PNGEncoder::new(&mut buf);
    encoder.encode(
        &rgba, 
        rgba.width(), rgba.height(),
        image::ColorType::RGBA(8)
    )?;

    Ok(base64::encode_config(&buf, base64::STANDARD_NO_PAD))
}