use std::collections::{BTreeSet, HashMap};

use serde::{ser::SerializeMap, Deserialize, Serialize, Serializer};

use super::CommandType;


#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub struct CommandDetails {
    #[serde(rename = "type", default = "default_command_type")]
    pub command_type: CommandType,
    #[serde(serialize_with = "serialize_env")]
    pub env: HashMap<String, String>,
    #[serde(default = "default_command")]
    pub command: String,
    #[serde(default)]
    pub toolchain: Option<String>,
    #[serde(default = "default_sub_command")]
    pub sub_command: String,
    #[serde(default)]
    pub sub_action: String,
    #[serde(default)]
    pub params: String,
    #[serde(default)]
    pub target: Option<String>,
    #[serde(default)]
    pub features: BTreeSet<String>,
    #[serde(default)]
    pub allowed_subcommands: BTreeSet<String>,
}

fn default_command_type() -> CommandType {
    CommandType::Cargo
}

fn default_sub_command() -> String {
    String::from("run")
}

fn default_command() -> String {
    CommandType::Cargo.into()
}



fn serialize_env<S>(env: &HashMap<String, String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut map = serializer.serialize_map(Some(env.len()))?;
    for (k, v) in env {
        if let Ok(bool_val) = v.parse::<bool>() {
            map.serialize_entry(k, &bool_val)?;
        } else if let Ok(int_val) = v.parse::<i64>() {
            map.serialize_entry(k, &int_val)?;
        } else {
            // Fallback to string if it's neither bool nor int
            map.serialize_entry(k, v)?;
        }
    }
    map.end()
}

