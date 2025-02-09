use clap::Parser;

/// Generate Kotlin projects with standard setup
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Maven groupId to use for the project
    #[arg(short, long)]
    pub group_id: String,

    /// Maven artifactId to use for the project
    #[arg(short, long)]
    pub artifact_id: String,

    /// Base package to generate
    #[arg(short, long)]
    pub package: String,
}
