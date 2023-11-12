use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    List,
    Add,
    Remove,
    Complete,
}

impl FromStr for Action {
    type Err = &'static str;

    fn from_str(str: &str) -> Result<Action, Self::Err> {
        match str {
            "add"       => Ok(Action::Add),
            "a"         => Ok(Action::Add),
            "remove"    => Ok(Action::Remove),
            "r"         => Ok(Action::Remove),
            "complete"  => Ok(Action::Complete),
            "c"         => Ok(Action::Complete),
            "list"      => Ok(Action::List),
            "l"         => Ok(Action::List),
            _           => Err("Unknown action."),
        }
    }
}
