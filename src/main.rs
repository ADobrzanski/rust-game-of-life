use coffee::graphics::{Color, Frame, Window, WindowSettings, Mesh, Shape, Rectangle};
use coffee::ui::{Align, Column, Element, Justify, Renderer, Text, UserInterface};
use coffee::load::Task;
use coffee::{Game, Result, Timer};

use coffee::input::KeyboardAndMouse;
use coffee::input::keyboard::KeyCode;

use rand::{ Rng, thread_rng };

mod state;
use state::GameMatrix;

fn main() -> Result<()> {
    <MyGame as UserInterface>::run(WindowSettings {
        title: String::from("Game of Rust"),
        size: (1280, 1024),
        resizable: true,
        fullscreen: false,
        maximized: false,
    })
}


fn get_random_boolean(true_chance: f32) -> bool {
    let mut rng = thread_rng();
    (rng.gen_range(0.0..1.0) < true_chance) as bool
}

fn should_cell_be_alive(state: &GameMatrix<100, 100>, cell: (usize, usize)) -> bool {
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

fn init_random_game_state() -> GameMatrix<100, 100> {
    // Init (random) state
    let game_matrix: GameMatrix<100, 100> = GameMatrix::new();
    let GameMatrix(mut raw_matrix) = game_matrix;

    for ((x, y), _) in game_matrix.iter_cells() {
        raw_matrix[x][y] = get_random_boolean(0.1)
    }

    GameMatrix(raw_matrix)
}


struct MyGame {
   // Your game state and assets go here...
    game_matrix: GameMatrix<100, 100>,
    sim_playing: bool,
    sim_speed: u16,
    tick: u16,
}

impl UserInterface for MyGame {
    type Message = ();
    type Renderer = Renderer;

    fn react(&mut self, _message: Self::Message, _window: &mut Window) {}

    fn layout(&mut self, window: &Window) -> Element<()> {
        let game_stats = Column::new()
            .width(200)
            .align_items(Align::Start)
            .justify_content(Justify::Start)
            .spacing(8)
            .push(
                if self.sim_playing { Text::new("► PLAYING") }
                else { Text::new("▌▌PAUSED") })
            .push(Text::new(&format!("SPEED: x{}", self.sim_speed)));

        let game_ctrls = Column::new()
            .align_items(Align::End)
            .justify_content(Justify::End)
            .spacing(8)
            .push(Text::new("scroll - inc/dec sim speed"))
            .push(Text::new("space - play/pause sim"));

        Column::new()
            .height(window.height() as u32)
            .push(game_stats)
            .push(game_ctrls)
            .justify_content(Justify::SpaceBetween)
            .into()
    }
}

impl Game for MyGame {
    // updates per second; not sure about draws;
    // for the time being used as sim_speed top cap
    const TICKS_PER_SECOND: u16 = 60; 

    type Input = KeyboardAndMouse;
    type LoadingScreen = (); // No loading screen

    fn load(_window: &Window) -> Task<MyGame> {
        // Load your game assets here. Check out the `load` module!
        let game_matrix = init_random_game_state();

        Task::succeed(move || MyGame {
            game_matrix,
            sim_playing: false,
            sim_speed: 1,
            tick: 0,
        })
    }

    fn interact(&mut self, input: &mut Self::Input, _window: &mut Window) {
        // handle play/pause
        if input.keyboard().was_key_released(KeyCode::Space) {
           self.sim_playing = !self.sim_playing;
        }

        // handle sim speed changes
        let new_sim_speed = ((self.sim_speed as f32) + input.mouse().wheel_movement().vertical).round();

        self.sim_speed = if new_sim_speed < 1.0 { 1u16 }
            else if new_sim_speed > MyGame::TICKS_PER_SECOND as f32 { MyGame::TICKS_PER_SECOND }
            else { new_sim_speed as u16 };

        // randle restart
        if input.keyboard().was_key_released(KeyCode::R) {
            self.game_matrix = init_random_game_state();
        }
    }

    fn update(&mut self, _window: &Window) {

        // Skip sim step if paused
        if !self.sim_playing { self.tick = 0; return; }

        // 
        let ticks_between_updates = MyGame::TICKS_PER_SECOND.div_euclid(self.sim_speed);
        if self.tick >= ticks_between_updates { self.tick = 0; }
        else { self.tick = self.tick + 1; return; }

        // Simulation step
        let new_game_matrix: GameMatrix<100, 100> = GameMatrix::new();
        let GameMatrix(mut raw_matrix) = new_game_matrix;

        for ((x, y), _) in self.game_matrix.iter_cells() {
            raw_matrix[x][y] = should_cell_be_alive(&self.game_matrix, (x, y));
        }

        self.game_matrix = GameMatrix(raw_matrix);
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        // Clear the current frame
        frame.clear(Color::BLACK);

        // Draw your game here. Check out the `graphics` module!

        let game_matrix = &self.game_matrix;

        // Calc cell size and offset
        let max_cell_height = frame.height() / (game_matrix.height() as f32);
        let max_cell_width = frame.width() / (game_matrix.width() as f32);
        let cell_size = max_cell_height.min(max_cell_width);

        let left_offset = (frame.width() - (game_matrix.width() as f32 * cell_size)) / 2.0;
        let top_offset = (frame.height() - (game_matrix.height() as f32 * cell_size)) / 2.0;

        // Prepare and draw mesh based on matrix, cell_size and offests
        let mut mesh = Mesh::new();

        for ((x, y), cell_is_alive) in game_matrix.iter_cells() {
            let cell_shape = Shape::Rectangle(Rectangle {
                x: left_offset + (x as f32 * cell_size),
                y: top_offset + (y as f32 * cell_size),
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
