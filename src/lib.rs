//mod geometry;
pub mod geometry;
pub mod page;
pub use geometry::*;
#[cfg(test)]

pub fn foo(p: Point) -> geometry::Line {
    page::Page::new(1., 1., page::PageUnit::Mm);
    Line::new(p, p)
}

mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
