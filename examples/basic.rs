use knart::geometry::*;
use knart::page::*;
use knart::Unit;

fn main() {
    let mut page = Page::new(10., 10., Unit::Mm);
    let poly = Poly::new(vec![point(1, 1), point(9, 1), point(9, 9), point(1, 9)]);
    let lines = poly.hatch(0.01, 0., 0.);

    for l in lines {
        page.add(&l);
    }

    page.save("image.svg".to_string());
}
