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

struct GameMatrix<const W: usize, const H: usize> ([[bool; H]; W]);

impl<const W: usize, const H: usize> GameMatrix<W, H> {
    pub fn new() -> GameMatrix<W, H> {
        GameMatrix([[false; H]; W])
    }

    pub fn width(&self) -> usize {
        W
    }

    pub fn height(&self) -> usize {
        H
    }

    pub fn iter_cells(&self) -> impl Iterator<Item = ((usize, usize), &bool)> {
        let GameMatrix(matrix) = self;
        matrix
            .iter().enumerate()
            .flat_map(|(x, col)| col
                .iter().enumerate()
                .map(move |(y, cell)| ((x, y), cell))
            )
    }
}

fn get_random_boolean(true_chance: f32) -> bool {
    let mut rng = thread_rng();
    (rng.gen_range(0.0..1.0) < true_chance) as bool
}


struct MyGame {
   // Your game state and assets go here...
    screen_width: f32,
    screen_height: f32,
    game_matrix: GameMatrix<100, 100>,
}

impl Game for MyGame {
    type Input = (); // No input data
    type LoadingScreen = (); // No loading screen

    fn load(_window: &Window) -> Task<MyGame> {
        // Load your game assets here. Check out the `load` module!

        // Init (random) state
        let game_matrix: GameMatrix<100, 100> = GameMatrix::new();
        let GameMatrix(mut raw_matrix) = game_matrix;

        for ((x, y), _) in game_matrix.iter_cells() {
            raw_matrix[x][y] = get_random_boolean(0.3)
        }

        let screen_height = _window.width();
        let screen_width = _window.height();

        Task::succeed(move || MyGame {
            game_matrix: GameMatrix(raw_matrix),
            screen_width,
            screen_height,
        })
    }

    fn update(&mut self, _window: &Window) {
        self.screen_width = _window.width();
        self.screen_height = _window.height();
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        // Clear the current frame
        frame.clear(Color::BLACK);

        // Draw your game here. Check out the `graphics` module!

        // Calc cell size and offset
        let max_cell_height = self.screen_height / (self.game_matrix.height() as f32);
        let max_cell_width = self.screen_width / (self.game_matrix.width() as f32);
        let cell_size =
            if max_cell_width < max_cell_height { max_cell_width }
            else { max_cell_height };

        let left_offset = (self.screen_width - (self.game_matrix.width() as f32 * cell_size)) / 2.0;
        let top_offset = (self.screen_height - (self.game_matrix.height() as f32 * cell_size)) / 2.0;

        // Prepare and draw mesh based on matrix, cell_size and offests
        let mut mesh = Mesh::new();

        for ((x, y), cell_is_alive) in self.game_matrix.iter_cells() {
            let cell_shape  = Shape::Rectangle(Rectangle {
                x: left_offset + x as f32 * cell_size,
                y: top_offset + y as f32 * cell_size,
                width: cell_size,
                height: cell_size,
            });

            if *cell_is_alive {
                mesh.fill(cell_shape, Color::WHITE);
            }
        }


        mesh.draw(&mut frame.as_target());
    }

}
