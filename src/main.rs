use askama::Template;
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

#[derive(Template)]
#[template(path = "pom.xml")]
struct PomTemplate<'a> {
    group_id: &'a str,
    artifact_id: &'a str,
    main_class: &'a str,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    println!("{:#?}", cli);

    let pom = PomTemplate {
        group_id: &cli.group_id,
        artifact_id: &cli.artifact_id,
        main_class: &format!("{}.MainKt", cli.package),
    };
    println!("{}", pom.render()?);

    Ok(())
}
