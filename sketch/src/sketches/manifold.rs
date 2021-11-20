use super::*;
use std::f64::consts::{PI, TAU};

use rand::prelude::*;
use rand_pcg::Pcg64;

/// A collection of concentric sine wave groupings forming nested "flowers"
#[sketch]
pub struct Manifold {
    #[param(default = 1.5, range = 0.01..=8.0)]
    dist_interval: f64,

    #[param(default = 0.4, range = 0.01..=8.0)]
    ring_width: f64,
    /// The number of rings to draw
    #[param(default = 21, range=1..=80)]
    levels: u64,

    /// The number of steps to sample the sine wave(s) at during a circular
    /// sweep of a ring
    #[param(default = 50, range=2..=50)]
    rotational_steps: u64,

    #[param(default = 4.4, range = 0.0..=8.0)]
    decay_maj: f64,

    #[param(default = 3.5, range = 0.0..=8.0)]
    decay_min: f64,

    #[param(default = 534)]
    seed: u64,

    #[param(default = 4)]
    flower_count: u64,

    #[param(default = true)]
    dir_toggle: bool,
}

impl Manifold {
    fn flower(
        &self,
        center: Point,
        n: u64,
        ring_dir: bool,
        rng: &mut Pcg64,
    ) -> SketchResult<Vec<Path>> {
        let phase_rand_prim = rng.gen_range(0.0..1.0);
        let phase_rand_sec = rng.gen_range(0.0..1.0);
        let rand_freq_a = 0;
        let rand_freq_b = rng.gen_range(-2.0..2.0);

        let rand_theta_start = rng.gen_range(0.0..TAU);

        let mut paths = vec![];
        let steps = self.rotational_steps;
        let levels = self.levels;
        let th_delta = TAU / steps as f64;

        let mut points = vec![];
        for level in 0..levels {
            points.clear();
            let level_prog = level as f64 / levels as f64;
            for step in 0..steps {
                let th = rand_theta_start + step as f64 * th_delta;
                let step_prog = step as f64 / steps as f64;

                let base_dist = n as f64 * self.dist_interval * INCH
                    + (level_prog * self.ring_width * INCH * if ring_dir { 1. } else { -1. });
                let sin_mod = sine_wave(
                    0.25 * INCH + (n as f64 * 0.125 * INCH),
                    5. + rand_freq_a as f64,
                    step_prog,
                    phase_rand_prim * PI,
                ) * exp_dec(self.decay_maj, level_prog)
                    + sine_wave(
                        0.0625 * INCH + (0.0625 * INCH * n as f64),
                        15. + rand_freq_b,
                        step_prog,
                        phase_rand_sec,
                    ) * exp_dec(self.decay_min, level_prog);
                let dist = (base_dist + sin_mod) * Vec2::from_angle(th);
                points.push(center + dist);
            }

            let path = PathBuilder::new()
                .points(&points)
                .closed()
                .smooth()
                .precompute()
                .stroke_width(if level == 0 { 0.7 * MM } else { 0.2 * MM })
                .build()?;
            paths.push(path);
        }

        Ok(paths)
    }
}

impl Sketch for Manifold {
    fn exec(&self) -> SketchResult<Canvas> {
        const WIDTH: f64 = 11. * INCH;
        const HEIGHT: f64 = 17. * INCH;
        let mut rng = Pcg64::seed_from_u64(self.seed);

        let mut canvas = Canvas::new(point(0, 0), Size::new(WIDTH, HEIGHT));
        let center = point(WIDTH / 2., HEIGHT / 2.);

        for n in 0..self.flower_count {
            let dir_outer = n % 2 == (if self.dir_toggle { 0 } else { 1 });
            let flower = self.flower(center, n + 1, dir_outer, &mut rng)?;
            for path in flower {
                canvas.add(path);
            }
        }

        Ok(canvas)
    }
}
