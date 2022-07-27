use std::panic::Location;

use tetra::graphics::{self, Color, Rectangle, Texture};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;
const PADDLE_SPEED: f32 = 8.0;
const BALL_SPEED: f32 = 5.0;

enum EntityRotationType {
    Default,
    Ninty,
    HundredEighty,
    TwoSeventy,
}

struct Entity {
    texture: Texture,
    location: Vec2<f32>,
    velocity: Vec2<f32>,
    rotation_type: EntityRotationType,
}

impl Entity {
    fn new(texture: Texture, location: Vec2<f32>, rotation_type: EntityRotationType) -> Entity {
        Entity {
            texture: texture,
            location: location,
            rotation_type: rotation_type,
            velocity: Vec2::zero(),
        }
    }

    fn new_with_velocity(
        texture: Texture,
        location: Vec2<f32>,
        rotation_type: EntityRotationType,
        velocity: Vec2<f32>,
    ) -> Self {
        Entity {
            texture: texture,
            location: location,
            rotation_type: rotation_type,
            velocity: velocity,
        }
    }

    fn draw(&mut self, ctx: &mut Context) {
        let params: tetra::graphics::DrawParams = graphics::DrawParams {
            position: self.location,
            rotation: self.draw_angle(),
            origin: self.get_draw_position(),
            ..Default::default()
        };
        self.texture.draw(ctx, params);
    }

    fn width(&self) -> f32 {
        if matches!(self.rotation_type, EntityRotationType::Default) {
            return self.width_texture();
        }
        else if matches!(self.rotation_type, EntityRotationType::Ninty) {
            return self.height_texture();
        }
        else if matches!(self.rotation_type, EntityRotationType::HundredEighty) {
            return self.width_texture();
        }
        else {
            return self.height_texture();
        }
    }

    fn height(&self) -> f32 {
        if matches!(self.rotation_type, EntityRotationType::Default) {
            return self.height_texture();
        }
        else if matches!(self.rotation_type, EntityRotationType::Ninty) {
            
            return self.width_texture();
        }
        else if matches!(self.rotation_type, EntityRotationType::HundredEighty) {
            return self.height_texture();
        }
        else {
            return self.width_texture();
        }
    }
    fn draw_angle(&self) -> f32 {
        if matches!(self.rotation_type, EntityRotationType::Default) {
            return 0.0;
        }
        else if matches!(self.rotation_type, EntityRotationType::Ninty) {
            return 1.57;
        }
        else if matches!(self.rotation_type, EntityRotationType::HundredEighty) {
            return 3.14;
        }
        else {
            return 4.71;
        }
    }

    fn width_texture(&self) -> f32 {
        return self.texture.width() as f32;
    }

    fn height_texture(&self) -> f32 {
        return self.texture.height() as f32;
    }

    fn get_draw_position(&self) -> Vec2<f32> {
       return self.width_height_half();
    }

    fn width_height(&self) -> Vec2<f32> {
        return Vec2::new(self.width(), self.height());
    }

    fn width_height_half(&self) -> Vec2<f32> {
        return self.width_height() * 0.5;
    }

    fn bounds(&self) -> Rectangle {
        return Rectangle {
            x: self.location.x,
            y: self.location.y,
            width: self.width() * 0.5,
            height : self.height() * 0.5,
        };
    }

    fn intersects_with(&self, other_bounds:Rectangle) -> bool {
        let self_bounds = self.bounds();
        let x = other_bounds.x - self_bounds.x;
        let y = other_bounds.y - self_bounds.y;
        return f32::abs(x) < self_bounds.width && f32::abs(y) < self_bounds.height;
    }

    fn intersects_with_object(&self, other:&Entity) -> bool {
        return self.intersects_with(other.bounds());
    }

}

struct GameState {
    player1: Entity,
    player2: Entity,
    ball: Entity,
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
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


        if self.player1.intersects_with_object(&self.ball) {
            self.ball.velocity = -self.ball.velocity;
        }
        if self.player2.intersects_with_object(&self.ball) {
            self.ball.velocity = -self.ball.velocity;
        }

        self.ball.location += self.ball.velocity;
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
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let mut paddle_texture1 = Texture::new(ctx, "./resources/paddleBlueRotate90.png")?;
        let mut paddle_texture2 = Texture::new(ctx, "./resources/paddleRedRotate90.png")?;
        let mut paddle_texture3 = Texture::new(ctx, "./resources/ballBlue.png")?;
        let paddle_position1 = Vec2::new(20.0, (WINDOW_HEIGHT) / 2.0);
        let paddle_position2 = Vec2::new(620.0, (WINDOW_HEIGHT) / 2.0);
        let paddle_position3 = Vec2::new((WINDOW_WIDTH as f32) / 2.0, (WINDOW_HEIGHT as f32) / 2.0);
        let ball_velocity = Vec2::new(-BALL_SPEED, 0.0);
        Ok(GameState {
            player1: Entity::new(paddle_texture1, paddle_position1, EntityRotationType::Default),
            player2: Entity::new(paddle_texture2, paddle_position2, EntityRotationType::Default),
            ball: Entity::new_with_velocity(
                paddle_texture3,
                paddle_position3,
                EntityRotationType::Default,
                ball_velocity,
            ),
        })
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Pong", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}
