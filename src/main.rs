use clap::{App, Arg};
use std::fs;
use std::fs::File;
use std::path::Path;

use git2::Repository;

const GIT_PREVIEW_DIR: &str = "git_preview";

fn main() {
    let matches = App::new("GitSee")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .arg(Arg::with_name("repo link").required(true).index(1))
        .about("Preview a git repo without .git/ attached")
        .get_matches();

    let repo_link = matches.value_of("repo link").unwrap();
    let repo_clone_url = format!("{}{}", repo_link, ".git");

    let dir = &Path::new(&std::env::var("HOME").unwrap())
        .join(GIT_PREVIEW_DIR)
        .join(parse_file_name(repo_link));

    fs::remove_dir_all(dir).unwrap();
    fs::create_dir_all(dir).unwrap();

    Repository::clone(&repo_clone_url, dir).unwrap();

    fs::remove_dir_all(dir.join(".git")).unwrap();
}

fn parse_file_name(repo_link: &str) -> &str {
    let split = repo_link.split("/").collect::<Vec<&str>>();
    split.last().unwrap()
}
