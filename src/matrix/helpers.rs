use rand::{ Rng, thread_rng };

use crate::matrix::GameMatrix;

fn get_random_boolean(true_chance: f32) -> bool {
    let mut rng = thread_rng();
    (rng.gen_range(0.0..1.0) < true_chance) as bool
}

pub fn init_random_game_state<const W: usize, const H: usize>(fill_factor: f32) -> GameMatrix<W, H> {
    // Init (random) state
    let game_matrix: GameMatrix<W, H> = GameMatrix::default();
    let GameMatrix(mut raw_matrix) = game_matrix;

    for ((x, y), _) in game_matrix.iter_cells() {
        raw_matrix[x][y] = get_random_boolean(fill_factor);
    }

    GameMatrix(raw_matrix)
}

