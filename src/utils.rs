use std::{fs, io, path::Path, process::Command};

use crate::{
    cli::{
        prompt_branch, prompt_commit_changes, prompt_commit_msg, prompt_name, prompt_new_path,
        prompt_push_changes, prompt_remote_name, prompt_remote_url, prompt_year,
    },
    license::LicenseContent,
};

pub fn init_repo() -> io::Result<()> {
    Command::new("git").arg("init").output()?;

    Ok(())
}

pub fn finalise_repo() -> io::Result<()> {
    let branch = prompt_branch();
    let remote_name = prompt_remote_name();
    let remote_url = prompt_remote_url();

    // track files
    Command::new("git").args(["add", "."]).output()?;

    // set branch
    Command::new("git")
        .args(["branch", "-M", &branch])
        .output()?;

    // add remote
    Command::new("git")
        .args(["remote", "add", &remote_name, &remote_url])
        .output()?;

    let commit_changes = prompt_commit_changes();

    if commit_changes {
        let commit_msg = prompt_commit_msg();

        // commit changes
        Command::new("git")
            .args(["commit", "-am", &commit_msg])
            .output()?;

        let push_changes = prompt_push_changes();

        if push_changes {
            // push changes
            Command::new("git")
                .args(["push", "-u", &remote_name, &branch])
                .output()?;
        }
    }

    Ok(())
}

pub fn generate_license(license: LicenseContent) -> io::Result<()> {
    let name = prompt_name();
    let year = prompt_year();

    let body = license
        .body
        .replace("[year]", &year)
        .replace("[yyyy]", &year)
        .replace("<year>", &year)
        .replace("[fullname]", &name)
        .replace("[name of copyright owner]", &name)
        .replace("<name of author>", &name);

    write_to_file("LICENSE", &body)?;

    Ok(())
}

// attempts to get git username from global config
// as default for prompt
pub fn get_git_username() -> Option<String> {
    let cmd = Command::new("git")
        .args(["config", "--global", "--get", "user.name"])
        .output()
        .ok()?;

    match cmd.status.success() {
        true => Some(String::from_utf8_lossy(&cmd.stdout).to_string()),
        false => None,
    }
}

pub fn write_to_file(path: &str, content: &str) -> io::Result<()> {
    match Path::new(path).exists() {
        true => {
            let new_path: String = prompt_new_path(path);

            fs::write(new_path, content)
        }
        false => fs::write(path, content),
    }
}
