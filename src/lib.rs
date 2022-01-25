#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

pub mod model;
pub mod session;
pub mod application;
pub mod engine;
pub mod cracker;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
