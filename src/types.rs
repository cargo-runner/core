use crate::{config::CommandDetails, errors::ConfigError};

pub type CommandDetailsValidation =
    Box<dyn Fn(&CommandDetails) -> Result<(), ConfigError> + 'static>;
