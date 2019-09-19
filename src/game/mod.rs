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

mod field;
mod renderer;

use field::*;
use std::time;

pub struct Game {
    won: bool,
    lose: bool,
    wants_exit: bool,
    cursor: (i32, i32),
    window: ncurses::WINDOW,
    field: Field,
    creation_time: time::SystemTime,
}

impl Game {
    pub fn new() -> Game {
        let window = ncurses::initscr();
        ncurses::noecho();
        ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_VERY_VISIBLE);
        ncurses::keypad(ncurses::stdscr(), true);
        Game::init_colors();

        return Game { won: false,
                      lose: false,
                      wants_exit: false,
                      cursor: (2, 8),
                      window: window,
                      field: Game::new_field(&window),
                      creation_time: time::SystemTime::now() };
    }

    fn new_field(window: &ncurses::WINDOW) -> Field {
        return Field::new(Game::ask_num(&window, "Enter width: "), 
                          Game::ask_num(&window, "Enter height: "),
                          Game::ask_num(&window, "Enter bombs count: "))
    }

    fn init_colors() {
        use ncurses::*;

        start_color();
        use_default_colors();
        
        init_color(16, 300, 300, 900);
        init_color(17, 300, 600, 300);
        init_color(18, 900, 300, 300);
        init_color(19, 300, 300, 600);
        init_color(20, 600, 300, 300);
        init_color(21, 300, 600, 600);
        init_color(22, 900, 300, 300);

        init_pair(1, 16, -1);
        init_pair(2, 17, -1);
        init_pair(3, 18, -1);
        init_pair(4, 19, -1);
        init_pair(5, 20, -1);
        init_pair(6, 21, -1);
        init_pair(7, 22, -1);
        init_pair(8, -1, -1);

        init_pair(9, COLOR_BLACK, COLOR_RED);
        init_pair(10, -1, COLOR_RED);
    }

    pub fn start_loop(&mut self) {
        while !self.wants_exit {
            ncurses::erase();

            if self.lose {
                self.field.open_all();
            }

            renderer::render(&self.field);

            if self.won || self.lose {
                self.winlose();
            }

            ncurses::mv((self.field.height() as i32) - self.cursor.1 - 1, 
                        self.cursor.0 * 3 + 1);

            ncurses::refresh();
            
            if !(self.won || self.lose) {
                self.controls();
            } else if !self.wants_exit {
                ncurses::getch();
            }
        }
        
        ncurses::endwin();
    }

    fn controls(&mut self) {
        use std::cmp::{ max, min };

        let key = std::char::from_u32(ncurses::getch() as u32).unwrap();
        let low_key = key.to_lowercase().last().unwrap();

        let step = if key.is_lowercase() || !key.is_ascii_alphabetic() { 1 } else { 4 };

        match low_key {
            'd' => { self.cursor.0 += step; }
            'a' => { self.cursor.0 -= step; }
            'w' => { self.cursor.1 += step; }
            's' => { self.cursor.1 -= step; }
            _ => {}
        }

        self.cursor.0 = max(0, min(self.field.width()  - 1, self.cursor.0));
        self.cursor.1 = max(0, min(self.field.height() - 1, self.cursor.1));

        self.lose = match low_key {
            ' '        => { self.field.open(self.cursor.0, self.cursor.1) }
            'e'        => { self.field.auto_open(self.cursor.0, self.cursor.1) }
            'f' | '\t' => { self.field.flag(self.cursor.0, self.cursor.1); false }
            '\u{1b}'   => { self.wants_exit = true; false }
            _ => { false } 
        };

        if self.field.opened() == self.field.width() * self.field.height() - self.field.bombs() {
            self.won = true;
        }
    }

    fn winlose(&mut self) {
        use ncurses::*;

        let message = if      self.won  { "You won!" } 
                      else if self.lose { "You lose!" }
                      else              { panic!() };

        attron(A_BOLD());
        attron(A_BLINK());

        mvaddstr(self.field.height(), 
                 ((self.field.width() * 3) - (message.len() as i32)) / 2, 
                 message);

        attroff(A_BLINK());

        if self.won {
            let elapsed = self.creation_time.elapsed().unwrap();
            let message = format!("Your time is {} sec", elapsed.as_secs());
            addch('\n' as u32);
            addstr(&message);
        }

        attroff(A_BOLD());

        addch('\n' as u32);
        self.ask_again();
    }

    fn ask_num(window: &ncurses::WINDOW, message: &str) -> i32 {
        use ncurses::*;

        addstr(message);

        let mut key = ' ';
        let mut result = String::from("");

        while key != '\n' {
            let raw_key = getch();
            key = std::char::from_u32(raw_key as u32).unwrap_or(' ');

            if raw_key == 127 && result.len() > 0 {
                result = result[..result.len()-1].to_owned();
                mvdelch(getcury(*window), getcurx(*window) - 1);
            } else if key.is_numeric() && result.len() < 5 {
                result.push(key);
                addch(key as u32);
            } 
        }

        addch('\n' as u32);

        return result.parse().unwrap();
    }

    fn ask_again(&mut self) {
        use ncurses::*;

        let message = "Do you want to try again? (y/N): "; 

        addstr(message);

        let mut key = ' ';
        let mut oldkey = ' ';
        let mut first = true;

        while key != '\n' {
            oldkey = key;
            key = std::char::from_u32(getch() as u32).unwrap_or(' ');

            if !first {
                mvdelch(getcury(self.window), getcurx(self.window) - 1);
            }

            addch(key as u32);

            first = false;
        }

        if oldkey.to_lowercase().last().unwrap() == 'y' {
            erase();
            self.field = Game::new_field(&self.window);
            self.won = false;
            self.lose = false;
            erase();
            renderer::render(&self.field);
            refresh();
        } else {
            self.wants_exit = true;
        }
    }
}
