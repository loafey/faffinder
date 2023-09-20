#![feature(let_chains)]
use anyhow::Result;
use async_recursion::async_recursion;
use clap::Parser;
use regex::bytes::Regex;
use std::{
    os::unix::prelude::OsStrExt,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::{fs::File, io::AsyncReadExt};

#[derive(Parser, Debug)]
#[command(author = "loafey", version = "0.1", about = "
A tool to get the status of your git repos.
Designed to easily be integrated into prompts.", long_about = None)]
struct Options {
    /// The folder to search from
    #[arg(
        short = 'p',
        long = "path",
        value_name = "DIRECTORY",
        default_value = "."
    )]
    path: PathBuf,
    /// Search content instead of file names
    #[arg(
        short = 'c',
        long = "content",
        value_name = "bool",
        default_value = "false"
    )]
    search_content: bool,
    #[arg(value_name = "STRING", trailing_var_arg = true)]
    search_terms: Vec<String>,
}

#[tokio::main]
async fn main() {
    let options = Options::parse();
    let regex = options
        .search_terms
        .into_iter()
        .map(|s| Regex::new(&s).unwrap())
        .collect::<Vec<_>>();
    worker(options.path, options.search_content, Arc::new(regex)).await;
}

#[async_recursion]
async fn worker(path: PathBuf, search_content: bool, regex: Arc<Vec<Regex>>) {
    let children = /*tokio::task::spawn_blocking({
        let regex = regex.clone();
        move || */{
            let mut children = Vec::new();
            if let Ok(read_dir) = path.read_dir() {
                for file in read_dir.filter_map(|d| d.ok()) {
                    if let Ok(file_type) = file.file_type() {
                        if file_type.is_dir() {
                            children.push(file);
                        } else {
                            let bytes = if !search_content {
                                file.file_name().as_bytes().to_vec()
                            } else {
                                let mut bytes = String::new();
                                if let Ok(mut file) = File::open(file.path()).await{
                                    let _ = file.read_to_string(&mut bytes).await;
                                }
                                bytes.as_bytes().to_vec()
                            }; 
                            for regex in &*regex {
                                if regex.captures(&bytes).is_some() {
                                    let path = format!("{:?}", file.path());
                                    println!("{}", &path[1..path.len() - 1]);
                                    break;
                                }
                            
                        }} 
                    }
                }
            }
            children
        }
    /*})
    .await
    .unwrap() */;
    futures::future::join_all(
        children
            .into_iter()
            .map(move |child| worker(child.path(), search_content, regex.clone())),
    )
    .await;
}
