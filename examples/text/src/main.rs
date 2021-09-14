use nightgraphics::prelude::*;

fn main() {
    let mut page = Page::new(8., 4., Unit::In);
    let text_path = TextBuilder::new()
        .size(2. * Unit::In.scale() as f32)
        .text("hello")
        .origin(point(
            2.5 * Unit::In.scale() as f32,
            2.5 * Unit::In.scale() as f32,
        ))
        .build()
        .unwrap();

    page.add(&text_path);
    page.save("image.svg".to_string());
}
