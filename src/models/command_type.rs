use std::fmt;

use serde::{de::{self, Visitor}, Deserialize, Deserializer, Serialize, Serializer};



#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum CommandType {
    #[default]
    Cargo,
    SubCommand,
    Command(String),
}


impl From<CommandType> for String {
    fn from(value: CommandType) -> Self {
        match value {
            CommandType::Cargo => "cargo".to_string(),
            CommandType::SubCommand => "cargo".to_string(),
            CommandType::Command(command) => command.to_string(),
        }
    }
}

impl Serialize for CommandType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CommandType::Cargo => serializer.serialize_str("cargo"),
            CommandType::SubCommand => serializer.serialize_str("subcommand"),
            CommandType::Command(_) => {
                // Serialize as type = "command" and add `command` as a field
                serializer.serialize_str("command")
            }
        }
    }
}

impl<'de> Deserialize<'de> for CommandType {
    fn deserialize<D>(deserializer: D) -> Result<CommandType, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CommandTypeVisitor;

        impl<'de> Visitor<'de> for CommandTypeVisitor {
            type Value = CommandType;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid command type")
            }

            fn visit_str<E>(self, value: &str) -> Result<CommandType, E>
            where
                E: de::Error,
            {
                match value {
                    "cargo" => Ok(CommandType::Cargo),
                    "subcommand" => Ok(CommandType::SubCommand),
                    "command" => Ok(CommandType::Command(String::new())),
                    _ => Err(de::Error::unknown_variant(
                        value,
                        &["cargo", "subcommand", "command"],
                    )),
                }
            }
        }

        deserializer.deserialize_str(CommandTypeVisitor)
    }
}
