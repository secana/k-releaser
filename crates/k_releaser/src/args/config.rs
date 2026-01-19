use std::path::{Path, PathBuf};

use clap::builder::PathBufValueParser;

use super::{OutputType, config_path::ConfigPath, manifest_command::ManifestCommand};

#[derive(clap::Parser, Debug)]
pub struct Config {
    #[command(subcommand)]
    pub subcommand: ConfigSubcommand,
}

#[derive(clap::Subcommand, Debug)]
pub enum ConfigSubcommand {
    /// Show the current configuration
    Show(ShowConfig),
}

#[derive(clap::Parser, Debug)]
pub struct ShowConfig {
    /// Path to the Cargo.toml of the project.
    /// If not provided, k-releaser will use the Cargo.toml of the current directory.
    #[arg(long, value_parser = PathBufValueParser::new(), alias = "project-manifest")]
    manifest_path: Option<PathBuf>,

    /// Filter to show configuration for a specific package only
    #[arg(long)]
    pub package: Option<String>,

    /// Path to the k-releaser config file.
    #[command(flatten)]
    pub config: ConfigPath,

    /// Output format
    #[arg(short, long, value_enum)]
    pub output: Option<OutputType>,
}

impl ManifestCommand for ShowConfig {
    fn optional_manifest(&self) -> Option<&Path> {
        self.manifest_path.as_deref()
    }
}
