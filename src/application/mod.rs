use crate::model::message::{Logon, Message};

trait FixApp {
    fn on_logon(&self, message: Logon);

    fn on_message(&self, message: dyn Message);
}