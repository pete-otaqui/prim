# PR Logger

This repo creates a list of PR data across multiple repositories, printed out in simple Markdown suitable for pasting into a chat.

Currently only Github PRs are supported.

## Sample output

Here's some example output - PRs are collected by "type" (Open, Bot, Draft) across different repositories:

> ## Open PRs (1):
> 
> **PR Title goes here**
> - https://github.com/pete-otaqui/prim/pulls/1
> - 0 reviews, 1 comment
> - Created by pete-otaqui 3 hours ago
> 
> ## Bot PRs (2):
> 
> **Bump glob-parent from 5.1.1 to 5.1.2 in /code/flock**
> - https://github.com/pete-otaqui/pete-otaqui.github.io/pull/4
> - 0 reviews, 0 comments
> - Created by dependabot 9 months ago
> 
> **Bump minimist from 1.2.5 to 1.2.6 in /code/flock**
> - https://github.com/pete-otaqui/pete-otaqui.github.io/pull/6
> - 0 reviews, 0 comments
> - Created by dependabot 3 weeks ago
> 
> ## Draft PRs (0):



## Setup

You'll need a Github Token environment variable called `GITHUB_TOKEN`.

You will also need a `~/.prim` file in this format:

```json
[
  {"kind": "github", "owner": "first-github-owner", "repo": "some-github-repo-1" },
  {"kind": "github", "owner": "first-github-owner", "repo": "some-github-repo-2" },
  {"kind": "github", "owner": "another-github-owner", "repo": "other-github-repo" }
]
```

## Usage

Once setup and installed you can use `prim` as follows:

- `prim` - fetch all PRs.
- `prim fetch` - as above.
- `prim list` - list all repositories.
- `prim add --kind github --owner repo-owner --repo repo-name` - add a repo to the `~/.prim` file.
- `prim remove --kind github --owner repo-owner --repo repo-name` - remove a repo from the `~/.prim` file.

## Usage during development

Prim is made using [rust](https://www.rust-lang.org/), you will need to set that up to develop / build the project.

* `cargo run` will execute prim from source.
* `cargo build --release` will build the release binary.
* `./target/release/prim` will execute the release binary.

Copy `./target/release/prim` to somewhere in your PATH and just call `prim`.



