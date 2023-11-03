use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    Add,
    Remove,
    Complete,
}

impl FromStr for Action {
    type Err = ();

    fn from_str(str: &str) -> Result<Action, Self::Err> {
        match str {
            "add"       => Ok(Action::Add),
            "a"         => Ok(Action::Add),
            "remove"    => Ok(Action::Remove),
            "r"         => Ok(Action::Remove),
            "complete"  => Ok(Action::Complete),
            "c"         => Ok(Action::Complete),
            _           => Err(()),
        }
    }
}
