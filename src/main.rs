use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
    terminal::{Clear, ClearType},
};
use rand::Rng;
use std::io::stdout;

fn clear_screen() -> Result<(), std::io::Error> {
    let mut stdout = stdout();
    // 隐藏光标，避免闪烁
    execute!(stdout, Hide)?;

    execute!(stdout, Clear(ClearType::All))?; // 清除屏幕

    Ok(())
}

fn dead_state(width: usize, height: usize) -> Vec<Vec<usize>> {
    vec![vec![0; width]; height]
}

fn random_state(width: usize, height: usize) -> Vec<Vec<usize>> {
    let mut state = dead_state(width, height);

    let mut rng = rand::thread_rng();

    for row in state.iter_mut() {
        for n in row.iter_mut() {
            let random_bool = rng.gen_bool(0.5);
            *n = if random_bool { 1 } else { 0 };
        }
    }

    state
}

fn get_alive_size(i: i32, j: i32, state: &Vec<Vec<usize>>) -> u32 {
    let mut alive_size = 0;

    for x in i - 1..=i + 1 {
        for y in j - 1..=j + 1 {
            if x < 0
                || x >= state.len() as i32
                || y < 0
                || y >= state[0].len() as i32
                || (x == i && y == j)
            {
                continue;
            }
            if state[x as usize][y as usize] == 1 {
                alive_size += 1;
            }
        }
    }

    alive_size
}

// rules
// 任何有 0 个或 1 个活邻居的活细胞都会因细胞数量不足而死亡
// 任何有 2 或 3 个活邻居的活细胞都能存活，因为它的邻居恰好合适
// 任何有超过 3 个活邻居的活细胞都会因过度繁殖而死亡
// 任何有 3 个活邻居的死细胞都会通过繁殖而复活
fn next_board_state(state: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut next_state = state.clone();

    for (i, row) in state.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            let alive_size = get_alive_size(i as i32, j as i32, &state);
            if !(2..=3).contains(&alive_size) {
                next_state[i][j] = 0;
            } else if alive_size == 3 && state[i][j] == 0 {
                next_state[i][j] = 1;
            }
        }
    }

    next_state
}

fn render(state: &Vec<Vec<usize>>) {
    let mut stdout = stdout();
    execute!(stdout, MoveTo(0, 0));

    let DEAD = " ";
    let ALIVE = "#";

    for (i, row) in state.iter().enumerate() {
        if i == 0 {
            for _ in 0..row.len() + 2 {
                print!("-");
            }
            print!("\r\n");
        }

        for (j, n) in row.iter().enumerate() {
            if j == 0 {
                print!("|");
            }

            if *n == 1 {
                print!("{}", ALIVE);
            } else if *n == 0 {
                print!("{}", DEAD);
            }

            if j == row.len() - 1 {
                print!("|");
            }
        }
        print!("\r\n");

        if i == state.len() - 1 {
            for _ in 0..row.len() + 2 {
                print!("-");
            }
            print!("\r\n");
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    // let dead_state = dead_state(40, 30);
    // render(&dead_state);
    // render(&next_board_state(&dead_state));

    // render(&random_state);
    // render(&next_board_state(&random_state));

    let mut random_state = random_state(35, 25);
    loop {
        clear_screen()?;
        render(&random_state);
        random_state = next_board_state(&random_state);
        std::thread::sleep(std::time::Duration::from_secs_f32(0.1)); // 暂停一秒
    }
    Ok(())
}
