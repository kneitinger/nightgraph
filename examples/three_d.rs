use knart::geometry::*;
use knart::geometry_3d::*;
use knart::page::*;

fn main() {
    let mut page = Page::new_from_pagetype(PageType::Pad11x14);

    fn path_gen(y: f64) -> Path3 {
        Path3::new(vec![
            point3(20, y, 40),
            point3(20, y, 20),
            point3(40, y, 20),
            point3(40, y, 40),
        ])
    }

    let paths: Vec<Path3> = (20..40).step_by(5).map(|y| path_gen(y as f64)).collect();

    let line = Line::new(point(20, 20), point(40, 40));
    page.add(&line);

    for p in paths {
        let projected_path = p
            .projected(&point3(30, 30, -30), &point3(30, 20, 30))
            .flatten();
        page.add(&projected_path);
    }

    page.save("image.svg".to_string());
}
