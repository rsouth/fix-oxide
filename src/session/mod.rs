use crate::model::field::NoSuchField;
use crate::model::generated::fields::Field;
use crate::model::BeginString;
use crate::model::BeginString::Fix40;
use regex::Captures;

// used to refer to a session
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SessionID {
    begin_string: String,
    sender_comp_id: String,
    target_comp_id: String,
}

impl SessionID {
    #[must_use]
    pub fn from_settings(settings: &Settings) -> Self {
        Self {
            begin_string: settings.begin_string.into(),
            sender_comp_id: settings.sender_comp_id.clone(),
            target_comp_id: settings.target_comp_id.clone(),
        }
    }
}

pub trait Crack<T> {
    fn crack(s: &str) -> Result<T, NoSuchField>;
}

impl Crack<BeginString> for BeginString {
    fn crack(s: &str) -> Result<BeginString, NoSuchField> {
        let re = regex::Regex::new("^8=(?P<begin_string>.+?)\x01").unwrap();
        match re.captures(s) {
            None => Err(NoSuchField { tag: 8 }),
            Some(caps) => {
                println!("Found {} captures: {:?}", caps.len(), caps);
                let bs = caps.name("begin_string").unwrap().as_str().to_string();
                BeginString::try_from(bs)
            }
        }
    }
}

impl TryFrom<String> for BeginString {
    type Error = NoSuchField;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "FIX.4.0" => Ok(BeginString::Fix40),
            "FIX.4.1" => Ok(BeginString::Fix41),
            "FIX.4.2" => Ok(BeginString::Fix42),
            "FIX.4.3" => Ok(BeginString::Fix43),
            "FIX.4.4" => Ok(BeginString::Fix44),
            "FIX.5.0" => Ok(BeginString::Fix50),
            _ => Err(NoSuchField { tag: 8 }),
        }
    }
}

impl From<BeginString> for String {
    fn from(begin_string: BeginString) -> Self {
        match begin_string {
            BeginString::Fix40 => "FIX.4.0".to_string(),
            BeginString::Fix41 => "FIX.4.1".to_string(),
            BeginString::Fix42 => "FIX.4.2".to_string(),
            BeginString::Fix43 => "FIX.4.3".to_string(),
            BeginString::Fix44 => "FIX.4.4".to_string(),
            BeginString::Fix50 => "FIX.5.0".to_string(),
            BeginString::Fix50Sp1 => "TODO".to_string(),
            BeginString::Fix50Sp2 => "TODO".to_string(),
            BeginString::Fixt11 => "TODO".to_string(),
        }
    }
}

// impl From<BeginString> for String {
//     fn from(begin_string: BeginString) -> Self {
//         match begin_string {
//             BeginString::Fix42 => "FIX.4.2".to_string(),
//             BeginString::Fix44 => "FIX.4.4".to_string(),
//             _ => {}
//         }
//     }
// }

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
    use crate::model::BeginString;
    use crate::session::{SessionType, Settings};

    #[test]
    fn it_works() {
        let mut engine = Engine::default();

        let settings = Settings {
            begin_string: BeginString::Fix42,
            sender_comp_id: "Rocks".to_string(),
            target_comp_id: "Exchange".to_string(),
            session_type: SessionType::Initiator,
        };

        let session_id = engine.create_session(settings).unwrap().clone();
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
