use std::fmt;

pub struct DisplayOption<T: fmt::Display>(pub Option<T>);

impl<T: fmt::Display> fmt::Display for DisplayOption<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.0 {
            Some(x) => write!(f, "{}", x),
            None => Ok(()),
        }
    }
}
