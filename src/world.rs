use crate::boid::Boid;
use crate::grid::Grid;

use raylib::prelude::*;

pub struct World {
    boids: Grid<Boid>,
}
impl World {
    pub fn new(size: (i32, i32), n_boids: usize) -> Self {
        let mut world = Self{
            boids:Grid::new((size.0/10, size.1/5))
        };
        world.boids.populate(n_boids, size);
        world
    }

    pub fn update(&mut self, d_time: f32, size: (i32, i32), distances: (f32, f32, f32), weights: (f32, f32, f32)) {
        let old_boids = &mut self.boids.clone();
        for boid in self.boids.iter_mut() {
            boid.update(d_time, old_boids, size, distances, weights);
        }
        self.boids.update_grid(size);
    }

    pub fn display(&mut self,  d: &mut RaylibDrawHandle) {
        if d.is_key_down(KeyboardKey::KEY_LEFT_ALT) {self.boids.display_grid(d);}
        for boid in self.boids.iter_mut() {
            boid.display(d);
        }
    }

    pub fn display_info(&mut self, d: &mut RaylibDrawHandle) {
        let infos = vec![
                format!("FPS: {}", d.get_fps()),
                format!("BOI: {}", self.boids.count)];
        let mut y = 10;
        for info in infos {
            d.draw_text(info.as_str(), 10, y, 20, Color::new(255, 255, 255, 200));
            y += 25;
        } 
    }
}