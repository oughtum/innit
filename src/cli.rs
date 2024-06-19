use chrono::{Datelike, Local};
use dialoguer::{theme::ColorfulTheme, Confirm, FuzzySelect, Input};

use crate::utils::get_git_username;

pub fn prompt_branch() -> String {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Branch")
        .default("main".into())
        .interact_text()
        .unwrap()
}

pub fn prompt_remote_name() -> String {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Remote name")
        .default("origin".into())
        .interact_text()
        .unwrap()
}

pub fn prompt_remote_url() -> String {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Remote URL")
        .interact_text()
        .unwrap()
}

pub fn prompt_commit_changes() -> bool {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Commit changes?")
        .default(true)
        .interact()
        .unwrap()
}

pub fn prompt_commit_msg() -> String {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Commit message")
        .default("initial commit".into())
        .interact_text()
        .unwrap()
}

pub fn prompt_push_changes() -> bool {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Push changes?")
        .default(true)
        .interact()
        .unwrap()
}

pub fn prompt_gen_readme() -> bool {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Generate README?")
        .default(true)
        .interact()
        .unwrap()
}

pub fn prompt_gen_gitignore() -> bool {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Generate .gitignore?")
        .default(true)
        .interact()
        .unwrap()
}

pub fn prompt_gitignore_template<T>(templates: &[T]) -> Option<String>
where
    T: ToString,
{
    FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose a template")
        .default(0)
        .items(templates)
        .max_length(12)
        .interact_opt()
        .unwrap()
        .map(|i| templates[i].to_string())
}

pub fn prompt_license<T>(licenses: &[T]) -> Option<String>
where
    T: ToString,
{
    FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose a license")
        .default(0)
        .items(licenses)
        .interact_opt()
        .unwrap()
        .map(|i| licenses[i].to_string())
}

pub fn prompt_name() -> String {
    match get_git_username() {
        Some(name) => Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Name")
            .default(name.trim_ascii().into())
            .interact()
            .unwrap(),
        None => Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Name")
            .interact_text()
            .unwrap(),
    }
}

pub fn prompt_year() -> String {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Year")
        .default(Local::now().year().to_string())
        .interact_text()
        .unwrap()
}

pub fn prompt_new_path(path: &str) -> String {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt(&format!(
            "Path `{}` already exists, enter a new name or it will be overwritten",
            path
        ))
        .with_post_completion_text(format!("Choose `{}` path", path))
        .default(path.into())
        .interact_text()
        .unwrap()
}
