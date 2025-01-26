use anyhow::Result;
use chrono::{DateTime, Duration, NaiveDateTime, TimeZone, Utc};
use octocrab::Octocrab;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::exit;
mod install;
use install::install_pre_commit_hook;

#[derive(Deserialize)]
struct GitHubEvent {
    #[serde(rename = "type")]
    event_type: String,
    created_at: DateTime<Utc>,
}

fn get_config_path() -> PathBuf {
    let mut config_dir = dirs::home_dir().expect("Could not find home directory");
    config_dir.push(".config");
    config_dir.push("commit-checker");
    fs::create_dir_all(&config_dir).expect("Could not create config directory");
    config_dir.push(".env");
    config_dir
}

#[tokio::main]
async fn main() -> Result<()> {
    // Check if we're in a git repository
    if !std::process::Command::new("git")
        .args(["rev-parse", "--git-dir"])
        .output()?
        .status
        .success()
    {
        println!("Error: Not in a git repository!");
        std::process::exit(1);
    }

    // Load .env from config directory
    let config_path = get_config_path();
    if !config_path.exists() {
        println!("Please create a config file at: {}", config_path.display());
        println!("With the following contents:");
        println!("GITHUB_TOKEN=your_github_token");
        println!("GITHUB_USERNAME=your_github_username");
        println!("START_DATE=YYYY-MM-DD");
        println!("REQUIRED_COMMITS=4");
        std::process::exit(1);
    }

    dotenv::from_path(config_path)?;

    let token = env::var("GITHUB_TOKEN")?;
    let username = env::var("GITHUB_USERNAME")?;
    let start_date_str = env::var("START_DATE")?;

    let octocrab = Octocrab::builder().personal_token(token).build()?;

    // Parse start date
    let start_date = NaiveDateTime::parse_from_str(
        &format!("{} 00:00:00", start_date_str),
        "%Y-%m-%d %H:%M:%S",
    )?;
    let start_date = Utc.from_utc_datetime(&start_date);
    let end_date = Utc::now();

    let events: Vec<GitHubEvent> = octocrab
        .get("/users/".to_string() + &username + "/events", None::<&()>)
        .await?;

    // Create a HashMap to store commit counts per day
    let mut daily_commits: HashMap<String, i32> = HashMap::new();

    // Count commits for each day
    for event in events {
        if event.created_at >= start_date
            && event.created_at <= end_date
            && event.event_type == "PushEvent"
        {
            let date_key = event.created_at.format("%Y-%m-%d").to_string();
            *daily_commits.entry(date_key).or_insert(0) += 1;
        }
    }

    // Check each day from start to now
    let mut current_date = start_date;
    let mut missing_commits = Vec::new();

    let required_commits = env::var("REQUIRED_COMMITS")
        .unwrap_or_else(|_| "4".to_string())
        .parse::<i32>()
        .unwrap_or(4);

    while current_date <= end_date {
        let date_key = current_date.format("%Y-%m-%d").to_string();
        let commit_count = daily_commits.get(&date_key).copied().unwrap_or(0);

        if commit_count < required_commits {
            let commits_needed = required_commits - commit_count;
            missing_commits.push((date_key, commits_needed));
        }

        current_date += Duration::days(1);
    }

    // Print commit messages needed for each day
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "install" {
        match install_pre_commit_hook() {
            Ok(_) => {
                println!("✓ Pre-commit hook installed successfully!");
                return Ok(());
            }
            Err(e) => {
                eprintln!("✗ Failed to install pre-commit hook: {}", e);
                exit(1);
            }
        }
    }

    if missing_commits.is_empty() {
        println!("✓ All commit requirements met!");
        exit(0);
    } else {
        eprintln!("✗ Missing commits for previous days!");
        eprintln!("\nRequired commits for each day:");
        for (date, needed) in missing_commits {
            eprintln!("\n📅 {}: {} more commits needed", date, needed);
            eprintln!("Copy and use these exact commands:");
            for i in 0..needed {
                let time = format!("{} {:02}:00:00", date, 12 + i);
                eprintln!(
                    "\ngit commit --date=\"{}\" -m \"feat: your commit message here\"",
                    time
                );
            }
        }
        exit(1);
    }
}
