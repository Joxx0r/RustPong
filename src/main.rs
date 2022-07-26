use tetra::graphics::{self, Color, Texture};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;
const PADDLE_SPEED: f32 = 8.0;

struct GameState{
    paddle_texture: Texture,
    paddle_position: Vec2<f32>,
    paddle_rotation_angle: f32,
}

impl GameState {
    fn new(ctx: &mut Context ) -> tetra::Result <GameState> {
        let mut paddle_texture = Texture::new(ctx, "./resources/paddleBlu.png")?;
        let paddle_position =
        Vec2::new(32.0, (WINDOW_HEIGHT - paddle_texture.height() as f32) / 2.0);
        let paddle_rotation_angle = 1.5705;
        Ok(GameState{ paddle_texture, paddle_position, paddle_rotation_angle })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context)  -> tetra::Result {
        if input::is_key_down(ctx, Key::W) {
            self.paddle_position.y -= PADDLE_SPEED;
        }
        if input::is_key_down(ctx, Key::S) {
            self.paddle_position.y += PADDLE_SPEED;
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.0, 0.0, 0.0));
        let params:tetra::graphics::DrawParams = graphics::DrawParams { position: self.paddle_position, rotation: self.paddle_rotation_angle, ..Default::default()};
        self.paddle_texture.draw(ctx, params );
        Ok(())
    }
}

fn main() -> tetra::Result {
  
    ContextBuilder::new("Pong", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
    .quit_on_escape(true)
    .build()?
    .run(GameState::new)
}

