<h1 align="center">
  <br>
  <br>
  Boids - Flocking Simulation
  <br>
</h1>

<h4 align="center">A flocking Simulation using the three simple steering behaviors based on the local flockmates.
<br>
<br>
<p align="center">
  <a href="https://docs.rs/crate/raylib/latest">
    <img src="https://img.shields.io/badge/raylib--rs-3.7.0-white.svg?logo=raylib">
  </a>
  <a href="https://www.raylib.com">
      <img src="https://img.shields.io/badge/raylib-5.0.1-white.svg?logo=raylib">
  </a>
  <a href="https://www.rust-lang.org">
    <img src="https://img.shields.io/badge/Rust-1.74.0-red.svg?logo=rust">
  </a>
</p>


![Simulation using 500 boids](/img/demo.gif)

## Description

Each boid will start with a random velocity and afterward, only follow the steering behaviors: 
- **Separation -** A boid will avoid nearby crowds
- **Alignment -** Steering towards the average heading
- **Cohesion -** Boids will move to the average position of the local group

Each behavior influences the boid with a specific weight, using a different radius to determine local mates.
```rust
pub fn update(&mut self, d_time: f32, size: (i32, i32), 
                distances: (f32, f32, f32), // (100.0, 100.0, 20.0)
                weights: (f32, f32, f32))   // (0.01, 1.0, 10.0)
```
Boids are organized in a grid of cells to improve performance. The sorting prevents quadratic growth, which would have a time complexity of $O(n²)$. In the case of a homogeneous distribution of the boids, the growth, without overhead such as sorting and cell selections, is linear $O(n)$.

## References 

Idea: [Boids](http://www.red3d.com/cwr/boids/) by Craig Reynolds

Optimization: [Interaction with Groups of Autonomous Characters](https://www.red3d.com/cwr/papers/2000/pip.pdf) by Craig Reynolds

Graphics: [raylib-rs](https://docs.rs/raylib/latest/raylib/)


## Credits

This software uses the following packages:

- [raylib-rs](https://docs.rs/crate/raylib/latest)
- [raylib](https://www.raylib.com)

## License  

[Attribution-NonCommercial-ShareAlike 4.0 International](/LICENSE.txt)
