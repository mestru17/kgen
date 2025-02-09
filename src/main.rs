use clap::Parser;

/// Generate Kotlin projects with standard setup
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The name of the project
    #[arg(short, long)]
    name: String,

    /// Maven groupId to use for the project
    #[arg(short, long)]
    group_id: String,

    /// Maven artifactId to use for the project
    #[arg(short, long)]
    artifact_id: String,

    /// Base package to use to generate
    #[arg(short, long)]
    package: String,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    println!("{:#?}", cli);

    Ok(())
}
