use std::net::SocketAddr;

use clap::Parser;

//
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long)]
    pub http_listen_addr: SocketAddr,
    #[arg(long, short = 'v')]
    pub verbose: bool,
}

impl Args {
    pub fn parse() -> Self {
        Parser::parse()
    }
}
