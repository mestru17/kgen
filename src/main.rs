mod cli;
mod output;
mod templates;

use askama::Template;
use clap::Parser;
use cli::Cli;
use output::{FilePreview, GeneratedFile};
use templates::{MainTemplate, MainTestTemplate, PomTemplate, ReadmeTemplate};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let main = MainTemplate {
        package: &cli.package,
    };

    let main_test = MainTestTemplate {
        package: &cli.package,
    };

    let env_template = include_str!("../static/.env.template").trim_end();

    let git_ignore = include_str!("../static/.gitignore").trim_end();

    let application_yml = include_str!("../static/application.yml").trim_end();

    let docker_compose = include_str!("../static/docker-compose.yml").trim_end();

    let dockerfile = include_str!("../static/Dockerfile").trim_end();

    let pom = PomTemplate {
        group_id: &cli.group_id,
        artifact_id: &cli.artifact_id,
        main_class: &format!("{}.MainKt", cli.package),
    };

    let readme = ReadmeTemplate {
        title: &cli.artifact_id,
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
        .with_file(GeneratedFile::new(".gitignore", "", git_ignore))
        .with_file(GeneratedFile::new(
            "application.yml",
            "yaml",
            application_yml,
        ))
        .with_file(GeneratedFile::new(
            "docker-compose.yml",
            "yaml",
            docker_compose,
        ))
        .with_file(GeneratedFile::new("Dockerfile", "dockerfile", dockerfile))
        .with_file(GeneratedFile::new("pom.xml", "xml", pom.render()?))
        .with_file(GeneratedFile::new(
            "README.md",
            "markdown",
            readme.render()?,
        ));

    preview.display();

    Ok(())
}
