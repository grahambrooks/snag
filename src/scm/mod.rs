use octocrab::Octocrab;
use anyhow::Result;

pub async fn read_repositories(org: &str) -> Result<()> {
    let octocrab = Octocrab::builder().build()?;
    let repos = octocrab
        .orgs(org)
        .list_repos()
        .send()
        .await?;

    for repo in repos {
        println!("Repo: {}", repo.name);
    }

    Ok(())
}