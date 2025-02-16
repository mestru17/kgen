mod cli;
mod generate;
mod templates;

use anyhow::Context;
use askama::Template;
use clap::Parser;
use cli::Cli;
use generate::{GeneratedDir, GeneratedFile, GeneratedProject};
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

    let settings_xml_template = include_str!("../static/settings.xml.template").trim_end();

    GeneratedProject::new(GeneratedDir::new(&cli.artifact_id))
        .dir(GeneratedDir::new("src/main/kotlin"))
        .dir(GeneratedDir::new("src/test/kotlin"))
        .file(GeneratedFile::new(
            "src/main/kotlin/Main.kt",
            main.render()?,
        ))
        .file(GeneratedFile::new(
            "src/test/kotlin/MainTest.kt",
            main_test.render()?,
        ))
        .file(GeneratedFile::new(".env.template", env_template))
        .file(GeneratedFile::new(".gitignore", git_ignore))
        .file(GeneratedFile::new("application.yml", application_yml))
        .file(GeneratedFile::new("docker-compose.yml", docker_compose))
        .file(GeneratedFile::new("Dockerfile", dockerfile))
        .file(GeneratedFile::new("pom.xml", pom.render()?))
        .file(GeneratedFile::new("README.md", readme.render()?))
        .file(GeneratedFile::new(
            "settings.xml.template",
            settings_xml_template,
        ))
        .generate()
        .with_context(|| format!("Failed to generate project {}", cli.artifact_id))?;

    Ok(())
}
