use rand::Rng;

fn dead_state(width: usize, height: usize) -> Vec<Vec<usize>> {
    vec![vec![0; width]; height]
}

fn radom_state(width: usize, height: usize) -> Vec<Vec<usize>> {
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

fn main() {
    let width = 5;
    let height = 6;
    println!("{:?}", radom_state(width, height));
}
