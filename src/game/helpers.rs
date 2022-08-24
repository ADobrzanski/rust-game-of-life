use coffee::graphics::{Color, Frame, Mesh, Shape, Point};
use crate::matrix::GameMatrix;

pub fn make_cell_shape(cell_size: &f32, world_offset: &Point, point_xy: &(usize, usize)) -> Shape {
    let (x, y) = point_xy;
    Shape::Circle {
        radius: cell_size / 2.2,
        center: Point::new(
            world_offset.x + (*x as f32 * cell_size) + cell_size / 2.2,
            world_offset.y + (*y as f32 * cell_size) + cell_size / 2.2
        ),
    }
}

pub fn draw_matrix_state<const W: usize, const H: usize>(
    game_matrix: &GameMatrix<W, H>,
    cell_size: &f32,
    world_offset: &Point,
    mesh: &mut Mesh
) {
    for (cell_xy, cell_is_alive) in game_matrix.iter_cells() {
        let cell_shape = make_cell_shape(&cell_size, &world_offset, &cell_xy);

        if *cell_is_alive {
            mesh.fill(cell_shape, Color::GREEN);
        }
    }
}

pub fn draw_cursor<const W: usize, const H: usize>(
    game_matrix: &GameMatrix<W, H>,
    cell_size: &f32,
    world_offset: &Point,
    point: &Point,
    mesh: &mut Mesh
) {
    let (x, y) = window_pos_to_point_xy(&cell_size, &world_offset, &point);
    let pointer_shape = make_cell_shape(&cell_size, &world_offset, &(x, y));
    let pointer_color = if game_matrix.is_cell_alive(x, y) { Color::WHITE } else { Color::from_rgb(200, 200, 200) };
    mesh.fill(pointer_shape, pointer_color);
}

pub fn window_pos_to_point_xy(cell_size: &f32, world_offset: &Point, point: &Point) -> (usize, usize) {
    let x = (point.coords.x - world_offset.x).div_euclid(*cell_size);
    let y = (point.coords.y - world_offset.y).div_euclid(*cell_size);
    (x as usize, y as usize)
}

pub fn calc_cell_size<const W: usize, const H: usize>(game_matrix: &GameMatrix<W, H>, frame: &Frame) -> f32 {
    let max_cell_height = frame.height() / (game_matrix.height() as f32);
    let max_cell_width = frame.width() / (game_matrix.width() as f32);
    max_cell_height.min(max_cell_width)
}

pub fn calc_window_offset<const W: usize, const H: usize>(game_matrix: &GameMatrix<W, H>, cell_size: &f32, frame: &Frame) -> Point {
    let left_offset = (frame.width() - (game_matrix.width() as f32 * cell_size)) / 2.0;
    let top_offset = (frame.height() - (game_matrix.height() as f32 * cell_size)) / 2.0;
    Point::new(left_offset, top_offset)
}

pub fn make_next_sim_state<const W: usize, const H: usize>(game_matrix: &GameMatrix<W, H>) -> GameMatrix<W,H>{
    let new_game_matrix: GameMatrix<W, H> = GameMatrix::default();
    let GameMatrix(mut raw_matrix) = new_game_matrix;

    for ((x, y), _) in game_matrix.iter_cells() {
        raw_matrix[x][y] = should_cell_be_alive(&game_matrix, (x, y));
    }

    GameMatrix(raw_matrix)
}

pub fn should_cell_be_alive<const W: usize, const H: usize>(state: &GameMatrix<W, H>, cell: (usize, usize)) -> bool {
    let (x, y) = cell;

    let left_coord = x.checked_sub(1);
    let right_coord = x.checked_add(1);
    let top_coord = y.checked_sub(1);
    let bottom_coord = y.checked_add(1);

    let neighbors_coords = [
        (left_coord, top_coord),
        (left_coord, Some(y)),
        (left_coord, bottom_coord),
        (Some(x), top_coord),
        (Some(x), bottom_coord),
        (right_coord, top_coord),
        (right_coord, Some(y)),
        (right_coord, bottom_coord),
    ];

    let neighbors_alive = neighbors_coords
        .iter().fold(0, |acc, (maybe_x, maybe_y)| {
            if let (Some(x), Some(y)) = (maybe_x, maybe_y) {
                if state.is_cell_alive(*x, *y) { return acc + 1; }
            } 
            return acc;
        });

   
    if state.is_cell_alive(x,y) {
        !(neighbors_alive < 2 || neighbors_alive > 3)
    } else {
        neighbors_alive == 3
    }
}
