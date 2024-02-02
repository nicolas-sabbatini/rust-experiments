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
struct SlicesConf {
    slices: u32,
    width: u32,
    height: u32,
}

// Game config
#[derive(Debug)]
struct GameData {
    sprite_size: Option<Size>,
}

#[derive(Debug)]
struct Player {
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

struct GameState {
    slices: SlicesConf,
    data: GameData,
    player: Player,
    first_run: bool,
}

impl GameLoop for GameState {
    fn new(c: &mut EngineState) -> Self {
        let mut state = Self {
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
            first_run: true,
        };
        state.slices = serde_json::from_str(
            str::from_utf8(include_bytes!("../asssets/slices.json"))
                .expect("Unable to read JSON bytes"),
        )
        .expect("Unable to parse JSON");
        let height = state.slices.height as f32;
        let width = state.slices.width as f32;
        state.data.sprite_size = Some(Size::world(1.0, height / width));
        state
    }

    fn update(&mut self, c: &mut EngineContext) {
        if self.first_run {
            c.load_texture_from_bytes("car", include_bytes!("../asssets/slices.png"));
            self.first_run = false;
        }

        clear_background(WHITE);

        if is_key_pressed(KeyCode::R) {
            self.player.reset();
        }
        if is_key_down(KeyCode::W) {
            self.player.move_forward();
        }
        if is_key_down(KeyCode::S) {
            self.player.move_backward();
        }
        if is_key_down(KeyCode::A) {
            self.player.rotate_left();
        }
        if is_key_down(KeyCode::D) {
            self.player.rotate_right();
        }

        self.player.render(
            self.slices.slices,
            self.slices.width as i32,
            self.slices.height as i32,
            self.data.sprite_size,
        );
    }
}

comfy_game!("This is a test", GameState);
