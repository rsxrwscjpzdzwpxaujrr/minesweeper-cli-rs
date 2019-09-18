/*
 * Copyright (c) 2019, Мира Странная <rsxrwscjpzdzwpxaujrr@yahoo.com>
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License
 * as published by the Free Software Foundation; either version 2
 * of the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

extern crate rand;

use rand::Rng;

pub struct Cell {
    pub bomb: bool,
    pub bombs: i32,
    pub opened: bool,
    pub flag: bool,
}

pub struct Field {
    cells: Vec<Vec<Cell>>,
    flags: i32,
    opened: i32,
    bombs: i32,
    generated: bool,
}

impl Cell {
    fn new() -> Cell {
        return Cell{ bomb: false,
                     bombs: 0,
                     opened: false,
                     flag: false }
    }
}

impl Field {
    pub fn new(width: i32, height: i32, bombs: i32) -> Field {
        let mut field: Field = Field { cells: Vec::new(),
                                       flags: 0,
                                       opened: 0,
                                       bombs: bombs,
                                       generated: false };

        for _ in 0..width {
            field.cells.push(Vec::new());
            let vec2 = field.cells.last_mut().unwrap();

            for _ in 0..height {
                vec2.push(Cell::new());
            }
        }

        return field;
    }

    fn generate(&mut self, x_noplace: i32, y_noplace: i32) {
        let mut rng = rand::thread_rng();
        let mut bombs_placed = 0;
        
        while bombs_placed < self.bombs {
            let x = rng.gen_range(0, self.width());
            let y = rng.gen_range(0, self.height());

            let mut cell = &mut self.cells[x as usize][y as usize];

            if !cell.bomb && (x != x_noplace && y != y_noplace) {
                cell.bomb = true;
                bombs_placed += 1;
            }
        }

        for i in 0..self.width() {
            for j in 0..self.height() {
                self.check_bombs(i, j);
            }
        }

        self.generated = true;
    }

    pub fn check_bombs(&mut self, x: i32, y: i32) {
        use std::cmp::{ max, min };

        let mut bombs: i32 = 0;

        for i in max(x - 1, 0) .. min(x + 2, self.width()) {
            for j in max(y - 1, 0) .. min(y + 2, self.height()) {
                let cell = &self.cells[i as usize][j as usize];

                if cell.bomb {
                    bombs += 1;
                }
            }
        }

        self.cells[x as usize][y as usize].bombs = bombs;
    }

    pub fn open(&mut self, x: i32, y: i32) -> bool {
        use std::cmp::{ max, min };

        if !self.generated {
            self.generate(x, y);
        }

        let mut cell = &mut self.cells[x as usize][y as usize];

        if cell.opened {
            return false;
        }

        self.opened += 1;
        cell.opened = true;
        cell.flag = false;

        if cell.bomb {
            return true;
        }

        if cell.bombs == 0 {
            for i in max(x - 1, 0) .. min(x + 2, self.width()) {
                for j in max(y - 1, 0) .. min(y + 2, self.height()) {
                    let cell = &self.cells[i as usize][j as usize];

                    if !cell.opened && !cell.flag {
                        self.open(i, j);
                    }
                }
            }
        }

        return false;
    }

    pub fn open_all(&mut self) {
        for i in self.cells.iter_mut() {
            for cell in i.iter_mut() {
                if (cell.bomb && !cell.flag || cell.flag && !cell.bomb) && !cell.opened{
                    cell.opened = true;
                    self.opened += 1;
                }
            }
        }
    }

    pub fn auto_open(&mut self, x: i32, y: i32) -> bool {
        use std::cmp::{ max, min };

        let cell = &self.cells[x as usize][y as usize];

        if cell.opened {
            let mut flagged = 0;

            for i in max(x - 1, 0) .. min(x + 2, self.width()) {
                for j in max(y - 1, 0) .. min(y + 2, self.height()) {
                    let cell = &self.cells[i as usize][j as usize];

                    if cell.flag {
                        flagged += 1;
                    }
                }
            }

            if flagged >= cell.bombs {
                for i in max(x - 1, 0) .. min(x + 2, self.width()) {
                    for j in max(y - 1, 0) .. min(y + 2, self.height()) {
                        let cell = &self.cells[i as usize][j as usize];

                        if !cell.flag {
                            if self.open(i, j) {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        
        return false;
    }

    pub fn flag(&mut self, x: i32, y: i32) {
        let mut cell = &mut self.cells[x as usize][y as usize];

        if !cell.opened {
            if !cell.flag { self.flags += 1; }
            else          { self.flags -= 1; }

            cell.flag = !cell.flag;
        }
    }

    pub fn cell(&self, x: i32, y: i32) -> &Cell {
        return &self.cells[x as usize][y as usize];
    }

    pub fn width(&self) -> i32 {
        return self.cells.len() as i32;
    }

    pub fn height(&self) -> i32 {
        return self.cells[0].len() as i32;
    }

    pub fn opened(&self) -> i32 {
        return self.opened;
    }

    pub fn bombs(&self) -> i32 {
        return self.bombs;
    }

    #[allow(dead_code)]
    pub fn flags(&self) -> i32 {
        return self.flags;
    }
}
