use std::net::SocketAddr;

use clap::Parser;

//
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long)]
    pub http_listen_addr: SocketAddr,
    #[arg(long, short = 'v')]
    pub verbose: bool,
    #[arg(long)]
    pub backend_max_response_body_size: Option<usize>,
}

impl Args {
    pub fn parse() -> Self {
        Parser::parse()
    }

    pub fn backend_max_response_body_size(&self) -> usize {
        self.backend_max_response_body_size.unwrap_or(1024 * 1024)
    }
}
