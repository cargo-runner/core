use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Copy, Default)]
#[serde(rename_all = "lowercase")]
pub enum ContextType {
    #[default]
    Run,
    Test,
    Build,
    Bench,
}

impl ContextType {
    pub fn sub_command(&self) -> String {
        match self {
            ContextType::Run => "run".to_string(),
            ContextType::Test => "test".to_string(),
            ContextType::Build => "build".to_string(),
            ContextType::Bench => "bench".to_string(),
        }
    }

    pub fn command(&self) -> String {
        String::from("cargo")
    }
}