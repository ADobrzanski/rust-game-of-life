use coffee::graphics::{Color, Frame, Window, Mesh, Point};
use coffee::ui::{Align, Column, Element, Justify, Renderer, Text, UserInterface};
use coffee::input::KeyboardAndMouse;
use coffee::input::keyboard::KeyCode;
use coffee::input::mouse::Button;
use coffee::load::Task;
use coffee::{Game, Timer};

mod helpers;
use helpers::*;
use crate::matrix::*;
use crate::matrix::helpers::*;

pub struct MyGame<const W: usize = 100, const H: usize = 100> {
   // Your game state and assets go here...
    game_matrix: GameMatrix<W, H>,
    sim_playing: bool,
    sim_speed: u16,
    tick: u16,
    cursor: Option<Point>,
    world_offset: Point,
    cell_size: f32,
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
        let game_matrix = init_random_game_state(0.1);

        Task::succeed(move || MyGame {
            game_matrix,
            sim_playing: false,
            sim_speed: 1,
            tick: 0,
            cursor: None,
            world_offset: Point::new(0.0, 0.0),
            cell_size: 10.0,
        })
    }

    fn interact(&mut self, input: &mut Self::Input, _window: &mut Window) {
        // handle manual cell placement
        if !self.sim_playing {
            for click in input.mouse().button_clicks(Button::Left) {
                let (x,y) = window_pos_to_point_xy(&self.cell_size, &self.world_offset, &click);
                let _ = self.game_matrix.try_set_alive(x, y, !self.game_matrix.is_cell_alive(x, y));
            }
        }

        // handle play/pause
        if input.keyboard().was_key_released(KeyCode::Space) {
           self.sim_playing = !self.sim_playing;
        }

        // handle sim speed changes
        let new_sim_speed = ((self.sim_speed as f32) + input.mouse().wheel_movement().vertical).round();

        self.sim_speed = if new_sim_speed < 1.0 { 1u16 }
            else if new_sim_speed > MyGame::TICKS_PER_SECOND as f32 { MyGame::TICKS_PER_SECOND }
            else { new_sim_speed as u16 };

        // handle restart
        if input.keyboard().was_key_released(KeyCode::R) {
            self.game_matrix = init_random_game_state(0.1);
        }

        // handle cursor position
        self.cursor = if input.mouse().is_cursor_within_window() {
            Some(input.mouse().cursor_position())
        } else {
            None
        }
    }

    fn update(&mut self, _window: &Window) {
        if !self.sim_playing { self.tick = 0; return; }

        // Skip sim step if too early
        let ticks_between_updates = MyGame::TICKS_PER_SECOND.div_euclid(self.sim_speed);
        if self.tick >= ticks_between_updates { self.tick = 0; }
        else { self.tick = self.tick + 1; return; }

        self.game_matrix = make_next_sim_state(&self.game_matrix);
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        // NOTE - improve performance with partial draw
        self.cell_size = calc_cell_size(&self.game_matrix, &frame);
        self.world_offset = calc_window_offset(&self.game_matrix, &self.cell_size, &frame);

        let mut mesh = Mesh::new();

        draw_matrix_state(&self.game_matrix, &self.cell_size, &self.world_offset, &mut mesh);
        if let Some(point) = self.cursor {
            draw_cursor(&self.game_matrix, &self.cell_size, &self.world_offset, &point, &mut mesh);
        }

        frame.clear(Color::BLACK);
        mesh.draw(&mut frame.as_target());
    }
}
