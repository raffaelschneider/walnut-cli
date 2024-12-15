use clap::{Arg, ArgAction, ArgMatches, Command, Subcommand};
use std::env;
use std::path::{Path, PathBuf};

mod config;
mod repo;
mod new;

use config::{load_config, Config};
use repo::{setup_repository, list_repositories};
use new::{create_journal_entry, create_note_entry};

#[derive(Subcommand)]
enum WalnutCmd {
    /// Set up a new Walnut repository
    Setup {
        /// Name of the walnut repository
        #[arg(short, long)]
        name: Option<String>,
        /// Location of the walnut repository
        #[arg(short, long)]
        location: Option<PathBuf>,
    },

    /// Create a new entry (journal by default)
    New {
        /// The type of entry to create, defaults to "journal" if not specified
        #[arg(value_parser = ["journal", "note"], default_value = "journal")]
        entry_type: String,

        /// Specify which Walnut repository to use
        #[arg(short, long)]
        walnut: Option<String>,
    },
}

fn main() {
    let cmd = Command::new("walnut")
        .alias("wat")
        .version("0.1.0")
        .about("Walnut Personal Knowledge Management CLI")
        .subcommand_required(true)
        .subcommand(
            Command::new("setup")
                .about("Set up a new Walnut repository")
                .arg(Arg::new("name").short('n').long("name").help("Name of the Walnut repository").value_name("NAME"))
                .arg(Arg::new("location").short('l').long("location").help("Location of the Walnut repository").value_name("PATH"))
        )
        .subcommand(
            Command::new("new")
                .about("Create a new entry")
                .arg(Arg::new("entry_type")
                     .value_parser(["journal", "note"])
                     .default_value("journal"))
                .arg(Arg::new("walnut").short('w').long("walnut").help("Specify which Walnut repository to use").value_name("NAME"))
        );

    let matches = cmd.get_matches();

    // Load configuration at every invocation
    let config = match load_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Warning: Could not load config: {e}");
            Config::default()
        }
    };

    match matches.subcommand() {
        Some(("setup", sub_m)) => handle_setup(sub_m, &config),
        Some(("new", sub_m)) => handle_new(sub_m, &config),
        _ => {}
    }
}

fn handle_setup(matches: &ArgMatches, config: &Config) {
    let name = matches.get_one::<String>("name").cloned();
    let location = matches.get_one::<PathBuf>("location").cloned();
    match setup_repository(name, location, config) {
        Ok(_) => println!("Walnut repository set up successfully."),
        Err(e) => eprintln!("Error setting up repository: {e}"),
    }
}

fn handle_new(matches: &ArgMatches, config: &Config) {
    let entry_type = matches.get_one::<String>("entry_type").unwrap(); 
    let walnut = matches.get_one::<String>("walnut").cloned();

    // Determine which repository to use
    let repos = match list_repositories(config) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Could not load repositories: {e}");
            return;
        }
    };

    if repos.is_empty() {
        eprintln!("No repositories found. Please run `walnut setup` first.");
        return;
    }

    let target_repo = if let Some(name) = walnut {
        // For demonstration: we assume the 'name' is actually the directory name.
        // In a real scenario, you might store name->path mappings.
        let found = repos.iter().find(|r| {
            if let Some(x) = r.file_name() {
                x.to_str() == Some(name.as_str())
            } else {
                false
            }
        });

        match found {
            Some(r) => r.to_path_buf(),
            None => {
                eprintln!("Specified walnut repository not found.");
                return;
            }
        }
    } else {
        // Default to the first repository
        repos[0].clone()
    };

    // Now create the new entry based on type
    match entry_type.as_str() {
        "journal" => {
            if let Err(e) = create_journal_entry(&target_repo) {
                eprintln!("Error creating journal entry: {e}");
            } else {
                println!("New journal entry created.");
            }
        }
        "note" => {
            if let Err(e) = create_note_entry(&target_repo) {
                eprintln!("Error creating note entry: {e}");
            } else {
                println!("New note entry created.");
            }
        }
        _ => unreachable!(), 
    }
}

