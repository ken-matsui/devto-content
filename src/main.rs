mod api;
mod config;
mod template;

use std::path::PathBuf;

use anyhow::{bail, Result};
use clap::{Parser, Subcommand};
use inflections::case::is_kebab_case;

use config::{update_config, CONFIG_PATH};

const BASE_DIR: &str = "./blog-posts";

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,

    /// dev.to token
    #[clap(short, long, env = "DEVTO_TOKEN")]
    pub devto_token: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new article
    New { title: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { title } => {
            if !is_kebab_case(title) {
                bail!("Title must be in kebab-case");
            }

            let article_file: PathBuf = [BASE_DIR, title, format!("{}.md", title).as_str()]
                .iter()
                .collect();
            if article_file.exists() {
                bail!("Article already exists");
            }

            let article_id = api::create_draft(title, cli.devto_token)?;
            println!("Created an article with id: {article_id}");

            update_config(article_id, article_file.clone())?;
            println!("Updated config file: {CONFIG_PATH}");

            template::generate(title, article_file.clone())?;
            println!("Generated template file: {:?}", article_file);

            Ok(())
        }
    }
}
