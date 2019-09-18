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

extern crate ncurses;

use super::field::*;

pub fn render(field: &Field) {
    use ncurses::*;

    for i in (0..field.height()).rev() {
        for j in 0..field.width() {
            let cell = &field.cell(j, i);

            if cell.opened {
                attron(A_BOLD());
                if cell.bomb || cell.flag{
                    attron(COLOR_PAIR(9));
                }

                addstr(" ");

                if cell.bomb {
                    addstr("X");
                } else if cell.flag {
                    addstr("!");
                } else if cell.bombs > 0 {
                    attron(COLOR_PAIR(cell.bombs as i16));
                    addstr(&format!("{}", cell.bombs));
                    attroff(COLOR_PAIR(cell.bombs as i16));
                } else {
                    addstr(" ");
                }
                
                addstr(" ");

                if cell.bomb || cell.flag {
                    attroff(COLOR_PAIR(9));
                }
                attroff(A_BOLD());
            } else {
                attron(A_REVERSE());

                addstr(" ");

                if cell.flag {
                    attron(COLOR_PAIR(10));
                    addstr(">");
                    attroff(COLOR_PAIR(10));
                } else {
                    addstr(".");
                }

                addstr(" ");

                attroff(A_REVERSE());
            }
        }

        addstr("\n");
    }
}