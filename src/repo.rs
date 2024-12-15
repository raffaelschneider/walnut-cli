use std::fs::{self, File, OpenOptions};
use std::io::{Write, BufRead, BufReader};
use std::path::{Path, PathBuf};
use chrono::Local;

use crate::config::{config_dir, data_dir, Config};

pub fn setup_repository(name: Option<String>, location: Option<PathBuf>, _config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    // Determine repository location
    let base_dir = location.unwrap_or_else(|| data_dir().join("walnut_repos"));

    // If name isn't specified, create a name based on date/time or some default
    let repo_name = name.unwrap_or_else(|| {
        let date_str = Local::now().format("%Y%m%d%H%M%S").to_string();
        format!("walnut_{}", date_str)
    });

    let repo_path = base_dir.join(&repo_name);

    // Create directory structure according to your Walnut schema
    // For simplicity, we create top-level dirs
    fs::create_dir_all(repo_path.join("Journal"))?;
    fs::create_dir_all(repo_path.join("Notes/Personal"))?;
    fs::create_dir_all(repo_path.join("Notes/Projects"))?;
    fs::create_dir_all(repo_path.join("Notes/Work"))?;
    fs::create_dir_all(repo_path.join("References/Articles"))?;
    fs::create_dir_all(repo_path.join("References/Bibliography"))?;
    fs::create_dir_all(repo_path.join("References/Books"))?;
    fs::create_dir_all(repo_path.join("References/Contacts"))?;
    fs::create_dir_all(repo_path.join("References/Glossary"))?;
    fs::create_dir_all(repo_path.join("References/Media"))?;
    fs::create_dir_all(repo_path.join("References/Papers"))?;
    fs::create_dir_all(repo_path.join("References/Podcasts"))?;
    fs::create_dir_all(repo_path.join("References/WebContent"))?;
    fs::create_dir_all(repo_path.join("Templates/Journal"))?;
    fs::create_dir_all(repo_path.join("Templates/Notes"))?;
    fs::create_dir_all(repo_path.join("Templates/References/Bibliography"))?;

    // Create a walnut.yml file
    let walnut_yml = repo_path.join("walnut.yml");
    let mut f = File::create(&walnut_yml)?;
    writeln!(f, "# Walnut configuration file")?;

    // Add repository to the global state
    add_repository(&repo_path)?;

    Ok(())
}

pub fn add_repository(repo_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let repos_file = config_dir().join("repositories");
    let mut f = OpenOptions::new().create(true).append(true).open(&repos_file)?;
    writeln!(f, "{}", repo_path.display())?;
    Ok(())
}

pub fn list_repositories(_config: &Config) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let repos_file = config_dir().join("repositories");
    if !repos_file.exists() {
        return Ok(vec![]);
    }
    let file = File::open(&repos_file)?;
    let reader = BufReader::new(file);
    let mut repos = vec![];
    for line in reader.lines() {
        let l = line?;
        if !l.trim().is_empty() {
            repos.push(PathBuf::from(l));
        }
    }
    Ok(repos)
}

