use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod cmd_import;
mod cmd_validate;
mod config;
mod types;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Import {
        language: String,

        #[clap(short, long, default_value = ".")]
        path: PathBuf,
    },

    Validate {
        project: String,
        language: String,

        #[clap(short, long, default_value = "config")]
        config_path: PathBuf,

        #[clap(short, long, default_value = "data")]
        data_path: PathBuf,
    },
}

fn main() {
    let args = Args::parse();

    match args.cmd {
        Commands::Import { language, path } => {
            let json = if language == "english" {
                cmd_import::english(&path, &"master".to_string(), true)
            } else {
                cmd_import::language(&path, &language)
            };

            let json = serde_json::to_string_pretty(&json).unwrap();
            println!("{}", json);
        }
        Commands::Validate {
            config_path,
            data_path,
            project,
            language,
        } => {
            cmd_validate::validate(&config_path, &data_path, &project, &language);
        }
    }
}
