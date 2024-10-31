use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Copy, Default)]
#[serde(rename_all = "lowercase")]
pub enum CargoContext {
    #[default]
    Run,
    Test,
    Build,
    Bench,
}

impl CargoContext {
    pub fn sub_command(&self) -> String {
        match self {
            CargoContext::Run => "run".to_string(),
            CargoContext::Test => "test".to_string(),
            CargoContext::Build => "build".to_string(),
            CargoContext::Bench => "bench".to_string(),
        }
    }

    pub fn command(&self) -> String {
        String::from("cargo")
    }
}