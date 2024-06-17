use std::fs;

use ncursesw::*;
use gettextrs::*;

fn draw(grid: &[char], nrows: usize, ncols:usize) {
    for i in 0..nrows {
        for j in 0..ncols {
            
            //let ch = ChtypeChar::from('░' as u32);
            let output: &str = &format!("{}", grid[i*ncols + j]);
            let woutput = WideString::from(output);
            addwstr(&woutput);
        }
        let _ = addstr("\n");
    }

}

fn main() {

    let time = 20000;

    // raw();
    // keypad(stdscr(), true); https://docs.rs/ncursesw/latest/ncursesw/shims/bindings/fn.setlocale.html
    setlocale(LocaleCategory::LcAll, "");

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
    timeout(std::time::Duration::from_millis(time as u64));

    let mut c = CharacterResult::Character('0');

    const NROWS: usize = 49;
    const NCOLS: usize = 97;
    let mut grid = vec![0 as char; NROWS * NCOLS];

    let contents = fs::read_to_string("coffee.txt").expect("Something went wrong reading the file");
    for (i, line) in contents.lines().enumerate() {
        let mut count = 0;
        for (j, c) in line.chars().enumerate() {
            grid[i * NCOLS + j] = c;
            count = count + 1;
        }
        // let line_count = format!("{}: {}\n", i as i32, count);
        // let _ = addstr(&line_count);
    }

    loop {
    
        erase();
    
        // let _ = addstr(&format!("You pressed: {}", c));
        // let random_string: String = (0..20)
        //     .map(|_| rand::thread_rng().gen_range(33..127) as u8 as char)
        //     .collect();
        // let _ = addstr(&format!("Random String: {}", random_string));
    
        // draw(&grid, NROWS, NCOLS);
    
        draw(&grid, NROWS, NCOLS);
    
        refresh();
        // TODO: Remove this call to getch. It's just for hanging between frames.
        getch();
        match getch() {
            Err(e) => {
                println!("{}", e);
                continue;
            },
            Ok(c_ret) => {
                c = c_ret;
                if c == CharacterResult::Character('q') {
                    break;
                }
            }
        }

    }
   
    endwin();
}

// enum Num {
//     N(i32),
//     Imaginary,
//     NaN
// }

// fn myfun() -> Num {
//     Num::NaN
// }

// fn isanumber() -> bool {
//     let result = match myfun() {
//         Num::N(_number) => { true },
//         Num::Imaginary | Num::NaN => { false }
//     };
//     result
// }