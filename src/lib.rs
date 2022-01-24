#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

pub mod model;

// #[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
// struct SessionID {}
//
// #[allow(dead_code)]
// struct Session {
//     pub session_id: SessionID,
// }
//
// trait FIXApp {
//     fn on_message(&self, message: Box<dyn Message>);
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
