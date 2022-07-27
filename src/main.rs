use std::panic::Location;

use tetra::graphics::{self, Color, Texture};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;
const PADDLE_SPEED: f32 = 8.0;

struct Entity {
    texture:Texture,
    location: Vec2<f32>,
    rotation_angle: f32,
}

impl Entity {
    fn new(texture:Texture, location:Vec2<f32>, rotation_angle:f32 ) -> Entity {
        Entity { texture:texture, location:location, rotation_angle:rotation_angle }
    }

    fn draw(&mut self, ctx: &mut Context ) {
        let params:tetra::graphics::DrawParams = graphics::DrawParams { position: self.location, rotation: self.rotation_angle, ..Default::default()};
        self.texture.draw(ctx, params );
    }
}

struct GameState{
    player1: Entity,
    player2: Entity,
    ball: Entity,
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context)  -> tetra::Result {
        if input::is_key_down(ctx, Key::W) {
            self.player1.location.y -= PADDLE_SPEED;
        }
        if input::is_key_down(ctx, Key::S) {
            self.player1.location.y += PADDLE_SPEED;
        }
        if input::is_key_down(ctx, Key::Up) {
            self.player2.location.y -= PADDLE_SPEED;
        }
        if input::is_key_down(ctx, Key::Down) {
            self.player2.location.y += PADDLE_SPEED;
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.0, 0.0, 0.0));
        self.player1.draw(ctx);
        self.player2.draw(ctx);
        self.ball.draw(ctx);
        Ok(())
    }
}

impl GameState {
    fn new(ctx: &mut Context ) -> tetra::Result <GameState> {
        let mut paddle_texture1 = Texture::new(ctx, "./resources/paddleBlu.png")?;
        let mut paddle_texture2 = Texture::new(ctx, "./resources/paddleRed.png")?;
        let mut paddle_texture3 = Texture::new(ctx, "./resources/ballBlue.png")?;
        let paddle_position1 =  Vec2::new(32.0, (WINDOW_HEIGHT - paddle_texture1.height() as f32) / 2.0);
        let paddle_position2 = Vec2::new(600.0, (WINDOW_HEIGHT - paddle_texture2.height() as f32) / 2.0);
        let paddle_position3 = Vec2::new((WINDOW_WIDTH - paddle_texture3.width() as f32) / 2.0, (WINDOW_HEIGHT - paddle_texture3.height() as f32) / 2.0);
        let paddle_rotation_angle1 = 1.5705;
        let paddle_rotation_angle2 = 0.0;
        Ok(GameState{ 
            player1: Entity::new(paddle_texture1, paddle_position1, paddle_rotation_angle1),
            player2: Entity::new(paddle_texture2, paddle_position2, paddle_rotation_angle1),
            ball:  Entity::new(paddle_texture3, paddle_position3, paddle_rotation_angle2),
        })
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Pong", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
    .quit_on_escape(true)
    .build()?
    .run(GameState::new)
}

