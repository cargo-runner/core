use crate::{errors::ConfigError, models::config::CommandDetails};

pub trait ValidateCommandDetails {
    fn validate(&self, details: &CommandDetails) -> Result<(), ConfigError>;
}

pub struct Validator<F>(pub F)
where
    F: Fn(&CommandDetails) -> Result<(), ConfigError> + 'static;

impl<F> ValidateCommandDetails for Validator<F>
where
    F: Fn(&CommandDetails) -> Result<(), ConfigError> + 'static,
{
    fn validate(&self, details: &CommandDetails) -> Result<(), ConfigError> {
        (self.0)(details)
    }
}
