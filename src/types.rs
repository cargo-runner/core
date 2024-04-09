use crate::{errors::ConfigError, models::config::CommandDetails};

pub type CommandDetailsValidation =
    Box<dyn for<'a> Fn(&'a CommandDetails) -> Result<(), ConfigError>>;
