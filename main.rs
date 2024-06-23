pub mod window;
pub mod cellular;

use std::{f32::consts::E, fs};
use cellular::update;
use clap::Parser;
use ncurses::*;
use anyhow::Result;
use window::Window;
use cellular::{Cell, CellValue};
use window::{draw, ArrowKeys};


#[derive(Debug, PartialEq, Clone, Copy)] 
enum InputType {
    Quit,
    Continue,
    IncreaseTimeout,
    DecreaseTimeout,
    Up,
    Down,
}

struct InputHandler {
    input: InputType,
}

impl InputHandler {
    fn new() -> InputHandler {
        InputHandler {
            input: InputType::Continue,
        }
    }

    fn handle_input(&mut self, state: &mut State) -> Result<InputType> {
        let c: i32 = getch();
        self.input = if c == ArrowKeys::Down as i32 || c == 'j' as i32 {
            InputType::Down
        } else if c == ArrowKeys::Up as i32 || c == 'k' as i32 {
            InputType::Up
        } else {
            match c as u8 as char {
                'q' => InputType::Quit,
                'a' => InputType::IncreaseTimeout,
                's' => InputType::DecreaseTimeout,
                _ => InputType::Continue,
            }
        };

        match self.input {
            InputType::Quit | InputType::Continue => (),
            InputType::IncreaseTimeout => {
                // Increase timeout
                if state.timeout < 1000 {
                    state.timeout += 10;
                }
                timeout(state.timeout);
            }
            InputType::DecreaseTimeout => {
                // Decrease timeout
                if state.timeout > 10 {
                    state.timeout -= 10;
                }
                timeout(state.timeout);
            }
            _ => (),
        }

        Ok(self.input)
    }
}

#[derive(Parser)]
pub struct Cli {
    /// Timeout in milliseconds
    #[clap(short = 't', long = "timeout", default_value = "500")]
    timeout: i32,
    /// What character to use to draw each cell
    #[clap(short = 'c', long = "character", default_value = "▓")]
    character: char,
}

pub struct State {
    timeout: i32,
    draw_char: char,
}

impl State {
    pub fn new(timeout: i32, draw_char: char) -> State {
        State { timeout, draw_char }
    }

    pub fn get_timeout(&self) -> i32 {
        self.timeout
    }

    pub fn get_draw_char(&self) -> char {
        self.draw_char
    }

    pub fn set_timeout(&mut self, timeout: i32) {
        self.timeout = timeout;
    }

    pub fn set_draw_char(&mut self, draw_char: char) {
        self.draw_char = draw_char;
    }
}

fn main() {

    let mut args = Cli::parse();

    ncurses::setlocale(LcCategory::all, "");

    /* initialize screen */
    initscr();

    /* enables colors */
    start_color();

    /* hide cursor */
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    /* initially refreshes screen, emptying it */
    refresh();

    /* keypresses will not be displayed on screen */
    noecho();

    /* 
     * sets getch timeout (and I assume other hanging operations' timeouts, but 
     * I haven't confirmed that). This is used for the timing functionality
     */
    timeout(args.timeout as i32);

    /* get the number of rows and columns */
    let nrows: usize = LINES() as usize - 1;
    let ncols: usize = COLS() as usize - 1;
    let mut input_handler: InputHandler = InputHandler::new();

    let mut win: Window = Window::new(nrows as i32, ncols as i32, 0, 0);
    let mut state = State::new(args.timeout, args.character);

    let grid_width: usize = 100;
    let grid_height: usize = 100;
    let mut grid: Vec<Cell> = Vec::new();
    let grid_loc_x = 0 as usize; //TODO add bounds no less than 0
    let grid_loc_y = 0 as usize; //TODO add bounds no less than 0, no more than 
    let mut char_grid: Vec<char> = vec![' '; nrows * ncols];

    for i in 0..nrows {
        for j in 0..ncols {
            grid.push(Cell::new(CellValue::Empty, j as i32, i as i32));
        }
    }

    /* Load coffee */
    let contents = fs::read_to_string("coffee.txt").expect("Something went wrong reading the file");
    for (i, line) in contents.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            let x = j; // +1 to account for the border
            let y = i; // +1 to account for the border
            match (y as i32) % 2 {
                0 => grid[y*ncols+x].value = CellValue::Scene('E'),
                1 => grid[y*ncols+x].value = CellValue::Scene('O'),
                _ => grid[y*ncols+x].value = CellValue::Scene('?'),
            };
            grid[y*ncols+x].value = CellValue::Scene(c);
            // grid[y*ncols+x].value = CellValue::Scene('.');
        }
    }
    // for i in 0..8{
    //     grid[18*ncols+i+29].value = CellValue::Steam(4);
    // }
    // for i in 0..24{
    //     grid[17*ncols+i+37].value = CellValue::Steam(4);
    // }
    // for i in 0..8{
    //     grid[18*ncols+i+61].value = CellValue::Steam(4);
    // }
    /* End Coffee */

    loop {

        win.erase();

        // 1. Calc char_grid
        for i in 0..nrows {
            for j in 0..ncols {
                //TODO Bound for moement
                let cell = &grid[(i + grid_loc_y) * nrows + (j + grid_loc_x)];
                // char_grid[i * ncols + j] = match (j as i32)%2 {
                //     0 => ' ',
                //     1 => '█',
                //     _ => 'X',
                // };
                char_grid[i * ncols + j] = match cell.value {
                    cellular::CellValue::Empty => 'X',
                    cellular::CellValue::Steam(4) => '█',
                    cellular::CellValue::Steam(3) => '▓',
                    cellular::CellValue::Steam(2) => '▒',
                    cellular::CellValue::Steam(1) => '░',
                    cellular::CellValue::Scene(c) => c,
                    _ => ' ',
                };
            }
        }

        // TODO: Better hadle inputs
        // 2. Draw
        draw(&mut win, &char_grid,  nrows, ncols);

        // 3. Update
        // grid = update(&grid, nrows, ncols);

        // 4. Refresh
        win.refresh();

        // TODO: Handle input better
        // 5. Handle input
        let input: InputType = input_handler.handle_input(&mut state).expect("Input error");

        if input == InputType::Quit {
            break;
        }
    }

    endwin();
}
