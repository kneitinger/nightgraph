use std::f64::consts::{E, TAU};

/// Exponential decay f(t) = e<sup>-lambda * t</sup>
pub fn exp_dec(lambda: f64, t: f64) -> f64 {
    E.powf(-lambda * t)
}

/// Sine wave of the form f(t) = amplitude * sin(2π * t + phase)
pub fn sine_wave(amplitude: f64, freq: f64, t: f64, phase: f64) -> f64 {
    amplitude * (TAU * freq * t + phase).sin()
}
