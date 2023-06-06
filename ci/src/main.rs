extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct Repos {
    name: String,
    id: String,
    url: String,
    author: String,
    packages: HashMap<String, Versions>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Versions {
    versions: HashMap<String, Package>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Package {
    name: String,
    #[serde(rename = "displayName")]
    display_name: String,
    version: String,
    unity: String,
    description: String,
    author: Author,
    #[serde(default)]
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Author {
    name: String,
    email: String,
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Release {
    tag_name: String,
}

const PACKGE_LIST_JSON_PATH: &str = "../docs/index.json";

const LATEST_RELEASE_URL: &str =
    "https://api.github.com/repos/shivaduke28/kanikama/releases/latest";
const RAW_REPOSITORY_URL: &str = "https://raw.githubusercontent.com/shivaduke28/kanikama";

const PACKAGES_DIR_GIT_PATH: &str =
    "https://github.com/shivaduke28/kanikama.git?path=/Kanikama/Packages";

const PACKAGE_NAME_KANIKAMA: &str = "net.shivaduke28.kanikama";
const PACKAGE_NAME_KANIKAMA_BAKERY: &str = "net.shivaduke28.kanikama.bakery";
const PACKAGE_NAME_KANIKAMA_UDON: &str = "net.shivaduke28.kanikama.udon";

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

async fn update_kanikama(
    repos: &mut Repos,
    tag: &str,
    package_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // download package.json in the released tag
    let package_json_url = String::from(RAW_REPOSITORY_URL)
        + "/"
        + tag
        + "/Kanikama/Packages/"
        + package_name
        + "/package.json";

    let package_json_str = reqwest::get(package_json_url).await?.text().await?;
    let mut package: Package = serde_json::from_str(&package_json_str).unwrap();

    // insert url to package.json
    package.url = String::from(PACKAGES_DIR_GIT_PATH) + "/" + PACKAGE_NAME_KANIKAMA + "#" + tag;

    // add versions to repos if not contained
    if !repos.packages.contains_key(&package.name) {
        repos.packages.insert(
            package.name.to_string(),
            Versions {
                versions: HashMap::new(),
            },
        );
    }

    // insert latest package
    repos
        .packages
        .get_mut(&package.name)
        .unwrap()
        .versions
        .insert(package.version.to_string(), package);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    // build http client
    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()?;

    // get latest released tag
    let release: Release = client.get(LATEST_RELEASE_URL).send().await?.json().await?;
    let tag = release.tag_name;
    println!("letest release: {}", tag);

    let repos_data = fs::read_to_string(PACKGE_LIST_JSON_PATH).expect("Unable to read file");
    let mut repos: Repos = serde_json::from_str(&repos_data).unwrap();

    update_kanikama(&mut repos, &tag, PACKAGE_NAME_KANIKAMA).await?;
    update_kanikama(&mut repos, &tag, PACKAGE_NAME_KANIKAMA_BAKERY).await?;
    update_kanikama(&mut repos, &tag, PACKAGE_NAME_KANIKAMA_UDON).await?;

    let _ = fs::write(
        PACKGE_LIST_JSON_PATH,
        serde_json::to_string_pretty(&repos).unwrap(),
    );

    dbg!(repos);

    Ok(())
}
