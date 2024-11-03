use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum Context {
    Run,
    Build,
    Test,
    Bench,
    None,
}

impl From<&str> for Context {
    fn from(val: &str) -> Self {
        match val {
            "run" => Context::Run,
            "build" => Context::Build,
            "test" => Context::Test,
            "bench" => Context::Bench,
            _ => Context::None,
        }
    }
}

impl Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Into<String> for Context {
    fn into(self) -> String {
        match self {
            Context::Run => String::from("run"),
            Context::Build => String::from("build"),
            Context::Test => String::from("test"),
            Context::Bench => String::from("bench"),
            Context::None => String::new(),
        }
    }
}

impl Into<&str> for Context {
    fn into(self) -> &'static str {
        match self {
            Context::Run => "run",
            Context::Build => "build",
            Context::Test => "test",
            Context::Bench => "bench",
            Context::None => "",
        }
    }
}
