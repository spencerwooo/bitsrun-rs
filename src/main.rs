use reqwest;

const SRUN_PORTAL: &str = "http://10.0.0.55";

fn main() {
    // http get srun_portal, and get redirect url
    let client = reqwest::blocking::Client::new();
    let res = client.get(SRUN_PORTAL).send();
    if let Err(e) = res {
        println!("Error: {}", e);
        return;
    }

    let redirect_url = res.unwrap().url().to_string();
    println!("redirect_url: {}", redirect_url);
}
