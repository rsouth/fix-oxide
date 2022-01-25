use crate::model::message::{Logon, Message};
use std::any::Any;

pub trait FixApp {
    fn as_any(&self) -> &dyn Any; // todo no no no

    fn on_logon(&mut self, message: Logon);

    fn on_message(&mut self, message: Box<dyn Message>);
}
