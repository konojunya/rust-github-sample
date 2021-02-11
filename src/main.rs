use octocrab::{models, params};

async fn get_issue_list(
    owner: &str,
    repo: &str,
) -> std::result::Result<octocrab::Page<models::issues::Issue>, octocrab::Error> {
    let octocrab = octocrab::instance();
    
    octocrab
        .issues(owner, repo)
        .list()
        .state(params::State::Open)
        .per_page(50)
        .send()
        .await
}

#[tokio::main]
async fn main() {
    match get_issue_list("rust-lang", "rust").await {
        Ok(page) => {
            for issue in page {
                println!("{}", issue.title);
            }
        },
        Err(e) => println!("error: {}", e),
    }
}
