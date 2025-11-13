use serde::{Deserialize, Serialize};
use std::{collections::HashSet, hash::Hash};

type Cells = HashSet<CellPos>;

#[derive(Clone, Serialize, Deserialize)]
pub struct Grid {
    pub alive: Cells,
    pub generation: u64,
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CellPos {
    pub x: i32,
    pub y: i32,
}

impl CellPos {
    pub fn new(x: i32, y: i32) -> Self {
        CellPos { x, y }
    }
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            alive: Cells::with_capacity(100_000),
            generation: 0,
        }
    }

    fn is_alive(&self, pos: &CellPos) -> bool {
        self.alive.contains(pos)
    }

    pub fn revive(&mut self, pos: CellPos) -> bool {
        self.alive.insert(pos)
    }

    pub fn kill(&mut self, pos: &CellPos) -> bool {
        self.alive.remove(pos)
    }

    fn get_neighbors_positions(&self, pos: &CellPos) -> [CellPos; 8] {
        // {x-1; y-1} {x; y-1} {x+1; y-1}  - previous row
        // {x-1; y}   {x; y}   {x+1; y}    - current row
        // {x-1; y+1} {x; y+1} {x+1; y+1}  - next row

        let previous_row_y = pos.y - 1;
        let next_row_y = pos.y + 1;

        [
            CellPos::new(pos.x - 1, previous_row_y),
            CellPos::new(pos.x, previous_row_y),
            CellPos::new(pos.x + 1, previous_row_y),
            // current row
            CellPos::new(pos.x - 1, pos.y),
            CellPos::new(pos.x + 1, pos.y),
            // next row
            CellPos::new(pos.x - 1, next_row_y),
            CellPos::new(pos.x, next_row_y),
            CellPos::new(pos.x + 1, next_row_y),
        ]
    }

    fn count_neighbors(&self, pos: &CellPos) -> u8 {
        let mut count = 0;

        for neighbor_pos in self.get_neighbors_positions(pos) {
            if self.is_alive(&neighbor_pos) {
                count += 1;
            }
        }

        return count;
    }

    pub fn evolve(&mut self) {
        let mut potential_cells = Cells::with_capacity(self.alive.len() * 4);

        for pos in &self.alive {
            potential_cells.insert(*pos);
            for potential_pos in self.get_neighbors_positions(pos) {
                potential_cells.insert(potential_pos);
            }
        }

        let mut alive_next = Cells::with_capacity(self.alive.len());

        for pos in &potential_cells {
            let live_neighbors = self.count_neighbors(pos);
            if self.alive.contains(&pos) && (live_neighbors == 2 || live_neighbors == 3) {
                alive_next.insert(*pos);
            } else if live_neighbors == 3 {
                    alive_next.insert(*pos);
                
            }
        }

        self.alive = alive_next;
        self.generation += 1;
    }

    pub fn reset(&mut self) {
        self.alive.clear();
        self.generation = 0;
    }
}
