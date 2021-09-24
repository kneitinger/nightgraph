use nightgraphics::prelude::*;
use rand::prelude::*;

fn main() {
    //let mut page = Page::new(10., 10., Unit::In);

    let mut canvas = Canvas::new(point(0, 0), Size::new(10. * INCH, 10. * INCH));

    let poly = Poly::new(vec![
        point(10, 10),
        point(50, 10),
        point(50, 50),
        point(10, 50),
    ])
    .unwrap();
    println!("{}", poly.area());
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
    // Area is additive instead of subtractive
    println!("{}", cpath.area());
    let p = point(100, 35);
    canvas.add(Circle::new(p, 2.));
    // Contains does not account for holes
    println!("contains {} ? {}", p, cpath.contains(p));
    canvas.add(cpath);

    let circ = Circle::new(point(50, 50), 20.);
    canvas.add(circ);

    let mut path = Path::new(point(5, 5), PathEl::LineTo(point(25, 60)));
    path.line_to(point(10, 80));
    canvas.add(path);

    let text = TextBuilder::new()
        .size(200.)
        .origin(point(thread_rng().gen_range(10..30), 300))
        .build()
        .unwrap();
    canvas.add(text);

    canvas.render_svg();
}
