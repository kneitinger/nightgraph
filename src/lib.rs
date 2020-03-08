//#![warn(clippy::all, clippy::pedantic, clippy::cargo)]

//mod geometry;
pub mod geometry;
pub mod geometry_3d;
pub mod page;
pub mod units;
pub use geometry::*;
pub use units::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
