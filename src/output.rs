use core::fmt;

#[derive(Debug)]
pub enum FileType {
    Kotlin,
    Xml,
    Properties,
    Other(String),
}

impl FileType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Kotlin => "kotlin",
            Self::Xml => "xml",
            Self::Properties => "properties",
            Self::Other(lang) => lang,
        }
    }

    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "kt" | "kts" => Self::Kotlin,
            "xml" => Self::Xml,
            "properties" | "env" => Self::Properties,
            _ => Self::Other(ext.to_string()),
        }
    }
}

#[derive(Debug)]
pub struct GeneratedFile {
    pub name: String,
    pub file_type: FileType,
    pub contents: String,
}

impl GeneratedFile {
    pub fn from_path(name: impl Into<String>, contents: impl Into<String>) -> Self {
        let name = name.into();
        let extension = name.split(".").last().unwrap_or("").to_string();

        Self {
            name,
            file_type: FileType::from_extension(&extension),
            contents: contents.into(),
        }
    }
}

impl From<String> for FileType {
    fn from(s: String) -> Self {
        Self::Other(s)
    }
}

impl From<&str> for FileType {
    fn from(s: &str) -> Self {
        Self::Other(s.to_string())
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
            writeln!(f, "```{}", file.file_type.as_str())?;
            writeln!(f, "{}", file.contents)?;
            writeln!(f, "```")?;
        }

        Ok(())
    }
}
