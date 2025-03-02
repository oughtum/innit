# innit

![A GIF showing the usage of `innit` and the prompts it displays](innit.gif)

## Overview

`innit` is a blazingly fast 🚀 commandline utility for initialising git repositories quickly and easily. I created this to solve the common problem of having to generate a `.gitignore`, `README.md` & `LICENSE` for most of the projects I make - this tool makes it trivial to set up that boilerplate for a new project.

  NOTE: I am aware that the [Github CLI](https://cli.github.com/) provides this exact functionality when creating a new repository, however having this separate from the Github CLI is important to me for two reasons:

- I might want to migrate to another VCS hosting solution in future, like GitLab
- I abandon projects a lot (thanks ADHD), so I'd rather just be able to do `rm -rf local_repo/` than have to delete a remote repo, particularly if I don't have access to the internet for whatever reason

I don't expect anybody to actually use this, I just made it for myself but if somebody out there finds it useful, then great!

## Features

Below is the options you can change, their types, and their default values where applicable:

| Option                 | Type     | Default                         |
| ---------------------- | -------- | ------------------------------- |
| `generate README?`     | `bool`   | `true`                          |
| `generate .gitignore?` | `bool`   | `true`                          |
| `.gitignore template`  | `string` | -                               |
| `license`              | `string` | -                               |
| `name`                 | `string` | `git config --global user.name` |
| `year`                 | `string` | `current year`                  |
| `branch`               | `string` | `'main'`                        |
| `remote name`          | `string` | `'origin'`                      |
| `remote URL`           | `string` | -                               |
| `commit changes?`      | `bool`   | `true`                          |
| `commit message`       | `string` | `'initial commit'`              |
| `push changes?`        | `bool`   | `true`                          |

## Dependencies

- cargo
- git

## Installation

### Cargo

```bash
cargo install innit
```

### Source

```bash
git clone https://github.com/oughtum/innit.git
cd innit
cargo install --path .
```

Or as a single command:

```bash
git clone https://github.com/oughtum/innit.git && cd innit && cargo install --path .
```

## Usage

```bash
innit
```
