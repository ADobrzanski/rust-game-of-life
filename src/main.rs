use coffee::graphics::{Color, Frame, Window, WindowSettings, Mesh, Shape, Rectangle};
use coffee::load::Task;
use coffee::{Game, Result, Timer};
use rand::{ Rng, thread_rng };


fn main() -> Result<()> {
    MyGame::run(WindowSettings {
        title: String::from("A caffeinated game"),
        size: (1280, 1024),
        resizable: true,
        fullscreen: false,
        maximized: false,
    })
}

struct GameMatrix ([[bool; 100]; 100]);

impl GameMatrix {
    pub fn new() -> GameMatrix {
        GameMatrix([[false; 100];100])
    }

    pub fn iter_cells(&self) -> impl Iterator<Item = ((usize, usize), &bool)> {
        let GameMatrix(matrix) = self;
        matrix.iter().enumerate()
            .flat_map(|(x, col)| col.iter().enumerate().map(move |(y, cell)| ((x, y), cell)))
    }
}

fn get_random_boolean(true_chance: f32) -> bool {
    let mut rng = thread_rng();
    (rng.gen_range(0.0..1.0) < true_chance) as bool
}


struct MyGame {
   // Your game state and assets go here...
  matrix: [[bool; 100]; 100],
}

impl Game for MyGame {
    type Input = (); // No input data
    type LoadingScreen = (); // No loading screen

    fn load(_window: &Window) -> Task<MyGame> {
        // Load your game assets here. Check out the `load` module!

        // Init (random) state
        let game_matrix = GameMatrix::new();
        let GameMatrix(mut raw_matrix) = game_matrix;

        for ((x, y), _) in game_matrix.iter_cells() {
            raw_matrix[x][y] = get_random_boolean(0.3)
        }

        Task::succeed(move || MyGame { matrix: raw_matrix })
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        // Clear the current frame
        frame.clear(Color::BLACK);

        // Draw your game here. Check out the `graphics` module!

        // Draw state as Rectangle matrix
        let mut mesh = Mesh::new();

        for col_idx in 0..self.matrix.len() {
            for row_idx in 0..self.matrix[col_idx].len() {
                let cell_shape  = Shape::Rectangle(Rectangle {
                    x: col_idx as f32 * 50.0,
                    y: row_idx as f32 * 50.0,
                    width: 50.0,
                    height: 50.0,
                });

                if self.matrix[col_idx][row_idx] {
                    mesh.fill(cell_shape, Color::WHITE);
                }
            }
        }

        mesh.draw(&mut frame.as_target());
    }

}
