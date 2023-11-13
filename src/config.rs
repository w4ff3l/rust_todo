use std::{str::FromStr, path::PathBuf};

use crate::action::Action;

#[derive(Debug, PartialEq)]
pub struct Config {
    pub action: Action,
    pub action_parameters: Vec<String>,
    pub file_directory: PathBuf,
}

impl Config {
    pub fn build(file_directory: PathBuf, arguments: &[String]) -> Result<Config, &'static str> {
        if arguments.len() < 2 {
            return Err("No arguments. Nothing to do.");
        }

        let action = Action::from_str(arguments[1].as_str())?;
        let action_parameters = arguments[1..arguments.len()].to_vec();

        Ok(Config {
            action,
            action_parameters,
            file_directory,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::action::Action;

    use super::Config;

    #[test]
    fn builds() {
        let arguments = ["w/e".to_string(), "add".to_string()];
        let config = Config::build(PathBuf::from("w/e"), &arguments).unwrap();

        assert_eq!(Action::Add, config.action);
        assert_eq!(1, config.action_parameters.len());
    }

    #[test]
    fn errors_with_too_few_arguments() {
        let arguments = ["w/e".to_string()];
        let config = Config::build(PathBuf::from("w/e"), &arguments);

        assert!(config.is_err());
        assert_eq!("No arguments. Nothing to do.", config.unwrap_err());
    }

    #[test]
    fn error_with_unparsable_action() {
        let arguments = ["w/e".to_string(), "unknown action".to_string()];
        let config = Config::build(PathBuf::from("w/e"), &arguments);

        assert!(config.is_err());
        assert_eq!("Unknown action.", config.unwrap_err());
    }
}
