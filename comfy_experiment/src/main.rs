#![allow(
    clippy::wildcard_imports,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss
)]
use comfy::*;
use serde::Deserialize;
use std::str;

// load JSON slice
#[derive(Debug, Deserialize)]
pub struct SlicesConf {
    slices: u32,
    width: u32,
    height: u32,
}

// Game config
#[derive(Debug)]
pub struct GameData {
    sprite_size: Option<Size>,
}

#[derive(Debug)]
pub struct Player {
    position: Vec2,
    velocity: f32,
    score: u32,
    angle: f32,
}

impl Player {
    fn move_forward(&mut self) {
        let angle = self.angle.to_radians() + std::f32::consts::FRAC_PI_2;
        self.position.x += angle.cos() * self.velocity;
        self.position.y += angle.sin() * self.velocity;
    }

    fn move_backward(&mut self) {
        let angle = self.angle.to_radians() + std::f32::consts::FRAC_PI_2;
        self.position.x -= angle.cos() * self.velocity;
        self.position.y -= angle.sin() * self.velocity;
    }

    fn rotate_left(&mut self) {
        self.angle += 2.0;
    }

    fn rotate_right(&mut self) {
        self.angle -= 2.0;
    }

    fn render(&self, slices: u32, width: i32, height: i32, dest_size: Option<Size>) {
        for i in 0..slices {
            let src_rect = IRect::new(ivec2(0, i as i32 * height), ivec2(width, height));
            draw_sprite_ex(
                texture_id("car"),
                self.position + vec2(0.0, 0.02 * (i as f32)),
                GRAY,
                i as i32,
                DrawTextureParams {
                    dest_size,
                    source_rect: Some(src_rect),
                    rotation: self.angle.to_radians(),
                    ..Default::default()
                },
            );
        }
    }

    fn reset(&mut self) {
        self.position = splat(0.0);
        self.velocity = 0.1;
        self.score = 0;
        self.angle = 0.0;
    }
}

pub struct GameState {
    pub slices: SlicesConf,
    pub data: GameData,
    pub player: Player,
}

impl GameState {
    pub fn new(_c: &mut EngineContext) -> Self {
        Self {
            slices: SlicesConf {
                slices: 0,
                width: 0,
                height: 0,
            },
            data: GameData { sprite_size: None },
            player: Player {
                position: splat(0.0),
                velocity: 0.1,
                score: 0,
                angle: 0.0,
            },
        }
    }
}

pub struct GameContext<'a, 'b: 'a> {
    pub game_state: &'a mut GameState,
    pub engine: &'a mut EngineContext<'b>,
}

fn make_context<'a, 'b: 'a>(
    state: &'a mut GameState,
    engine: &'a mut EngineContext<'b>,
) -> GameContext<'a, 'b> {
    GameContext {
        game_state: state,
        engine,
    }
}

comfy_game!(
    "This is a test",
    GameContext,
    GameState,
    make_context,
    setup,
    update
);

fn setup(c: &mut GameContext) {
    // Load
    c.engine
        .load_texture_from_bytes("car", include_bytes!("../asssets/slices.png"));
    c.game_state.slices = serde_json::from_str(
        str::from_utf8(include_bytes!("../asssets/slices.json"))
            .expect("Unable to read JSON bytes"),
    )
    .expect("Unable to parse JSON");

    let height = c.game_state.slices.height as f32;
    let width = c.game_state.slices.width as f32;

    c.game_state.data.sprite_size = Some(Size::world(1.0, height / width));
}

fn update(c: &mut GameContext) {
    clear_background(WHITE);

    if is_key_pressed(KeyCode::R) {
        c.game_state.player.reset();
    }
    if is_key_down(KeyCode::W) {
        c.game_state.player.move_forward();
    }
    if is_key_down(KeyCode::S) {
        c.game_state.player.move_backward();
    }
    if is_key_down(KeyCode::A) {
        c.game_state.player.rotate_left();
    }
    if is_key_down(KeyCode::D) {
        c.game_state.player.rotate_right();
    }

    c.game_state.player.render(
        c.game_state.slices.slices,
        c.game_state.slices.width as i32,
        c.game_state.slices.height as i32,
        c.game_state.data.sprite_size,
    );
}
