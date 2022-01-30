use std::fmt;

use crate::model::field::FieldSet;
use crate::model::generated::generated::MsgTypeField;

#[derive(Debug, Clone)]
pub struct UnknownMsgTypeError {
    pub val: String,
}

impl fmt::Display for UnknownMsgTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unknown MsgType value: {}", self.val)
    }
}

impl TryFrom<&FieldSet> for MsgTypeField {
    type Error = ();

    fn try_from(fs: &FieldSet) -> Result<Self, Self::Error> {
        match fs.get_field(Self::tag()) {
            Ok(a) => Ok(Self { fd: a.clone() }),
            Err(_) => Err(()),
        }
    }
}
