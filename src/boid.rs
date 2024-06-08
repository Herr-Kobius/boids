use rand::Rng;
use raylib::prelude::*;

use crate::{grid::Grid, position::Position};

#[derive(Debug, Clone)]
pub struct Boid {
    pos: Vector2,
    vel: Vector2,
    acc: Vector2,
    max_vel: f32
}

impl Boid {
    pub fn get_angle(&mut self) -> f32{
        if self.vel.length() == 0.0 {
            0.0
        } else {
            Vector2{x:0.0, y:0.0}.angle_to(self.vel)
        }
    }

    pub fn rotate(v: Vector2, angle: f32) -> Vector2 {
        Vector2{
            x: v.x*angle.cos()-v.y*angle.sin(),
            y: v.x*angle.sin()+v.y*angle.cos()
        }
    }

    pub fn update(&mut self, d_time:f32, old_boids: &mut Grid<Boid>, size: (i32, i32), distances: (f32, f32, f32), weights: (f32, f32, f32)) {
        let mut sep = Vector2::zero();
        let mut ali = Vector2::zero();
        let mut coh = Vector2::zero();
        let boids = old_boids.get_neighbors(self.get_position(), distances.0);
        for boid in boids.iter() {
            let con = boid.get_position()-self.get_position();
            coh+=boid.get_position();
            if con.length() < distances.1 {
                ali +=boid.vel;
                if con.length() < distances.2 {
                    sep += -con*(20.0/(con.length()+1.0));
                }
            }
           
        }
        coh = coh/boids.len() as f32-self.pos;
        ali = ali/boids.len() as f32;
        sep = sep/boids.len() as f32;

        self.acc = (coh*weights.0+ali*weights.1+sep*weights.2)/3.0;

        self.vel+=self.acc*d_time;
        self.acc.scale(0.0);
        if self.vel.length() > self.max_vel {
            self.vel.normalize();
            self.vel.scale(self.max_vel);
        }
        self.pos+=self.vel*d_time;
        self.pos.x = self.pos.x.rem_euclid(size.0 as f32);
        self.pos.y = self.pos.y.rem_euclid(size.1 as f32);
    }

    pub fn display(&mut self, d: &mut RaylibDrawHandle) {
        let angle = self.get_angle();
        let tip = self.pos+Self::rotate(Vector2 { x: 7.0, y: 0.0}, angle);
        let rw =self.pos+Self::rotate(Vector2 { x: -7.0, y: 5.0}, angle);
        let lw = self.pos+Self::rotate(Vector2 { x: -7.0, y: -4.0}, angle);
        let dip = self.pos+Self::rotate(Vector2 { x: -4.0, y: 0.0}, angle);
        d.draw_triangle(tip, dip, rw, Color::WHITE);
        d.draw_triangle(tip, lw, dip, Color::WHITE);
    }
}


impl Position for Boid {
    fn new(x_range: std::ops::Range<f32>, y_range: std::ops::Range<f32>) -> Self {
        let mut rng = rand::thread_rng();
        Self { 
            pos: Vector2::new(rng.gen_range(x_range), rng.gen_range(y_range)),
            vel: Self::rotate(Vector2{x:70.0, y:0.0}, rng.gen_range(0.0..6.28)),
            acc: Vector2::zero(),
            max_vel: 70.0
        }
    }

    fn get_position(&self) -> raylib::prelude::Vector2 {
        self.pos
    }
}