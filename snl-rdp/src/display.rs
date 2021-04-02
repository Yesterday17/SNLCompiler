use std::fmt;
use std::fmt::{Display, Formatter};
use crate::models::*;

// TODO
impl Display for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ProK")?;
        write!(f, "  PheadK {}", self.name)?;
        {
            let t = &self.declare.type_declare;
            if t.len() > 0 {
                write!(f, "  TypeK")?;
            }
        }
        Ok(())
    }
}