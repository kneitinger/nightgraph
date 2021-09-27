use nightgraphics::prelude::*;

fn main() {
    let mut canvas = Canvas::new(point(0, 0), Size::new(11. * INCH, 17. * INCH));

    let poly = Poly::new(vec![
        point(10, 10),
        point(50, 10),
        point(50, 50),
        point(10, 50),
    ])
    .unwrap();
    canvas.add(poly);

    // Test complex paths (i.e. with holes)
    let mut cpath = Path::new(point(90, 10), PathEl::LineTo(point(130, 10)));
    cpath.line_to(point(130, 50));
    cpath.line_to(point(90, 50));
    cpath.close();
    cpath.move_to(point(95, 15));
    cpath.line_to(point(125, 15));
    cpath.line_to(point(125, 45));
    cpath.line_to(point(95, 45));
    cpath.close();
    let p = point(100, 35);
    canvas.add(Circle::new(p, 2.));
    canvas.add(cpath);

    let circ = Circle::new(point(50, 50), 20.);
    canvas.add(circ);

    let mut path = Path::new(point(5, 5), PathEl::LineTo(point(25, 60)));
    path.line_to(point(10, 80));
    canvas.add(path);

    let text = TextBuilder::new()
        .size(200.)
        .text_line("fooosdfsdfs")
        .origin(point(150, 8. * INCH))
        .build()
        .unwrap();
    canvas.add(text);

    canvas.render_svg();
}
