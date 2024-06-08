use std::cmp::{max, min};
use raylib::prelude::*;

use crate::position::Position;

#[derive(Clone, Debug)]
pub struct Grid<T: Position + Clone> {
    cells: Vec<Vec<Vec<T>>>,
    scale: (i32, i32),
    grid_size: Option<(i32, i32)>,
    pub count: usize
}

impl<T: Position + Clone> Grid<T> {
    pub fn new(scale:(i32, i32)) -> Self {
        Self {
            cells: vec![],
            scale,
            grid_size: None,
            count: 0
        }
    }

    pub fn populate(&mut self, n: usize, size: (i32, i32)) {
        let (w, h) = (size.0 / self.scale.0, size.1 / self.scale.1);
        self.grid_size = Some((w, h));
        for xd in 0..w {
            self.cells.push(vec![]);
            for _ in 0..h {
                self.cells[xd as usize].push(vec![]);
            }
        }
        for _ in 0..n {
            let new_item: T = T::new(0.0..size.0 as f32, 0.0..size.1 as f32);
            let (x, y) = self.calculate_index(new_item.get_position());
            self.cells[x][y].push(new_item);
        }
        self.count = n;
    }

    pub fn calculate_index(& self, position: Vector2) -> (usize, usize) {
        let x = position.x as i32 / self.scale.0;
        let y = position.y as i32 / self.scale.1;
        let (max_x, max_y) = self.grid_size.expect("Grid size not set");
        (max(0, min(x, max_x-1)) as usize, max(0, min(y, max_y-1)) as usize)
    }

    pub fn get_neighbors(&mut self, position: Vector2, radius: f32) -> Vec<&mut T> {
        let (x_min, y_min) = self.calculate_index(position-radius);
        let (x_max, y_max) = self.calculate_index(position+radius);
        let mut cells: Vec<&mut Vec<T>> = vec![];
        for col in self.cells[x_min..=x_max].iter_mut() {
            for cell in col[y_min..=y_max].iter_mut() {
                cells.push(cell);
            }
        }
        let mut neighbors: Vec<&mut T> = vec![];
        for item in cells.into_iter().flatten() {
            if item.get_position().distance_to(position) < radius {
                neighbors.push(item)
            }
        }
        neighbors
    }

    pub fn clear(&mut self) {
        self.cells = vec![];
        self.grid_size = None;
    }

    pub fn insert(&mut self, item: T) {
        let (new_x, new_y) = self.calculate_index(item.get_position());
        self.cells[new_x][new_y].push(item);
    }

    pub fn update_grid(&mut self, size: (i32, i32)) {
        let (x, y) = ((size.0 / self.scale.0), (size.1 / self.scale.1));
        
        let items = self.cells.clone().into_iter().flatten().flatten().collect::<Vec<T>>();
        self.clear();
        self.grid_size = Some((x,y));
        self.count = items.len();
        for xd in 0..x {
            self.cells.push(vec![]);
            for _ in 0..y {
                self.cells[xd as usize].push(vec![]);
            }
        }
        for item in items {
            self.insert(item.clone());
        }
    }

    pub fn display_grid(&mut self, d: &mut RaylibDrawHandle) {
        let (x, y) = self.grid_size.expect("Grid size not set");
        for xd in 0..x {
            for yd in 0..y {
                d.draw_rectangle_lines(xd*self.scale.0, yd*self.scale.1, self.scale.0, self.scale.1, Color::new(50, 50, 50, 200));
                let cell_count = self.cells[xd as usize][yd as usize].len();
                d.draw_text(format!("{}", cell_count).as_str(), (xd*self.scale.0)+(self.scale.0)/2-4, (yd*self.scale.1)+self.scale.1/2-8, 20, Color::new(50, 50, 50, 200));
            }
        }
    }

    pub fn iter_mut(&mut self) -> std::vec::IntoIter<&mut T> {
        self.cells.iter_mut().flatten().flatten().collect::<Vec<&mut T>>().into_iter()
    }
}