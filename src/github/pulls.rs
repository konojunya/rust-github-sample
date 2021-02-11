use super::GitHub;

impl GitHub {
    pub async fn create_pull_request(&self, owner: &str, repo: &str) -> octocrab::Result<octocrab::models::pulls::PullRequest, octocrab::Error> {
        self.client.pulls(owner, repo).create("issue list with number", "feat/with-issue-number", "main").body("create pull request by rust").send().await
    }
}
