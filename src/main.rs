mod api;
mod config;
mod template;

use std::env;
use std::path::PathBuf;

use anyhow::{bail, Result};
use inflections::case::is_kebab_case;

use config::{CONFIG_PATH, update_config};

const BASE_DIR: &str = "./blog-posts";

fn get_title() -> Result<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        bail!("Usage: new.rs <title>");
    }

    if !is_kebab_case(args[1].as_str()) {
        bail!("Title must be in kebab-case");
    }

    Ok(args[1].clone())
}

fn main() -> Result<()> {
    let title = get_title()?;
    let article_file: PathBuf = [BASE_DIR, title.as_str(), format!("{}.md", title).as_str()].iter().collect();

    if article_file.exists() {
        bail!("Article already exists");
    }

    let article_id = api::create_draft(title.clone())?;
    println!("Created an article with id: {article_id}");

    update_config(article_id, article_file.clone())?;
    println!("Updated config file: {}", CONFIG_PATH);

    template::generate(title, article_file.clone())?;
    println!("Generated template file: {:?}", article_file);

    Ok(())
}
