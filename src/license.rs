#![allow(dead_code)]

use serde::Deserialize;

// taken from https://docs.github.com/en/rest/licenses/licenses?apiVersion=2022-11-28
#[derive(Debug, Deserialize)]
pub struct LicenseMetadata {
    pub key: String,
    pub name: String,
    spdx_id: String,
    pub url: String,
    node_id: String,
}

// taken from https://docs.github.com/en/rest/licenses/licenses?apiVersion=2022-11-28
#[derive(Debug, Deserialize)]
pub struct LicenseContent {
    #[serde(flatten)]
    pub metadata: LicenseMetadata,
    pub description: String,
    pub permissions: Vec<String>,
    pub conditions: Vec<String>,
    pub limitations: Vec<String>,
    pub body: String,
}

impl LicenseContent {
    pub fn get(url: &str) -> Self {
        match ureq::get(url)
            .set("Accept", "application/vnd.github+json")
            .set("X-GitHub-Api-Version", "2022-11-28")
            .call()
        {
            Ok(res) => res.into_json::<LicenseContent>().unwrap(),
            Err(error) => panic!("Error fetching license content: {}", error),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Licenses(pub Vec<LicenseMetadata>);

impl Licenses {
    pub fn get() -> Self {
        match ureq::get("https://api.github.com/licenses")
            .set("Accept", "application/vnd.github+json")
            .set("X-GitHub-Api-Version", "2022-11-28")
            .call()
        {
            Ok(res) => Licenses(res.into_json::<Vec<LicenseMetadata>>().unwrap()),
            Err(error) => panic!("Error fetching licenses: {}", error),
        }
    }

    pub fn get_keys(&self) -> Vec<String> {
        self.0
            .iter()
            .map(|metadata| metadata.key.to_string())
            .collect()
    }

    pub fn get_names(&self) -> Vec<String> {
        self.0
            .iter()
            .map(|metadata| metadata.name.to_string())
            .collect()
    }

    pub fn get_license_from_name(&self, name: &str) -> LicenseContent {
        let result = &self
            .0
            .iter()
            .filter(|metadata| metadata.name == name)
            .map(|metadata| metadata.url.clone())
            .collect::<String>();

        LicenseContent::get(result)
    }
}
