use std::collections::HashMap;

use serde::{Deserialize, Serialize, Serializer};

use super::CommandType;

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub struct CommandConfig {
    pub name: String,  // Change from &'a str to String
    #[serde(serialize_with = "serialize_command_type")]
    pub command_type: CommandType, // Change from CommandType<'a> to CommandType
    pub command: String,  // Change from &'a str to String
    pub sub_command: String,  // Change from &'a str to String
    pub allowed_subcommands: Vec<String>,  // Change from Vec<&'a str> to Vec<String>
    pub env: HashMap<String, String>,  // Change from HashMap<&'a str, &'a str> to HashMap<String, String>
}


fn serialize_command_type<S>(cmd_type: &CommandType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match *cmd_type {
        CommandType::Cargo => serializer.serialize_str("cargo"),
        CommandType::Shell(_) => serializer.serialize_str("shell"),
        CommandType::SubCommand => serializer.serialize_str("subcommand"),
    }
}