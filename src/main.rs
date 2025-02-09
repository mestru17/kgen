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

struct MarkdownSection {
    name: String,
    lang: String,
    contents: String,
}

impl MarkdownSection {
    fn new(name: impl Into<String>, lang: impl Into<String>, contents: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            lang: lang.into(),
            contents: contents.into(),
        }
    }
}

fn print_markdown(sections: &[MarkdownSection]) {
    println!("# Files");
    for section in sections {
        println!();
        println!("## {}", section.name);
        println!();
        println!("```{}", section.lang);
        println!("{}", section.contents);
        println!("```");
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let main = MainTemplate {
        package: &cli.package,
    };

    let main_test = MainTestTemplate {
        package: &cli.package,
    };

    let pom = PomTemplate {
        group_id: &cli.group_id,
        artifact_id: &cli.artifact_id,
        main_class: &format!("{}.MainKt", cli.package),
    };

    let env_template = include_str!("../static/.env.template").trim_end();

    let sections = vec![
        MarkdownSection::new("Main.kt", "kotlin", main.render()?),
        MarkdownSection::new("MainTest.kt", "kotlin", main_test.render()?),
        MarkdownSection::new("pom.xml", "xml", pom.render()?),
        MarkdownSection::new(".env.template", "properties", env_template),
    ];

    print_markdown(&sections);

    Ok(())
}
