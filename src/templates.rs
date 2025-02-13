use askama::Template;

#[derive(Template)]
#[template(path = "Main.kt", escape = "none")]
pub struct MainTemplate<'a> {
    pub package: &'a str,
}

#[derive(Template)]
#[template(path = "MainTest.kt", escape = "none")]
pub struct MainTestTemplate<'a> {
    pub package: &'a str,
}

#[derive(Template)]
#[template(path = "pom.xml")]
pub struct PomTemplate<'a> {
    pub group_id: &'a str,
    pub artifact_id: &'a str,
    pub main_class: &'a str,
}

#[derive(Template)]
#[template(path = "README.md")]
pub struct ReadmeTemplate<'a> {
    pub title: &'a str,
}
