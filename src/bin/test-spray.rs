use std::env;
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
            if args.len() > 2 {
                args[2].clone()
            } else {
                eprintln!("Missing Google Vision API key as environmental variable 'SAFESEARCH_API_KEY' or as second argument, test-spray <input> [apikey]");
                std::process::exit(1)
            }
        }
    };
    

    if args.len() < 2 {
        eprintln!("Usage: test-spray <path to .dat/.vtf file> [safe search api key]");
        std::process::exit(1)
    }
    let path = Path::new(&args[1]);
    let encoded = vtf_tools::get_vtf_as_base64_png(path)?;

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
    let types = &resp.responses[0].safeSearchAnnotation;
    print!("adult={}\nracy={}\nmedical={}", types.adult, types.racy, types.medical);
    Ok(())
}
