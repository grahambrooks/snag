use std::env::var;
use octocrab::Octocrab;
use anyhow::Result;

pub async fn read_repositories(name: &str) -> Result<()> {
    log::info!("Reading repositories for {}", name);
    let builder = Octocrab::builder();
    let token = var("GITHUB_TOKEN");

    let gh = match token {
        Ok(t) => builder.personal_token(t).build()?,
        Err(_) => builder.build()?,
    };

    let repos = if gh.orgs(name).get().await.is_ok() {
        gh.orgs(name).list_repos().send().await?
    } else {
        gh.users(name).repos().send().await?
    };
    for repo in repos {
        println!("Repo: {}", repo.name);
    }
    Ok(())
}