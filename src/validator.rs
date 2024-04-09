use crate::{errors::ConfigError, models::config::CommandDetails};

pub trait ValidateCommandDetails {
    fn validate(&self, details: &CommandDetails) -> Result<(), ConfigError>;
}

pub struct Validator<F>(pub F)
where
    F: for<'a> Fn(&'a CommandDetails) -> Result<(), ConfigError>;

impl<F> ValidateCommandDetails for Validator<F>
where
    F: for<'a> Fn(&'a CommandDetails) -> Result<(), ConfigError>,
{
    fn validate(&self, details: &CommandDetails) -> Result<(), ConfigError> {
        (self.0)(details)
    }
}
