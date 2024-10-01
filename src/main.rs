use rand::Rng;

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

fn render(state: Vec<Vec<usize>>) {
    let DEAD = "#";
    let ALIVE = "@";

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

fn main() {
    let dead_state = dead_state(40, 30);
    render(dead_state);
    let random_state = random_state(20, 30);
    render(random_state);
}
