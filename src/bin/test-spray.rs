use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::vec::Vec;
use serde_json::json;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct SafeBrowsingResponse {
    responses: Vec<SafeSearchAnnotation>
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct SafeSearchAnnotation {
    safeSearchAnnotation: SafeSearchResult
}

#[derive(Serialize, Deserialize, Debug)]
struct SafeSearchResult {
    adult: String,
    spoof: String,
    medical: String,
    violence: String,
    racy: String
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let apikey = match std::env::var("SAFESEARCH_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("Need Google Vision API key environmental variable :'SAFESEARCH_API_KEY'");
            std::process::exit(1)
        }
    };
    

    if args.len() < 2 {
        eprintln!("Usage: test-spray <path to .dat/.vtf file>");
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

    let rgba = image.into_rgba();
    buf.clear();
    let encoder = image::png::PNGEncoder::new(&mut buf);
    encoder.encode(
        &rgba, 
        rgba.width(), rgba.height(),
        image::ColorType::RGBA(8)
    )?;

    let encoded = base64::encode_config(&buf, base64::STANDARD_NO_PAD);

    let req_json = json!({
        "requests": [{
            "image": {
                "content": encoded 
            },
            "features": [
                {
                "type": "SAFE_SEARCH_DETECTION"
                }
            ]
        }]
    });
    let url = format!("https://vision.googleapis.com/v1/images:annotate?key={}", apikey);

    let resp = reqwest::blocking::Client::new()
        .post(url)
        .json(&req_json)
        .send()?
        .json::<SafeBrowsingResponse>()?;

    print!("{}", resp.responses[0].safeSearchAnnotation.adult);
    Ok(())
}
