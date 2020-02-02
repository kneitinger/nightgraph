//mod geometry;
pub mod geometry;
pub mod geometry_3d;
pub mod page;
pub use geometry::*;

pub fn foo(p: Point) -> geometry::Line {
    page::Page::new(1., 1., page::PageUnit::Mm);
    Line::new(p, p)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
