use clap::Parser;

#[derive(Debug, Parser)]
pub struct Arguments {
    /// Path to the configuration file
    #[arg(value_parser, short, long, default_value_t = String::from("./config.toml"))]
    pub config: String,
}
