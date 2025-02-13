use core::fmt;

#[derive(Debug)]
pub struct GeneratedFile {
    pub name: String,
    pub file_type: String,
    pub contents: String,
}

impl GeneratedFile {
    pub fn new(
        name: impl Into<String>,
        file_type: impl Into<String>,
        contents: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            file_type: file_type.into(),
            contents: contents.into(),
        }
    }
}

#[derive(Debug, Default)]
pub struct FilePreview {
    files: Vec<GeneratedFile>,
}

impl FilePreview {
    pub fn new() -> Self {
        Self { files: Vec::new() }
    }

    pub fn add_file(&mut self, file: GeneratedFile) {
        self.files.push(file);
    }

    pub fn with_file(mut self, file: GeneratedFile) -> Self {
        self.add_file(file);
        self
    }

    pub fn display(&self) {
        println!("{}", self);
    }
}

impl fmt::Display for FilePreview {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "# Generated Files")?;

        for file in &self.files {
            writeln!(f)?;
            writeln!(f, "## {}", file.name)?;
            writeln!(f)?;
            writeln!(f, "```{}", file.file_type)?;
            writeln!(f, "{}", file.contents)?;
            writeln!(f, "```")?;
        }

        Ok(())
    }
}
