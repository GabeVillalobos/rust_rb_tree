use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub struct NodeNotFoundErr;

// No idea if it's standard practice to pull errors into their own module
//  but it made sense to me since the base_tree module is only crate public:
impl Display for NodeNotFoundErr {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "No node with the specified value exists")
    }
}

impl Error for NodeNotFoundErr {}
