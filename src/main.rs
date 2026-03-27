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

        /// Suppress all output except the actual response body
        #[arg(long)]
        silent: bool,

        /// Force the output to be raw JSON, disabling fancy formatting
        #[arg(long)]
        json: bool,

        /// Print only the response headers
        #[arg(long)]
        headers_only: bool,

        /// Validate parameters and variables without sending the request
        #[arg(long)]
        offline: bool,
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
            silent,
            json: json_flag,
            headers_only,
            offline,
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

            if offline {
                println!("--- OFFLINE MODE ---");
                println!("Method: {:?}", method);
                println!("URL: {}", final_url);
                println!("Headers: {:#?}", final_headers);
                if let Some(b) = final_body {
                    println!("Body:\n{}", b);
                }
                return Ok(());
            }

            let engine = RequestEngine::new();
            let response = engine
                .send(method.into(), &final_url, final_headers, final_body)
                .await?;

            if !silent && !headers_only {
                println!("Status: {}", response.status());
            }

            if headers_only {
                println!("{:#?}", response.headers());
                return Ok(());
            }

            if !silent {
                println!("Headers: {:#?}", response.headers());
            }

            let body_text = response.text().await?;

            if json_flag {
                // If the user requested raw JSON output, we print the raw body.
                // Alternatively, we could attempt to parse and print it, but raw is often better for piping to jq.
                println!("{}", body_text);
            } else if !silent {
                println!("Body:\n{}", body_text);
            } else {
                // Silent mode only prints the body
                print!("{}", body_text);
            }
        }
    }

    Ok(())
}
