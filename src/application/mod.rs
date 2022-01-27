use std::any::Any;

use crate::model::message::Message;

pub trait FixApp {
    fn as_any(&self) -> &dyn Any; // todo no no no

    fn on_message(&mut self, message: Message);
}
