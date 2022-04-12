use clap::StructOpt;
use futures::executor::block_on;
use prim::{list_repos,add_repo,remove_repo,list_pulls};

#[derive(StructOpt, Debug)]
#[structopt(name = "prim")]
struct Opt {
    #[structopt(subcommand)]
    cmd: Option<Command>,
}
#[derive(StructOpt, Debug)]
enum Command {
    /// Fetch PRs
    /// 
    /// Fetch PRs for all the repositories, this is the default
    Fetch {},

    /// List repositories
    ///
    /// List the repositories whose PRs are included
    List {},
    
    /// Add repository
    /// 
    /// Add a repository to the list
    Add {
      #[structopt(short, long)]
      /// The kind, only "github" is supported
      kind: String,
      #[structopt(short, long)]
      /// The owner, e.g. for this repo it's "pete-otaqui"
      owner: String,
      #[structopt(short, long)]
      /// The repo, e.g. for this repo it's "prim"
      repo: String,
    },

    /// Remove repository
    /// 
    /// Remove a repository from the list
    Remove {
      #[structopt(short, long)]
      /// The kind, only "github" is supported
      kind: String,
      #[structopt(short, long)]
      /// The owner, e.g. for this repo it's "pete-otaqui"
      owner: String,
      #[structopt(short, long)]
      /// The repo, e.g. for this repo it's "prim"
      repo: String,
    },
}

fn main() {

    let opt = Opt::parse();
    let repo_store = list_repos();

    match opt.cmd {
        None | Some(Command::Fetch {}) => {
            let future = list_pulls();
            block_on(future).expect("Could not fetch PRs");
        },
        Some(Command::List {}) => {
            println!("Listing repositories");
            println!("{:#?}", repo_store);
        },
        Some(Command::Add { kind, owner, repo }) => {
            add_repo(&kind, &owner, &repo).expect("Could not add repo");
            println!("Added repository");
        },
        Some(Command::Remove { kind, owner, repo }) => {
            remove_repo(&kind, &owner, &repo).expect("Could not remove repo");
            println!("Removed repository");
        },
    }
}
