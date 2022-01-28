use std::fmt;

use crate::model::field::FieldSet;
use crate::model::twopointoh::MsgType;

#[derive(Debug, Clone)]
pub struct UnknownMsgTypeError {
    pub(crate) val: String,
}

impl fmt::Display for UnknownMsgTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unknown MsgType value: {}", self.val)
    }
}

impl TryFrom<&FieldSet> for MsgType {
    type Error = ();

    fn try_from(fs: &FieldSet) -> Result<Self, Self::Error> {
        match fs.get_field(Self::tag()) {
            Ok(a) => Ok(Self { fd: a.clone() }),
            Err(_) => Err(()),
        }
    }
}
