mod world;
mod boid;
mod grid;
mod position;


use raylib::prelude::*;

use crate::world::World;

fn main() {
    let (mut rl, thread) = raylib::init()
            .title("Boids")
        //  .size(1920, 1020)
            .size(1920, 1080).fullscreen()
            .resizable().build();
    
    let size = (rl.get_screen_width(), rl.get_screen_height());
    rl.set_target_fps(100);
    
    let mut world = World::new(size, 500);
   
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        let size = (d.get_screen_width(), d.get_screen_height());
        let frame_time = d.get_frame_time();

        d.clear_background(Color::BLACK);
        world.update(frame_time, size, (100.0, 100.0, 20.0), (0.01, 1.0, 10.0));
        
        world.display(&mut d);

        if d.is_key_pressed(KeyboardKey::KEY_I) {world.display_info(&mut d)}
    }
}
