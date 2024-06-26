//! This module contains the Window struct and its methods.
//!
//! The Window struct is a wrapper around the ncurses WINDOW struct.

use anyhow::Result;
use ncurses::*;

pub enum ArrowKeys {
    Up = 65,
    Down = 66,
    Right = 67,
    Left = 68,
}

#[derive(Clone, Copy)]
pub enum Color {
    Black = 0,
    Red = 1,
    Green = 2,
    Yellow = 3,
    Blue = 4,
    Magenta = 5,
    Cyan = 6,
    White = 7,
}

#[derive(Clone, Copy)]
pub struct ColorPair {
    foreground: Color,
    background: Color,
}

impl ColorPair {
    pub fn new(foreground: Color, background: Color) -> Self {
        ColorPair {
            foreground,
            background,
        }
    }
}

pub struct Window {
    win: *mut i8,
    rows: i32,
    cols: i32,
    x: i32,
    y: i32,
}

impl Window {
    pub fn new(rows: i32, cols: i32, x: i32, y: i32) -> Self {
        let win: *mut i8 = newwin(rows, cols, x, y);
        let new_window: Self = Self {
            win,
            rows,
            cols,
            x,
            y,
        };
        new_window.mv();
        new_window
    }

    pub fn draw_border(&self) -> Result<()> {
        //! Draws a border along the right-hand side of the window.
        for i in 0..self.rows {
            mvwprintw(self.win, i, self.cols - 1, "|")?;
        }

        Ok(())
    }

    pub fn refresh(&self) {
        wrefresh(self.win);
    }

    pub fn erase(&self) {
        werase(self.win);
    }

    pub fn print(&self, x: i32, y: i32, s: &str, color_pair: Option<&ColorPair>) -> Result<()> {
        //! Prints a string to the window at the specified x and y coordinates.
        if let Some(color) = color_pair {
            init_pair(1, color.foreground as i16, color.background as i16);
            wattron(self.win, COLOR_PAIR(1));
            mvwprintw(self.win, y, x, s)?;
            wattroff(self.win, COLOR_PAIR(1));
        } else {
            mvwprintw(self.win, y, x, s)?;
        }
        Ok(())
    }

    pub fn getch(&self) -> i32 {
        wgetch(self.win)
    }

    pub fn getmaxyx(&self, y: &mut i32, x: &mut i32) {
        getmaxyx(self.win, y, x);
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }

    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }

    pub fn inc_x(&mut self, x: i32) {
        self.x += x;
    }

    pub fn inc_y(&mut self, y: i32) {
        self.y += y;
    }

    pub fn mv(&self) {
        mvwin(self.win, self.y, self.x);
    }

    pub fn get_rows(&self) -> i32 {
        self.rows
    }

    pub fn get_cols(&self) -> i32 {
        self.cols
    }
}


pub fn draw(window: &mut Window, char_grid: &[Vec<char>]) -> Result<()> {
    //! Draws the grid on the screen
    //!
    //! # Arguments
    //! * `grid` - The grid to draw
    //! * `nrows` - Number of rows in the grid
    //! * `ncols` - Number of columns in the grid
    //! * `input_handler` - Input handler to get the character to draw for alive cells
    for (i, row) in char_grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            let output = format!(
                "{}",
                c
            );
            //window.print(j as i32 * 2, i as i32, &output, None)?;
            window.print(j as i32,i as i32, &output, None)?;
        }
    }
    window.print(
        0,
        char_grid.len() as i32,
        &format!(
            "q: Quit, a: increase timeout, s: decrease timeout"
        ),
        // &format!(
        //     "Alive: {}, Timeout: {} | q: Quit, a: increase timeout, s: decrease timeout",
        //     num_alive, state.timeout
        // ),
        None,
    )
}


// pub fn draw(grid: &Vec<Cell>, nrows: usize, ncols: usize) {
//     /// Render of grid blocks, a point of refactoring if getting slow. 
//     for i in 0..nrows {
//         for j in 0..ncols {
//             let output = format!(
//                 "{}",
//                     match grid[i * ncols + j].value {
//                         CellValue::Empty => ' ',                                    
//                         CellValue::Steam(4) => '█',
//                         CellValue::Steam(3) => '▓',
//                         CellValue::Steam(2) => '▒',
//                         CellValue::Steam(1) => '░',
//                         CellValue::Scene(c) => c,
//                         _ => ' ',
//                     }
//             );
//             let _ = addstr(&output);
//         }
//         let _ = addch('\n' as u32);
//     }
// }
