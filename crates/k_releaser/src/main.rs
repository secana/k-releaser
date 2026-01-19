mod args;
mod changelog_config;
mod config;
mod config_show;
mod log;

use args::OutputType;
use clap::Parser;
use k_releaser_core::ReleaseRequest;
use serde::Serialize;
use tracing::error;

use crate::args::{CliArgs, Command, manifest_command::ManifestCommand as _};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = CliArgs::parse();
    log::init(args.verbosity()?);
    run(args).await.map_err(|e| {
        error!("{:?}", e);
        e
    })?;

    Ok(())
}

async fn run(args: CliArgs) -> anyhow::Result<()> {
    match args.command {
        Command::Update(cmd_args) => {
            let cargo_metadata = cmd_args.cargo_metadata()?;
            let config = cmd_args.load_config()?;
            let update_request = cmd_args.update_request(&config, cargo_metadata)?;
            let (packages_update, _temp_repo) = k_releaser_core::update(&update_request).await?;
            println!("{}", packages_update.summary());
        }
        Command::ReleasePr(cmd_args) => {
            let cargo_metadata = cmd_args.update.cargo_metadata()?;
            let config = cmd_args.update.load_config()?;
            let request = cmd_args.release_pr_req(&config, cargo_metadata)?;

            if cmd_args.dry_run {
                // Dry-run mode: calculate what the PR would contain but don't create it
                let dry_run_result = k_releaser_core::release_pr_dry_run(&request).await?;
                println!("=== Dry Run Results ===\n");
                println!("Title: {}\n", dry_run_result.title);
                if let Some(version) = &dry_run_result.version {
                    println!("Version: {}\n", version);
                }
                println!("Body:\n{}\n", dry_run_result.body);
                if !dry_run_result.commits.is_empty() {
                    println!("Commits detected:");
                    for commit in &dry_run_result.commits {
                        println!("  {}", commit);
                    }
                }
            } else {
                anyhow::ensure!(
                    cmd_args.update.git_token.is_some(),
                    "please provide the git token with the --git-token cli argument."
                );
                let release_pr = k_releaser_core::release_pr(&request).await?;
                if let Some(output_type) = cmd_args.output {
                    let prs = match release_pr {
                        Some(pr) => vec![pr],
                        None => vec![],
                    };
                    let prs_json = serde_json::json!({
                        "prs": prs
                    });
                    print_output(output_type, prs_json);
                }
            }
        }
        Command::Publish(cmd_args) => {
            let cargo_metadata = cmd_args.cargo_metadata()?;
            let config = cmd_args.load_config()?;
            let print_order = cmd_args.print_order;
            let cmd_args_output = cmd_args.output;
            let request = cmd_args.publish_request(&config, cargo_metadata)?;

            if print_order {
                let order_output = k_releaser_core::print_publish_order(&request)?;
                if let Some(output_type) = cmd_args_output {
                    print_output(output_type, order_output);
                } else {
                    println!("{}", order_output.display());
                }
            } else {
                let output = k_releaser_core::publish(&request)
                    .await?
                    .unwrap_or_default();
                if let Some(output_type) = cmd_args_output {
                    print_output(output_type, output);
                }
            }
        }
        Command::Release(cmd_args) => {
            let cargo_metadata = cmd_args.cargo_metadata()?;
            let config = cmd_args.load_config()?;
            let cmd_args_output = cmd_args.output;
            let request: ReleaseRequest = cmd_args.release_request(&config, cargo_metadata)?;
            let output = k_releaser_core::release(&request)
                .await?
                .unwrap_or_default();
            if let Some(output_type) = cmd_args_output {
                print_output(output_type, output);
            }
        }
        Command::Config(cmd) => match cmd.subcommand {
            crate::args::config::ConfigSubcommand::Show(show_args) => {
                config_show::show_config(show_args)?;
            }
        },
    }
    Ok(())
}

fn print_output(output_type: OutputType, output: impl Serialize) {
    match output_type {
        OutputType::Json => match serde_json::to_string(&output) {
            Ok(json) => println!("{json}"),
            Err(e) => tracing::error!("can't serialize release pr to json: {e}"),
        },
    }
}
