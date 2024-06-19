use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Template {
    pub name: String,
    pub source: String,
}

impl Template {
    pub fn get(name: &str) -> Self {
        match ureq::get(&format!(
            "https://api.github.com/gitignore/templates/{}",
            name
        ))
        .set("Accept", "application/vnd.github+json")
        .set("X-GitHub-Api-Version", "2022-11-28")
        .call()
        {
            Ok(res) => res.into_json::<Template>().unwrap(),
            Err(err) => panic!("Error fetching .gitignore template `{}`: {}", name, err),
        }
    }
}

pub struct Templates(pub Vec<String>);

impl Templates {
    pub fn get() -> Self {
        match ureq::get("https://api.github.com/gitignore/templates")
            .set("Accept", "application/vnd.github+json")
            .set("X-GitHub-Api-Version", "2022-11-28")
            .call()
        {
            Ok(res) => Templates(res.into_json::<Vec<String>>().unwrap()),
            Err(err) => panic!("Error fetching .gitignore templates: {}", err),
        }
    }
}
