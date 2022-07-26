use tetra::graphics::{self, Color, Texture};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};

struct GameState{}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(1.0, 0.0, 0.0));
        Ok(())
    }
}

fn main() -> tetra::Result {
 
    ContextBuilder::new("Pong", 640, 480)
    .quit_on_escape(true)
    .build()?
    .run(|_| Ok(GameState{}))
}

