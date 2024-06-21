use std::{f32::consts::E, fs};
use clap::Parser;
use ncurses::*;
use rand::prelude::*;


#[derive(Parser)]
pub struct Cli {
    /// Timeout in milliseconds
    #[clap(short = 't', long = "timeout", default_value = "500")]
    timeout: i32,
    /// What character to use to draw each cell
    #[clap(short = 'c', long = "character", default_value = "▓")]
    character: char,
}

#[derive(Debug, Clone, Copy)]
enum CellValue {
    Empty,
    Steam(i8),
    Scene(char)
}

struct Cell {
    value: CellValue,
    x: i32,
    y: i32
}

impl Cell {
    fn new(value: CellValue, x: i32, y: i32) -> Cell {
        Cell { value, x, y }
    }

    fn update(&mut self) {
        match self.value {
            CellValue::Steam(steam) => {
                if steam > 0 {
                    self.value = CellValue::Steam(steam - 1);
                } else {
                    self.value = CellValue::Empty;
                }
            },
            _ => {}
        }
    }

    fn update_value(&mut self, new_value: CellValue) {
        self.value = new_value;
    }
}

fn draw(grid: &Vec<Cell>, nrows: usize, ncols: usize) {
    for i in 0..nrows {
        for j in 0..ncols {
            let output = format!(
                "{}",
                    match grid[i * ncols + j].value {
                        CellValue::Empty => ' ',                                    
                        CellValue::Steam(4) => '█',
                        CellValue::Steam(3) => '▓',
                        CellValue::Steam(2) => '▒',
                        CellValue::Steam(1) => '░',
                        CellValue::Scene(c) => c,
                        _ => ' ',
                    }
            );
            let _ = addstr(&output);
        }
        let _ = addch('\n' as u32);
    }
}

fn update(grid: &mut Vec<Cell>, nrows: usize, ncols: usize) {
    let mut rng = rand::thread_rng();

    for i in 1..nrows-1 {
        for j in 1..ncols-1 {
            let val: CellValue = grid[i * ncols + j].value;
            match val {
                CellValue::Steam(_) => {
                    let p: f64 = rng.gen(); 
                    let newval: CellValue = match val {
                            CellValue::Steam(4) => {
                                if p < 0.5 {
                                    CellValue::Steam(4)
                                } else {
                                    CellValue::Steam(3)
                                }
                            },
                            CellValue::Steam(3) => {
                                if p < 0.5 {
                                    CellValue::Steam(3)
                                } else {
                                    CellValue::Steam(2)
                                }
                            },
                            CellValue::Steam(2) => {
                                if p < 0.5 {
                                    CellValue::Steam(2)
                                } else {
                                    CellValue::Steam(1)
                                }
                            },
                            CellValue::Steam(1) => {
                                if p < 0.5 {
                                    CellValue::Steam(1)
                                } else {
                                    CellValue::Empty
                                }
                            },
                            _ => val,
                        };
                    grid[(i - 1) * ncols + j].update_value(newval);
                }   
                _ => {continue}
            }
        }
    }
}

fn main() {

    let mut args = Cli::parse();

    ncurses::setlocale(LcCategory::all, "");

    /* initialize screen */
    initscr();

    /* enables colors */
    start_color();

    /* initially refreshes screen, emptying it */
    refresh();

    /* keypresses will not be displayed on screen */
    noecho();

    /* 
     * sets getch timeout (and I assume other hanging operations' timeouts, but 
     * I haven't confirmed that). This is used for the timing functionality
     */
    timeout(args.timeout as i32);

    let mut c = 0 as char;

    const NROWS: usize = 50;
    const NCOLS: usize = 99;
    let mut grid: Vec<Cell> = Vec::new();

    // for i in 0..NROWS {
    //     for j in 0..NCOLS {
    //         // /2 to account for space between characters
    //         grid.push(Cell::new(CellValue::Empty, j as i32, i as i32));
    //     }
    // }
    for i in 0..NROWS {
        for j in 0..NCOLS {
            grid.push(Cell::new(CellValue::Empty, j as i32, i as i32));
        }
    }
    let contents = fs::read_to_string("coffee.txt").expect("Something went wrong reading the file");
    for (i, line) in contents.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            let x = j + 1; // +1 to account for the border
            let y = i + 1; // +1 to account for the border
            grid[y*NCOLS+x].value = CellValue::Scene(c);
        }
    }
    // let nrows: usize = window.get_rows() as usize - 1; // -1 to account for status bar at bottom
    // let ncols: usize = window.get_cols() as usize;
    
    for i in 0..8{
        grid[17*NCOLS+i+29].value = CellValue::Steam(4);
    }
    for i in 0..24{
        grid[16*NCOLS+i+37].value = CellValue::Steam(4);
    }
    for i in 0..8{
        grid[17*NCOLS+i+61].value = CellValue::Steam(4);
    }

    loop {
    
        erase();
    
        draw(&grid, NROWS, NCOLS);

        update(&mut grid, NROWS, NCOLS);
        
        refresh();
        
        // TODO: Remove this call to getch. It's just for hanging between frames.
        getch();
        let c = getch();
        match c as u8 as char {
            'q' => break,
             _ => continue,
        }   
       
    }
   
    endwin();
}
