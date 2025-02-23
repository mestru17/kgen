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
pub struct GeneratedDir {
    path: PathBuf,
}

impl GeneratedDir {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn generate(&self) -> anyhow::Result<()> {
        fs::create_dir_all(&self.path).with_context(|| {
            format!(
                "Failed to create directory {:?} or one of its parent directories",
                self.path
            )
        })?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct GeneratedProject {
    dir: GeneratedDir,
    dirs: Vec<GeneratedDir>,
    files: Vec<GeneratedFile>,
}

impl GeneratedProject {
    pub fn new(path: GeneratedDir) -> Self {
        Self {
            dir: path,
            dirs: Vec::new(),
            files: Vec::new(),
        }
    }

    pub fn dir(&mut self, dir: GeneratedDir) -> &mut Self {
        let dir = GeneratedDir::new(self.dir.path.join(dir.path));
        self.dirs.push(dir);
        self
    }

    pub fn file(&mut self, file: GeneratedFile) -> &mut Self {
        let file = GeneratedFile::new(self.dir.path.join(file.path), file.content);
        self.files.push(file);
        self
    }

    pub fn generate(&self) -> anyhow::Result<()> {
        if self.dir.path.exists() {
            bail!("Project {:?} already exists", self.dir.path);
        }

        self.dir
            .generate()
            .with_context(|| format!("Failed to generate project directory {:?}", self.dir))?;

        for dir in &self.dirs {
            dir.generate().with_context(|| {
                format!("Failed to generate dir {:?} in project {:?}", dir, self.dir)
            })?;
        }

        for file in &self.files {
            file.generate().with_context(|| {
                format!(
                    "Failed to generate file {:?} in project {:?}",
                    file, self.dir
                )
            })?;
        }

        Ok(())
    }
}
