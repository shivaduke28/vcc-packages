extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Package {
    name: String,
    version: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Release {
    tag_name: String,
}

const RELEASE_URL: &str = "https://api.github.com/repos/shivaduke28/kanikama/releases/latest";
const REPOSITORY_PATH: &str = "https://raw.githubusercontent.com/shivaduke28/kanikama";
const KANIKAMA_PATH: &str = "Kanikama/Packages/net.shivaduke28.kanikama/package.json";

// Name your user agent after your app?
static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    // build http client
    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()?;

    // get latest released tag
    let res: Release = client.get(RELEASE_URL).send().await?.json().await?;
    let tag = res.tag_name;
    println!("tag:{}", tag);

    // download package.json
    let url = String::from(REPOSITORY_PATH) + "/" + &tag + "/" + KANIKAMA_PATH;
    let response = reqwest::get(url).await?;
    let text = response.text().await?;
    let package: Package = serde_json::from_str(&text).unwrap();

    println!("kanikama version: {:#?}", package.version);
    Ok(())
}
