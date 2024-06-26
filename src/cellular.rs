use rand::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum CellValue {
    Empty,
    Steam(i8),
    Scene(char)
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub value: CellValue,
    pub x: i32,
    pub y: i32
}

impl Cell {
    pub fn new(value: CellValue, x: i32, y: i32) -> Cell {
        Cell { value, x, y }
    }

    // fn update(&mut self) {
    //     match self.value {
    //         CellValue::Steam(steam) => {
    //             if steam > 0 {
    //                 self.value = CellValue::Steam(steam - 1);
    //             } else {
    //                 self.value = CellValue::Empty;
    //             }
    //         },
    //         _ => {}
    //     }
    // }

    pub fn update_value(&mut self, new_value: CellValue) {
        self.value = new_value;
    }
}

pub fn update(grid: &[Vec<Cell>], nrows: usize, ncols: usize) -> Vec<Vec<Cell>> {
    let mut rng = rand::thread_rng();

    let mut new_grid: Vec<Vec<Cell>> = grid.to_vec().clone();
    for i in 1..nrows-1 {
        for j in 1..ncols-1 {
            let val: CellValue = grid[i][j].value;
            match val {
                // Update rules for steam
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
                    new_grid[(i - 1)][j].update_value(newval);
                }   
                _ => {continue}
            }
        }
    }
    new_grid
}
