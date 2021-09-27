use itertools::Itertools;
use nightgraphics::prelude::*;
use rand::prelude::*;

fn main() {
    let mut canvas = Canvas::new(point(0, 0), Size::new(11. * INCH, 17. * INCH));

    let text = TextBuilder::new()
        .size(200.)
        .text_line("In a dream")
        .text_line("one eludes.")
        .text_line("In a day")
        .text_line("one dilutes.")
        .line_padding(10.)
        .origin(point(150, 3. * INCH))
        .build()
        .unwrap();

    let mut lines = vec![];
    let mut line_count = 0;
    let line_max = 400;

    fn gen_point() -> Point {
        let x_range = 1. * INCH..10. * INCH;
        let y_range = 1. * INCH..16. * INCH;

        point(
            thread_rng().gen_range(x_range),
            thread_rng().gen_range(y_range),
        )
    }
    loop {
        let a = gen_point();
        let b = gen_point();

        if text.contains(a) || text.contains(b) {
            continue;
        }
        line_count += 1;
        let line = Line::new(a, b).unwrap();

        let mut intersections = text.intersections(&line);
        let mut points = vec![a, b];
        points.append(&mut intersections);
        points.sort_by(|p0, p1| p0.x.partial_cmp(&p1.x).unwrap());
        let _ = points
            .iter()
            .tuples()
            .map(|(p0, p1)| lines.push(Line::new(*p0, *p1)))
            .collect::<Vec<()>>();

        if line_count == line_max {
            break;
        }
    }

    canvas.add(text);
    for l in lines {
        canvas.add(l.unwrap());
    }

    canvas.render_svg();
}
