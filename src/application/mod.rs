use std::any::Any;

use crate::model::message::Message;

pub trait FixApp {
    fn as_any(&self) -> &dyn Any; // todo no no no

    // todo engine should handle these messages; FixApp should just be there for
    //  application to implement to ge tthe msgs.

    fn on_message(&mut self, message: Message) {
        println!("Received {}", message);
        match message.get_field(35).as_str() {
            "A" => {
                self.on_logon(message);
            }
            _ => todo!("Handle unexpected message types"),
        }
    }

    fn on_logon(&self, message: Message) {}
}
