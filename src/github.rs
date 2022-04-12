use ::reqwest::blocking::Client;
use graphql_client::{reqwest::post_graphql_blocking as post_graphql, GraphQLQuery};

use crate::pr::PrimPr;
use crate::age::readable_age_from_string;

// Force DateTime query results to be "String"
type DateTime = String;

#[allow(clippy::upper_case_acronyms)]
type URI = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/github-graphql/github.graphql",
    query_path = "src/github-graphql/PrList.graphql",
    response_derives = "Debug"
)]
struct PrList;

fn is_mergeable(mergeable: pr_list::MergeableState) -> bool {
  match mergeable {
    pr_list::MergeableState::MERGEABLE => true,
    _ => false,
  }
}

fn is_bot(author_type: pr_list::PrListRepositoryPullRequestsNodesAuthorOn) -> bool {
  match author_type {
    pr_list::PrListRepositoryPullRequestsNodesAuthorOn::Bot => true,
    _ => false,
  }
}

pub fn prs_for_repo(owner: &str, repo: &str) -> Result<Vec<PrimPr>, Box<dyn std::error::Error>> {
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");

    
    let variables = pr_list::Variables {
      owner: owner.to_string(),
      name: repo.to_string(),
    };

    let mut prim_prs: Vec<PrimPr> = Vec::new();
    
    let client = Client::builder()
        .user_agent("graphql-rust/0.10.0")
        .default_headers(
            std::iter::once((
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("Bearer {}", token))
                    .unwrap(),
            ))
            .collect(),
        )
        .build()?;

    let response_body =
        post_graphql::<PrList, _>(&client, "https://api.github.com/graphql", variables).unwrap();
    
    // println!("BODY: {:?}", response_body);

    let response_data: pr_list::ResponseData = response_body.data.expect("missing response data");
    
    // println!("DATA: {:?}", response_data);

    let repository = response_data.repository.expect("missing repository");

    let prs = repository.pull_requests.nodes.unwrap();
    
    // println!("PRS: {:?}", prs);
    for wrapped_pr in prs {
        let pr = wrapped_pr.unwrap();
        let title = pr.title;
        let url = pr.url;
        let pr_created_at = pr.created_at;
        let mergeable = is_mergeable(pr.mergeable);
        let comments = pr.comments.total_count as u8;
        let review_list = pr.reviews.unwrap();
        let reviews = review_list.total_count as u8;
        let age = readable_age_from_string(&pr_created_at).expect("could not parse date");
        let draft = pr.is_draft;
        let pr_author = pr.author.unwrap();
        let author = pr_author.login;
        let bot = is_bot(pr_author.on);

        let prim_pr = PrimPr {
            title,
            url,
            author,
            mergeable,
            comments,
            reviews,
            age,
            draft,
            bot,
        };
        prim_prs.push(prim_pr);
    }

    Ok(prim_prs)
}
