use std::fmt::{Display, Formatter, Result};
use std::error::Error;

#[derive(Debug, Clone)]
pub struct NodeNotFoundErr;

impl Display for NodeNotFoundErr {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "No node with the specified value exists")
    }
}

impl Error for NodeNotFoundErr {}