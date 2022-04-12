use serde::Deserialize;
use serde::Serialize;
use std::error;
use std::fs::File;
use std::fs::read_to_string;
use serde_json::Result;
use dirs::home_dir;

use crate::pr::{PrimPrList};
use crate::github::{prs_for_repo};

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoStore {
    repos: Vec<Repo>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Repo {
    kind: String,
    owner: String,
    repo: String,
}

// Change the alias to `Box<error::Error>`.
type BoxResult<T> = std::result::Result<T, Box<dyn error::Error>>;

pub fn load_repo_store(path: &str) -> Result<RepoStore> {
    // Open the file in read-only mode with buffer.
    let data = read_to_string(path).expect("Unable to read file");

    // Parse the string of data into RepoStore.
    let v: RepoStore = serde_json::from_str(&data).expect("Invalid prim data file");

    Ok(v)
}

pub fn write_repo_store(path: &str, repo_store: RepoStore) -> BoxResult<bool> {
    // Open a file in write-only mode, returns `io::Result<File>`
    let file = File::create(path)?;

    // Write the `User` to `file` returns `io::Result<()>`
    serde_json::to_writer(file, &repo_store)?;

    Ok(true)
}

fn prim_file_path() -> Result<String> {
    let path = format!("{}/{}", home_dir().unwrap().to_str().unwrap(), ".prim");
    return Ok(path)
}

pub fn list_repos() -> Result<RepoStore> {
    let path = prim_file_path()?;
    let repo_store = load_repo_store(&path).expect("Could not load data file");
    return Ok(repo_store);
}

pub fn add_repo(owner: &str, repo: &str, kind: &str) -> Result<bool> {
    let path = prim_file_path()?;
    let mut repo_store = load_repo_store(&path).expect("Could not load data file");
    let mut repos = repo_store.repos;
    repos.push(Repo {
        kind: kind.to_string(),
        owner: owner.to_string(),
        repo: repo.to_string(),  
    });
    repo_store.repos = repos;
    let result = write_repo_store(&path, repo_store).expect("Could not write data file");
    return Ok(result);
}

pub fn remove_repo(owner: &str, repo: &str, kind: &str) -> Result<bool> {
    let path = prim_file_path()?;
    let mut repo_store = load_repo_store(&path).expect("Could not load data file");
    let mut repos = repo_store.repos;
    repos.retain(|x| {
        if x.owner == owner && x.repo == repo && x.kind == kind {
            false
        } else {
            true
        }
    });
    repo_store.repos = repos;
    let result = write_repo_store(&path, repo_store).expect("Could not write data file");
    return Ok(result);
}

pub async fn list_pulls() -> Result<()> {
    let path = prim_file_path()?;
    let repo_store = load_repo_store(&path).expect("Could not load data file");
    let mut all_prs: PrimPrList = PrimPrList(vec![]);
    for repo in repo_store.repos {
        match repo.kind.as_str() {
            "github" => {
                let mut list = prs_for_repo(&repo.owner, &repo.repo).expect("Could not get prs for repo");
                all_prs.0.append(&mut list);
            },
            _ => {
                println!("Unsupported repo kind: {}", repo.kind);
            }
        }
    }

    println!("{}", all_prs);

    Ok(())
}
