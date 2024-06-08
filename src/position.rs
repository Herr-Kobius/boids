use raylib::prelude::*;
use std::ops::Range;

pub trait Position {
    fn new(x_range: Range<f32>, y_range: Range<f32>) -> Self;
    fn get_position(&self) -> Vector2;
}