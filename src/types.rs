use crate::{errors::ConfigError, models::config::CommandDetails};

pub type CommandDetailsValidation =
    Box<dyn Fn(&CommandDetails) -> Result<(), ConfigError> + 'static>;
