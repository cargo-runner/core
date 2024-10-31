use serde::{Deserialize, Serialize};

use super::CommandConfig;

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub struct Config {
    pub run: Vec<CommandConfig>,  // Change from Vec<CommandConfig<'a>> to Vec<CommandConfig>
    pub test: Vec<CommandConfig>,
    pub build: Vec<CommandConfig>,
    pub bench: Vec<CommandConfig>,
}