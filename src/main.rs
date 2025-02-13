mod cli;
mod output;
mod templates;

use askama::Template;
use clap::Parser;
use cli::Cli;
use output::{FilePreview, GeneratedFile};
use templates::{MainTemplate, MainTestTemplate, PomTemplate};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let main = MainTemplate {
        package: &cli.package,
    };

    let main_test = MainTestTemplate {
        package: &cli.package,
    };

    let env_template = include_str!("../static/.env.template").trim_end();

    let pom = PomTemplate {
        group_id: &cli.group_id,
        artifact_id: &cli.artifact_id,
        main_class: &format!("{}.MainKt", cli.package),
    };

    let preview = FilePreview::new()
        .with_file(GeneratedFile::new("Main.kt", "kotlin", main.render()?))
        .with_file(GeneratedFile::new(
            "MainTest.kt",
            "kotlin",
            main_test.render()?,
        ))
        .with_file(GeneratedFile::new(
            ".env.template",
            "properties",
            env_template,
        ))
        .with_file(GeneratedFile::new("pom.xml", "xml", pom.render()?));

    preview.display();

    Ok(())
}
