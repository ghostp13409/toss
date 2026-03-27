mod engine;
mod env;

use clap::{Parser, Subcommand, ValueEnum};
use engine::http::RequestEngine;
use env::Environment;
use reqwest::Method as ReqwestMethod;
use std::collections::HashMap;

#[derive(Parser)]
#[command(name = "toss")]
#[command(about = "A Vim-inspired TUI API client", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Send an HTTP request
    Send {
        /// HTTP method (GET, POST, PUT, PATCH, DELETE)
        #[arg(short, long, default_value = "GET")]
        method: Method,

        /// Request URL
        url: String,

        /// Request body (JSON)
        #[arg(short, long)]
        body: Option<String>,

        /// Request headers (Key:Value)
        #[arg(short = 'H', long)]
        header: Vec<String>,

        /// Path to environment file (JSON or YAML)
        #[arg(short, long)]
        env: Option<String>,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[value(rename_all = "UPPERCASE")]
enum Method {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

impl From<Method> for ReqwestMethod {
    fn from(m: Method) -> Self {
        match m {
            Method::Get => ReqwestMethod::GET,
            Method::Post => ReqwestMethod::POST,
            Method::Put => ReqwestMethod::PUT,
            Method::Patch => ReqwestMethod::PATCH,
            Method::Delete => ReqwestMethod::DELETE,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Send {
            method,
            url,
            body,
            header,
            env,
        } => {
            let environment = if let Some(path) = env {
                Environment::from_file(path)?
            } else {
                Environment::default()
            };

            let final_url = environment.replace_vars(&url);
            let final_body = body.map(|b| environment.replace_vars(&b));

            let mut final_headers = HashMap::new();
            for h in header {
                if let Some((key, value)) = h.split_once(':') {
                    final_headers.insert(
                        environment.replace_vars(key.trim()),
                        environment.replace_vars(value.trim()),
                    );
                }
            }

            let engine = RequestEngine::new();
            let response = engine
                .send(method.into(), &final_url, final_headers, final_body)
                .await?;

            println!("Status: {}", response.status());
            println!("Headers: {:#?}", response.headers());
            let body_text = response.text().await?;
            println!("Body:\n{}", body_text);
        }
    }

    Ok(())
}
