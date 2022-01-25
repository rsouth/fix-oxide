// used to refer to a session
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SessionID {
    begin_string: String,
    sender_comp_id: String,
    target_comp_id: String,
}
impl SessionID {
    pub fn from_settings(settings: &Settings) -> SessionID {
        SessionID {
            begin_string: settings.begin_string.clone().into(),
            sender_comp_id: settings.sender_comp_id.to_owned(),
            target_comp_id: settings.target_comp_id.to_owned(),
        }
    }
}

impl From<BeginString> for String {
    fn from(begin_string: BeginString) -> Self {
        match begin_string {
            BeginString::Fix42 => "FIX.4.2".to_string(),
            BeginString::Fix44 => "FIX.4.4".to_string(),
        }
    }
}

#[derive(Copy, Debug, Clone)]
enum BeginString {
    Fix42,
    Fix44,
}

#[derive(Clone, Debug)]
pub struct Settings {
    begin_string: BeginString,
    sender_comp_id: String,
    target_comp_id: String,
    // heartbeat etc config
    session_type: SessionType,
}

#[derive(Debug, Clone, Copy)]
enum SessionType {
    Initiator,
    Acceptor,
}

#[cfg(test)]
mod tests {
    use crate::engine::{Engine, State};
    use crate::session::{BeginString, SessionType, Settings};

    #[test]
    fn it_works() {
        let mut engine = Engine::default();

        let settings = Settings {
            begin_string: BeginString::Fix42,
            sender_comp_id: "Rocks".to_string(),
            target_comp_id: "Exchange".to_string(),
            session_type: SessionType::Initiator,
        };

        let session_id = engine.create_session(settings).unwrap().to_owned();
        assert_eq!(1, engine.sessions().len());

        let settings = Settings {
            begin_string: BeginString::Fix42,
            sender_comp_id: "Rocks".to_string(),
            target_comp_id: "Exchange".to_string(),
            session_type: SessionType::Initiator,
        };
        assert!(engine.create_session(settings).is_err());
        assert_eq!(1, engine.sessions().len());

        // let mut engine = engine;
        // let engine = engine;
        assert_eq!(&State::Created, engine.session_status(&session_id).unwrap());
    }
}
