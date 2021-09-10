use nightgraphics::prelude::*;

fn main() {
    let mut page = Page::new(8., 4., Unit::In);
    let text_path = Text::new()
        .set_size(2. * Unit::In.scale() as f32)
        .set_text("hello")
        .set_origin(point(
            2.5 * Unit::In.scale() as f32,
            2.5 * Unit::In.scale() as f32,
        ))
        .to_path();

    page.add(&text_path);
    page.save("image.svg".to_string());
}
