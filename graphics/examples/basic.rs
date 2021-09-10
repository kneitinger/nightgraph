use nightgraphics::prelude::*;

fn main() {
    let mut page = Page::new(10., 10., Unit::In);
    let mut group = Group::new("test_group");

    let poly = Poly::new(vec![
        point(10, 10),
        point(50, 10),
        point(90, 90),
        point(10, 90),
    ]);
    page.add(&poly);

    let circ = Circle::new(point(50, 50), 20.);
    page.add(&circ);

    let line = Line::new(point(10, 100), point(150, 80));
    group.add(&line);

    let mut text = Text::new();
    text.set_size(200.);
    text.set_origin(point(10, 300));
    group.add(&text);

    page.add_group(&group);

    page.save("image.svg".to_string());
}
