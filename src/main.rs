use std::io;

use dialoguer::console::Style;
use gitignore::{Template, Templates};
use license::Licenses;

pub mod cli;
pub mod gitignore;
pub mod license;
pub mod utils;

use cli::{prompt_gen_gitignore, prompt_gen_readme, prompt_gitignore_template, prompt_license};
use utils::{finalise_repo, generate_license, init_repo, write_to_file};

fn main() -> io::Result<()> {
    init_repo()?;

    // generate README.md? (Y/n)
    let gen_readme = prompt_gen_readme();

    // generate .gitignore? (Y/n)
    let gen_gitignore = prompt_gen_gitignore();

    // if yes, choose template (OPTIONAL)
    let template = if gen_gitignore {
        let templates = Templates::get();
        prompt_gitignore_template(&templates.0)
    } else {
        None
    };

    // choose a license (OPTIONAL)
    let licenses = Licenses::get();
    let license_names = licenses.get_names();

    let license = prompt_license(&license_names);

    // generate README if specified
    if gen_readme {
        write_to_file("README.md", "# README")?;
    }

    // generate .gitignore with/without template
    if let Some(temp) = &template {
        let template_content = Template::get(temp);
        write_to_file(".gitignore", &template_content.source)?;
    } else {
        write_to_file(".gitignore", "")?;
    }

    // generate license if specified
    if let Some(lic) = &license {
        let license_content = licenses.get_license_from_name(lic);

        generate_license(license_content)?;
    };

    finalise_repo()?;

    println!(
        "{}",
        Style::new().for_stdout().green().apply_to(
            "✔ Repository successfully initialised. Please ensure everything is correct."
        )
    );

    Ok(())
}
