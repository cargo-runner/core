use std::borrow::Cow;

use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum CommandType {
    #[default]
    Cargo,
    SubCommand,
    Shell(Cow<'static, str>), // Changed Command to Shell
}

impl Serialize for CommandType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match self {
            CommandType::Cargo => "cargo",
            CommandType::SubCommand => "subcommand",
            CommandType::Shell(ref shell) => shell.as_ref(),
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
            _ => Ok(CommandType::Shell(Cow::Owned(s))),
        }
    }
}