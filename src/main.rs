use askama::Template;
use clap::Parser;

/// Generate Kotlin projects with standard setup
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Maven groupId to use for the project
    #[arg(short, long)]
    group_id: String,

    /// Maven artifactId to use for the project
    #[arg(short, long)]
    artifact_id: String,

    /// Base package to generate
    #[arg(short, long)]
    package: String,
}

#[derive(Template)]
#[template(path = "Main.kt", escape = "none")]
struct MainTemplate<'a> {
    package: &'a str,
}

#[derive(Template)]
#[template(path = "MainTest.kt", escape = "none")]
struct MainTestTemplate<'a> {
    package: &'a str,
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

    let main = MainTemplate {
        package: &cli.package,
    };
    println!("{}", main.render()?);

    let main_test = MainTestTemplate {
        package: &cli.package,
    };

    let pom = PomTemplate {
        group_id: &cli.group_id,
        artifact_id: &cli.artifact_id,
        main_class: &format!("{}.MainKt", cli.package),
    };
    println!("{}", pom.render()?);

    let env_template = include_str!("../static/.env.template");
    print!("{}", env_template);

    Ok(())
}
