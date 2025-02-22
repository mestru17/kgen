mod cli;
mod config;
mod generate;
mod templates;

use anyhow::Context;
use askama::Template;
use clap::Parser;
use cli::Cli;
use config::Config;
use console::style;
use generate::{GeneratedDir, GeneratedFile, GeneratedProject};
use templates::{MainTemplate, MainTestTemplate, PomTemplate, ReadmeTemplate};

const NAME: &str = env!("CARGO_PKG_NAME");

fn main() -> anyhow::Result<()> {
    let project_settings = Cli::parse().prompt_for_missing_values()?;

    let config: Config = confy::load(NAME, Some("config")).context("Failed to load config")?;

    let main = MainTemplate {
        package: &project_settings.package,
    };

    let main_test = MainTestTemplate {
        package: &project_settings.package,
    };

    let env_template = include_str!("../static/.env.template").trim_end();

    let git_ignore = include_str!("../static/.gitignore").trim_end();

    let application_yml = include_str!("../static/application.yml").trim_end();

    let docker_compose = include_str!("../static/docker-compose.yml").trim_end();

    let dockerfile = include_str!("../static/Dockerfile").trim_end();

    let pom = PomTemplate {
        group_id: &project_settings.group_id,
        artifact_id: &project_settings.artifact_id,
        main_class: &format!("{}.MainKt", project_settings.package),
        repositories: &config.repositories,
    };

    let readme = ReadmeTemplate {
        title: &project_settings.artifact_id,
    };

    let settings_xml_template = include_str!("../static/settings.xml.template").trim_end();

    GeneratedProject::new(GeneratedDir::new(&project_settings.artifact_id))
        .dir(GeneratedDir::new("src/main/resources"))
        .dir(GeneratedDir::new("src/test/resources"))
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
        .with_context(|| {
            format!(
                "Failed to generate project {}",
                project_settings.artifact_id
            )
        })?;

    eprintln!("{}", style("✔ Generated project").for_stderr().green());

    Ok(())
}
