#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

pub mod application;
pub mod cracker;
pub mod engine;
pub mod model;
pub mod session;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
