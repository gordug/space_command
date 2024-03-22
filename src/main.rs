use std::time::Duration;
use macroquad::prelude::*;
use macroquad::time::get_time;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Base {
    position: Vec2,
    color: Color,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Explosion {
    position: Vec2,
    color: Color,
    size: f32,
    spawn_time: f64,
    duration: Duration,
    flash: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Bullet {
    source: Vec2,
    position: Vec2,
    velocity: Vec2,
    target: Vec2,
    color: Color,
    explosion: Option<Explosion>,
    exploding: bool,
    exploded: bool,
}

impl Bullet {
    fn new(source: Vec2, target: Vec2, velocity: Vec2) -> Self {
        Bullet {
            source,
            position: source,
            velocity,
            target,
            color: RED,
            explosion: None,
            exploding: false,
            exploded: false,
        }
    }

    fn explode(&mut self, color: Color, size: f32, duration: Duration, flash: bool) {
        if self.exploding || self.exploded {
            return;
        }

        self.explosion = Some(Explosion {
            position: self.position,
            color,
            size,
            spawn_time: get_time(),
            duration,
            flash,
        });
        self.exploding = true;
    }

    pub fn get_explosion_spawn_time(&self) -> f64 {
        if self.explosion.is_none() {
            return 0.;
        }
        self.explosion.unwrap().spawn_time
    }

    fn at_target(&self) -> bool {
        (self.position - self.target).length() < 5.
    }
}

impl Explosion {
    pub fn get_spawn_time(&self) -> f64 {
        self.spawn_time
    }
}

#[derive(Clone, Debug)]
struct Game {
    started: bool,
    font: Font,
    fullscreen: bool,
    window_size: Vec2,
    difficulty: Difficulty,
    grid: bool,
}

impl Game {
    async fn new(font_bytes: &[u8]) -> Self {
        Game {
            started: false,
            font: load_ttf_font_from_bytes(font_bytes).expect("Failed to load font"),
            fullscreen: false,
            window_size: vec2(screen_width(), screen_height()),
            difficulty: Difficulty::default(),
            grid: false,
        }
    }

    fn toggle_fullscreen(&mut self) {
        if !self.fullscreen {
            self.window_size = vec2(screen_width(), screen_height());
        }
        self.fullscreen = !self.fullscreen;
        set_fullscreen(self.fullscreen);
        if !self.fullscreen {

        }
    }
    
    async fn draw_menu(&mut self){
        let font = &self.font.clone();
        let main_text_params = TextParams {
            font_size: 20,
            font_scale: 1.0,
            font_scale_aspect: 1.0,
            font: Some(font),
            color: WHITE,
            rotation: 0.,
        };
        let sub_text_params = TextParams {
            font_size: 15,
            font_scale: 1.0,
            font_scale_aspect: 1.0,
            font: Some(font),
            color: GRAY,
            rotation: 0.,
        };
        loop {
            self.process_input().await;
            if self.grid {
                self.draw_grid().await;
            }
            else {
                clear_background(BLACK);
            }
            draw_text_ex("Space Command", screen_width() / 2. - 128., screen_height() / 2. - 50., main_text_params.clone());
            draw_text_ex("Press Space to Start", screen_width() / 2. - 158., screen_height(), main_text_params.clone());
            draw_text_ex("Press Ctrl + R to Reset", screen_width() / 2. - 128., screen_height() - 50., sub_text_params.clone());

            if is_key_pressed(KeyCode::Space) {
                break;
            }
            if is_key_pressed(KeyCode::R) && is_key_down(KeyCode::LeftControl) {
                self.difficulty.reset();
            }
            if is_key_pressed(KeyCode::I) && is_key_down(KeyCode::LeftControl) {
                self.difficulty.increase_difficulty();
            }
            if is_key_pressed(KeyCode::F11) {
                self.toggle_fullscreen();
            }

            next_frame().await;
        }
        self.started = true;        
    }
    
    async fn process_input(&mut self) {
        if is_key_pressed(KeyCode::F11) {
            self.toggle_fullscreen();
        }
        if is_key_pressed(KeyCode::Escape) && self.fullscreen {
            self.toggle_fullscreen();
        }

        // ctrl + R to reset the game
        if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::R) {
            self.difficulty.reset();
        }
        
        if is_key_pressed(KeyCode::F12) {
            self.grid = !self.grid;
        }
    }
    
    #[cfg(debug_assertions)]
    async fn draw_grid(&mut self){
        for i in (0..screen_width() as i32).step_by(10) {
            if i % 20 == 0 {
                draw_line(i as f32, 0., i as f32, screen_height(), 1., WHITE);
            }
            else{
                draw_line(i as f32, 0., i as f32, screen_height(), 1., GRAY);
            }
        }
        for i in (0..screen_height() as i32).step_by(10) {
            if i % 20 == 0 {
                draw_line(0., i as f32, screen_width(), i as f32, 1., WHITE);
            }
            else {
                draw_line(0., i as f32, screen_width(), i as f32, 1., GRAY);
            }
        }
        draw_line(screen_width() / 2., 0., screen_width() / 2., screen_height(), 2., RED);
        draw_line(0., screen_height() / 2., screen_width(), screen_height() / 2., 2., RED);
    
    }
    #[cfg(not(debug_assertions))]
    async fn draw_grid(&mut self){}
}

#[derive(Clone, Debug)]
struct Difficulty {
    missile_spawn_rate: f32,
    missile_speed: f32,
    missile_rounds: u32,
    round: u32,
    explosion_size: f32,
}

impl Difficulty {
    pub fn new() -> Self {
        Difficulty {
            missile_spawn_rate: 3.,
            missile_speed: 50.,
            missile_rounds: 10,
            round: 1,
            explosion_size: 100.,
        }
    }

    pub fn reset(&mut self) {
        self.missile_spawn_rate = 3.;
        self.missile_speed = 50.;
        self.missile_rounds = 10;
        self.round = 1;
        self.explosion_size = 100.;
    }

    pub fn increase_difficulty(&mut self) {
        self.missile_spawn_rate -= 0.1 * self.round as f32;
        self.missile_speed += 1.1 * self.round as f32;
        self.missile_rounds += 1 * self.round;
        self.round += 1;
        self.explosion_size -= 0.01 * self.round as f32;
    }

    pub fn get_missile_spawn_rate(&self) -> f32 {
        self.missile_spawn_rate
    }

    pub fn get_missile_speed(&self) -> f32 {
        self.missile_speed
    }

    pub fn get_missile_rounds(&self) -> u32 {
        self.missile_rounds
    }

    pub fn get_round(&self) -> u32 {
        self.round
    }

    pub fn get_explosion_size(&self) -> f32 {
        self.explosion_size
    }
}

impl Default for Difficulty {
    fn default() -> Self {
        Self::new()
    }
}

#[macroquad::main("Space Command")]
async fn main() {
    let font = include_bytes!("../assets/fonts/Geoplace-Bold.ttf");
    let mut game = Game::new(font).await;
    let mut bullets: Vec<Bullet> = Vec::new();
    let mut time_since_spawn = 0.;
    let mut difficulty = Difficulty::new();
    let mut debug_grid = false;
    loop {
        if !game.started {
            // Clear the screen
            clear_background(BLACK);
            set_default_camera();
            // Show Controls
            let font = &game.font.clone();
            let main_text_params = TextParams {
                font_size: 20,
                font_scale: 1.0,
                font_scale_aspect: 1.0,
                font: Some(font),
                color: WHITE,
                rotation: 0.,
            };
            let sub_text_params = TextParams {
                font_size: 15,
                font_scale: 1.0,
                font_scale_aspect: 1.0,
                font: Some(font),
                color: GRAY,
                rotation: 0.,
            };
            loop {
                if is_key_pressed(KeyCode::F12) {
                    debug_grid = !debug_grid;
                }
                if debug_grid {
                    game.draw_grid().await;
                }
                else {
                    clear_background(BLACK);
                }
                draw_text_ex("Space Command", screen_width() / 2. - 128., screen_height() / 2. - 50., main_text_params.clone());
                draw_text_ex("Press Space to Start", screen_width() / 2. - 158., screen_height(), main_text_params.clone());
                draw_text_ex("Press Ctrl + R to Reset", screen_width() / 2. - 128., screen_height() - 50., sub_text_params.clone());

                if is_key_pressed(KeyCode::Space) {
                    break;
                }
                if is_key_pressed(KeyCode::R) && is_key_down(KeyCode::LeftControl) {
                    difficulty.reset();
                }
                if is_key_pressed(KeyCode::I) && is_key_down(KeyCode::LeftControl) {
                    difficulty.increase_difficulty();
                }
                if is_key_pressed(KeyCode::F11) {
                    game.toggle_fullscreen();
                }

                next_frame().await;
            }
            game.started = true;
        }
        // Clear the screen
        clear_background(BLACK);


        if is_key_pressed(KeyCode::F11) {
            game.toggle_fullscreen();
        }
        if is_key_pressed(KeyCode::Escape) && game.fullscreen {
            game.toggle_fullscreen();
        }

        // ctrl + R to reset the game
        if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::R) {
            difficulty.reset();
        }

        clear_background(BLACK);

        // every 3 seconds add a new bullet

        time_since_spawn += get_frame_time();
        if time_since_spawn > 1. {
            let source = vec2(rand::gen_range(0., screen_height()), 0.);
            let target = vec2(rand::gen_range(0., screen_width()), screen_height());
            let velocity = (target - source).normalize() * difficulty.get_missile_speed();
            let bullet = Bullet::new(source, target, velocity);
            bullets.push(bullet);
            time_since_spawn = 0.;
        }

        // Draw bullets
        for bullet in &mut bullets {
            // if bullet is exploding, draw the explosion
            if bullet.exploding {
                let explosion = bullet.explosion;
                if let Some(explosion) = explosion {
                    draw_circle(explosion.position.x, explosion.position.y, explosion.size, explosion.color);
                }
                bullet.exploding = false;
                bullet.exploded = true;
            }

            // move bullet
            bullet.position += bullet.velocity * get_frame_time();
            draw_circle(bullet.position.x, bullet.position.y, 5., bullet.color);
        }

        // remove bullets that have reached their target
        bullets.retain(|bullet| {
            !bullet.exploded
        });

        // draw bullets, poly line leading from source to position
        for bullet in &bullets {
            draw_line(bullet.source.x, bullet.source.y, bullet.position.x, bullet.position.y, 1., bullet.color);
        }

        // check if any bullets have reached their target
        for bullet in &mut bullets {
            if bullet.at_target() {
                bullet.explode(RED, 50., Duration::from_secs(1), false);
            }
        }

        bullets.retain(|bullet| {
            !bullet.exploded
        });

        next_frame().await
    }
}