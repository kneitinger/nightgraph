use nightgraph_graphics::prelude::*;

fn main() {
    let mut page = Page::new(8., 4., Unit::In);
    let text_paths = Text::new()
        .set_size(2. * Unit::In.scale() as f32)
        .set_text("hello")
        .set_origin(point(
            2.5 * Unit::In.scale() as f32,
            2.5 * Unit::In.scale() as f32,
        ))
        .paths();

    for p in text_paths {
        page.add(&p);
    }
    page.save("image.svg".to_string());
}
