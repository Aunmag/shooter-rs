use clap::Parser;

#[derive(Debug, Parser)]
pub struct Arguments {
    /// Host a server
    #[arg(long)]
    pub host: bool,
    /// Join a server like a client
    #[arg(long)]
    pub join: bool,
    /// Path to the configuration file
    #[arg(value_parser, short, long, default_value_t = String::from("./config.toml"))]
    pub config: String,
}
