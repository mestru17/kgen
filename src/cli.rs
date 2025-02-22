use anyhow::Context;
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Input};

/// Generate Kotlin projects with standard setup
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Maven groupId to use for the project
    #[arg(short, long)]
    pub group_id: Option<String>,

    /// Maven artifactId to use for the project
    #[arg(short, long)]
    pub artifact_id: Option<String>,

    /// Base package to generate
    #[arg(short, long)]
    pub package: Option<String>,
}

#[derive(Debug)]
pub struct ProjectSettings {
    pub group_id: String,
    pub artifact_id: String,
    pub package: String,
}

fn prompt_for_missing_value(
    value: Option<String>,
    theme: &ColorfulTheme,
    value_name: &str,
    initial_text: &str,
) -> anyhow::Result<String> {
    match value {
        Some(value) => Ok(value),
        None => Input::with_theme(theme)
            .with_prompt(value_name)
            .with_initial_text(initial_text)
            .interact_text()
            .with_context(|| format!("Failed to parse {}", value_name)),
    }
}

impl Cli {
    pub fn prompt_for_missing_values(self) -> anyhow::Result<ProjectSettings> {
        let theme = ColorfulTheme::default();

        let group_id = prompt_for_missing_value(self.group_id, &theme, "group-id", "")?;
        let artifact_id = prompt_for_missing_value(self.artifact_id, &theme, "artifact-id", "")?;
        let package = prompt_for_missing_value(self.package, &theme, "package", &group_id)?;

        Ok(ProjectSettings {
            group_id,
            artifact_id,
            package,
        })
    }
}
