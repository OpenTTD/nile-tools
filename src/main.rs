use clap::Parser;
use std::path::Path;

mod blame;
mod language;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    language: String,

    #[clap(short, long)]
    path: Option<String>,
}

fn main() {
    let args = Args::parse();

    let path = Path::new(args.path.as_deref().unwrap_or("."));

    let json = if args.language == "english" {
        language::english(path, &"master".to_string(), true)
    } else {
        language::language(path, &args.language)
    };

    let json = serde_json::to_string_pretty(&json).unwrap();
    println!("{}", json);
}
