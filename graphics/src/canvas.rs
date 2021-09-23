use crate::geometry::{Shape, Shaped};
use crate::units::{Point, Size};

impl From<Canvas> for CanvasElement {
    fn from(c: Canvas) -> Self {
        Self::Canvas(c)
    }
}
impl From<Shape> for CanvasElement {
    fn from(s: Shape) -> Self {
        Self::Shape(s)
    }
}
impl<T: Shaped + Into<Shape> + Clone + 'static> From<T> for CanvasElement {
    fn from(s: T) -> CanvasElement {
        Self::Shape(Shape::new(s))
    }
}

pub enum CanvasElement {
    Canvas(Canvas),
    Shape(Shape),
}

pub struct Canvas {
    components: Vec<Shape>,
    elements: Vec<CanvasElement>,
    origin: Point,
    size: Size,
    inner: kurbo::Rect,
}

impl Canvas {
    pub fn new(origin: Point, size: Size) -> Self {
        Self {
            components: vec![],
            elements: vec![],
            origin,
            size,
            inner: kurbo::Rect::new(
                origin.x,
                origin.y,
                origin.x + size.width,
                origin.y + size.height,
            ),
        }
    }

    pub fn uniform_margin_subcanvas(&self, margin: f64) -> Self {
        let origin = self.origin + (margin, margin);
        let size = self.size - Size::new(margin * 2., margin * 2.);
        Self {
            origin,
            size,
            elements: vec![],
            components: vec![],
            inner: kurbo::Rect::new(
                origin.x,
                origin.y,
                origin.x + size.width,
                origin.y + size.height,
            ),
        }
    }

    pub fn tiled_subcanvases(&self, x: u32, y: u32, inner_margin: f64) -> Vec<Self> {
        let cell_width = self.size.width / x as f64 - inner_margin * (x - 1) as f64;
        let cell_height = self.size.height / y as f64 - inner_margin * (y - 1) as f64;
        let cell_size = Size::new(cell_width, cell_height);

        let mut canvases = vec![];

        for i_x in 0..x {
            for i_y in 0..y {
                let origin = Point::new(
                    i_x as f64 * (cell_width + inner_margin),
                    i_y as f64 * (cell_height + inner_margin),
                );
                canvases.push(Canvas::new(origin, cell_size));
            }
        }
        canvases
    }

    pub fn rect(&self) -> kurbo::Rect {
        kurbo::Rect::from_origin_size(self.origin, self.size)
    }

    pub fn add<T: Into<CanvasElement>>(&mut self, component: T) {
        self.elements.push(component.into());
    }

    pub fn components(&self) -> &Vec<Shape> {
        &self.components
    }
    pub fn elements(&self) -> &Vec<CanvasElement> {
        &self.elements
    }
    fn inner(&self) -> kurbo::Rect {
        self.inner
    }
}
