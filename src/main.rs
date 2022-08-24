use coffee::graphics::WindowSettings;
use coffee::ui::UserInterface;
use coffee::Result;

mod matrix;
mod changes;
mod game;

use game::MyGame;


fn main() -> Result<()> {
    <MyGame as UserInterface>::run(WindowSettings {
        title: String::from("Game of Rust"),
        size: (1280, 1024),
        resizable: true,
        fullscreen: false,
        maximized: false,
    })
}





