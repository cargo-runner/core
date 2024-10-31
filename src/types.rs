use crate::{errors::ConfigError, models::CommandDetails};

pub type CommandDetailsValidation =
    Box<dyn Fn(&CommandDetails) -> Result<(), ConfigError> + 'static>;
