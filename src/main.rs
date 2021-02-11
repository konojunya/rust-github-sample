mod github;
mod shell;

use github::GitHub;

#[tokio::main]
async fn main() {

    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");
    let github = GitHub::new(token);
    
    match github.get_issue_list("rust-lang", "rust").await {
        Ok(page) => {
            for issue in page {
                println!("#{}: {}",issue.number, issue.title);
            }
        },
        Err(e) => println!("error: {}", e),
    }

    let origin = shell::get_current_origin();

    match github.create_pull_request(&origin.0, &origin.1).await {
        Ok(result) => println!("{:?}", result),
        Err(e) => println!("error: {}", e),
    }

}
