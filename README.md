# PR Logger

This repo creates a list of PR data across multiple repositories, printed out in simple Markdown suitable for pasting into a chat.

Currently only Github PRs are supported.

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



