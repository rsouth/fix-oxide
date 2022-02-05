use crate::model::BeginString;

#[derive(Clone, Debug)]
pub struct ConnectionSettings {
    heartbeat_interval: u16,
    //tcp4,6
}

impl ConnectionSettings {
    fn default_heartbeat_interval() -> u16 {
        5_000
    }
}

#[derive(Clone, Debug)]
pub struct SessionSettings {
    reset_on_logon: bool,
}

impl SessionSettings {
    fn default_reset_on_logon() -> bool {
        false
    }
}

#[derive(Clone, Debug)]
pub struct ScheduleSettings {
    // daily, weekly, specified days
// start time, stop time, timezone
}

#[derive(Clone, Debug)]
pub struct Settings {
    pub begin_string: BeginString,
    pub sender_comp_id: String,
    pub target_comp_id: String,
    pub session_type: SessionType,
    pub session_settings: SessionSettings,
    pub connection_settings: ConnectionSettings,
    pub schedule_settings: ScheduleSettings,
}

pub struct SettingsBuilder {
    begin_string: BeginString,
    sender_comp_id: String,
    target_comp_id: String,
    session_type: SessionType,
    heartbeat_interval_ms: u16,
    reset_on_logon: bool,
}

impl SettingsBuilder {
    pub fn new(
        begin_string: BeginString,
        sender_comp_id: String,
        target_comp_id: String,
        session_type: SessionType,
    ) -> SettingsBuilder {
        SettingsBuilder {
            begin_string,
            sender_comp_id,
            target_comp_id,
            session_type,
            heartbeat_interval_ms: 5_000,
            reset_on_logon: false,
        }
    }

    pub fn with_hb_interval(&mut self, interval: u16) -> &mut Self {
        self.heartbeat_interval_ms = interval;
        self
    }

    pub fn with_reset_on_logon(&mut self, reset_on_logon: bool) -> &mut Self {
        self.reset_on_logon = reset_on_logon;
        self
    }

    pub fn build(&self) -> Settings {
        Settings {
            begin_string: self.begin_string.clone(),
            sender_comp_id: self.sender_comp_id.clone(),
            target_comp_id: self.target_comp_id.clone(),
            session_type: self.session_type.clone(),
            session_settings: SessionSettings {
                reset_on_logon: self.reset_on_logon,
            },
            connection_settings: ConnectionSettings {
                heartbeat_interval: self.heartbeat_interval_ms,
            },
            schedule_settings: ScheduleSettings {},
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SessionType {
    Initiator,
    Acceptor,
}
