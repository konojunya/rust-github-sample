use super::GitHub;
use octocrab::{models, params};

impl GitHub {
    pub async fn get_issue_list(
        &self,
        owner: &str,
        repo: &str,
    ) -> std::result::Result<octocrab::Page<models::issues::Issue>, octocrab::Error> {
        self.client
            .issues(owner, repo)
            .list()
            .state(params::State::Open)
            .per_page(50)
            .send()
            .await
    }
}
