
// used to refer to a session
pub struct SessionID {

}

enum BeginString {
    Fix42, Fix44
}

pub struct Settings {
    begin_string: BeginString,
    sender_comp_id: String,
    target_comp_id: String,
    // heartbeat etc config
    session_type: SessionType,
}

enum SessionType {
    Initiator, Acceptor
}


#[cfg(test)]
mod tests {
    use crate::engine::Engine;
    use crate::session::{BeginString, SessionType, Settings};

    #[test]
    fn it_works() {
        let mut engine = Engine::default();

        let settings = Settings {
            begin_string: BeginString::Fix42,
            sender_comp_id: "Rocks".to_string(),
            target_comp_id: "Exchange".to_string(),
            session_type: SessionType::Initiator
        };

        let _ = engine.create_session(settings);
        assert_eq!(1, engine.sessions().len());
    }
}

