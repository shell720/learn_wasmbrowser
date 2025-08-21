mod utils;

use wasm_bindgen::prelude::*;
use std::fmt;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-game-of-life!");
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell{
    Dead,
    Alive
}

#[wasm_bindgen]
pub struct Universe{
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe{
    pub fn new()-> Universe{
        let width = 64;
        let height = 64;

        let cells = (0..width*height).map(|i| {
            if i%2==0 || i%7==0{
                Cell::Alive
            } else{
                Cell::Dead
            }
        }).collect();

        Universe{
            width,
            height,
            cells,
        }
    }

    pub fn render(&self) -> String{
        self.to_string()
    }

    fn get_index(&self, x: u32, y: u32) -> usize{
        (y*self.width+x) as usize
    }

    fn live_neighbor_count(&self, x: u32, y: u32) -> u8{
        let mut cnt = 0;
        for delta_y in [self.height-1, 0, 1] {
            for delta_x in [self.width-1, 0, 1]{
                if delta_x == 0 && delta_y==0{
                    continue;
                }

                let neighbor_x = (x+delta_x) % self.width;
                let neighbor_y = (y+delta_y) % self.height;
                let idx = self.get_index(neighbor_x, neighbor_y);
                if  self.cells[idx] == Cell::Alive {
                    cnt += 1;
                }
            }
        }
        cnt
    }

    pub fn tick(&mut self){
        let mut next = self.cells.clone();

        for y in 0..self.height{
            for x in 0..self.width{
                let idx = self.get_index(x, y);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(x, y);

                let next_cell = match (cell, live_neighbors){
                    (Cell::Alive, x) if x<2 => Cell::Dead,
                    (Cell::Alive, 2)  => Cell::Alive,
                    (Cell::Alive, 3)  => Cell::Alive,
                    (Cell::Alive, x) if x>3 => Cell::Dead,
                    (Cell::Dead, 3)  => Cell::Alive,
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }
}

impl fmt::Display for Universe{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result{
        for line in self.cells.as_slice().chunks(self.width as usize){
            for &cell in line{
                let s = match cell{
                    Cell::Dead  => '◻' ,
                    Cell::Alive => '◼',
                };

                write!(f, "{}", s)?;
            }

            write!(f, "\n")?;
        }
        Ok(())
    }
}