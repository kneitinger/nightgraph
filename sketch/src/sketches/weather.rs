use super::*;
use std::f64::consts::{PI, TAU};

/// A description
#[sketch]
pub struct Weather {}

impl Sketch for Weather {
    fn exec(&self) -> SketchResult<Canvas> {
        const WIDTH: f64 = 11. * INCH;
        const HEIGHT: f64 = 17. * INCH;

        let mut canvas = Canvas::new(point(0, 0), Size::new(WIDTH, HEIGHT));
        let center = point(WIDTH / 2., HEIGHT / 2.);

        Ok(canvas)
    }
}
