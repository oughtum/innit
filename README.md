# innit

![A GIF showing the usage of `innit` and the prompts it displays](innit.gif)

## Overview

`innit` is a blazingly fast 🚀 commandline utility for initialising git repositories quickly and easily. I created this to solve the common problem of having to generate a `.gitignore`, `README.md` & `LICENSE` for most of the projects I make - this tool makes it trivial to set up that boilerplate for a new project.

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
