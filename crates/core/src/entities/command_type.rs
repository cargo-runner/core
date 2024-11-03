use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum CommandType {
    #[default]
    Cargo,
    SubCommand,
    Shell,
}

impl Into<String> for CommandType {
    fn into(self) -> String {
        match self {
            CommandType::Cargo => "cargo".to_string(),
            CommandType::SubCommand => "subcommand".to_string(),
            CommandType::Shell => "shell".to_string(),
        }
    }
}

impl Into<&str> for CommandType {
    fn into(self) -> &'static str {
        match self {
            CommandType::Cargo => "cargo",
            CommandType::SubCommand => "subcommand",
            CommandType::Shell => "shell",
        }
    }
}

impl Serialize for CommandType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match self {
            CommandType::Cargo => "cargo",
            CommandType::SubCommand => "subcommand",
            CommandType::Shell => "shell",
        };
        serializer.serialize_str(s)
    }
}

impl<'de> Deserialize<'de> for CommandType {
    fn deserialize<D>(deserializer: D) -> Result<CommandType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        match s.as_str() {
            "cargo" => Ok(CommandType::Cargo),
            "subcommand" => Ok(CommandType::SubCommand),
            "shell" => Ok(CommandType::Shell),
            _ => Ok(CommandType::Cargo),
        }
    }
}
