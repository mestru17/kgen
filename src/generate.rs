use std::{fs, path::PathBuf};

use anyhow::{bail, Context};

#[derive(Debug)]
pub struct GeneratedFile {
    path: PathBuf,
    content: String,
}

impl GeneratedFile {
    pub fn new(path: impl Into<PathBuf>, content: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            content: content.into(),
        }
    }

    pub fn generate(&self) -> anyhow::Result<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!(
                    "Failed to create parent directories of file {:?}",
                    self.path
                )
            })?;
        }

        fs::write(&self.path, &self.content)
            .with_context(|| format!("Failed to write content to file {:?}", self.path))?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct GeneratedDir(PathBuf);

impl GeneratedDir {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self(path.into())
    }

    pub fn generate(&self) -> anyhow::Result<()> {
        fs::create_dir_all(&self.0).with_context(|| {
            format!(
                "Failed to create directory {:?} or one of its parent directories",
                self.0
            )
        })?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct GeneratedProject {
    path: GeneratedDir,
    dirs: Vec<GeneratedDir>,
    files: Vec<GeneratedFile>,
}

impl GeneratedProject {
    pub fn new(path: GeneratedDir) -> Self {
        Self {
            path,
            dirs: Vec::new(),
            files: Vec::new(),
        }
    }

    pub fn dir(&mut self, dir: GeneratedDir) -> &mut Self {
        let dir = GeneratedDir::new(self.path.0.join(dir.0));
        self.dirs.push(dir);
        self
    }

    pub fn file(&mut self, file: GeneratedFile) -> &mut Self {
        let file = GeneratedFile::new(self.path.0.join(file.path), file.content);
        self.files.push(file);
        self
    }

    pub fn generate(&self) -> anyhow::Result<()> {
        if self.path.0.exists() {
            bail!("Project {:?} already exists", self.path.0);
        }

        self.path
            .generate()
            .with_context(|| format!("Failed to generate project directory {:?}", self.path))?;

        for dir in &self.dirs {
            dir.generate().with_context(|| {
                format!(
                    "Failed to generate dir {:?} in project {:?}",
                    dir, self.path
                )
            })?;
        }

        for file in &self.files {
            file.generate().with_context(|| {
                format!(
                    "Failed to generate file {:?} in project {:?}",
                    file, self.path
                )
            })?;
        }

        Ok(())
    }
}
