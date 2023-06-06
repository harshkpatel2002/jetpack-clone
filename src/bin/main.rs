//Harsh Patel
//CSC 363
use macroquad::prelude::*;
use macroquad::rand::gen_range;

struct Player {
    pos: Vec2,
    velocity: Vec2,
    on_ground: bool,
}

enum ObstacleType {
    Vertical,
    Horizontal,
}

struct Obstacle {
    pos: Vec2,
    size: Vec2,
    obstacle_type: ObstacleType,
}

struct GameState {
    player: Player,
    start_button: Button,
    game_started: bool,
    obstacles: Vec<Obstacle>,
    failed: bool,
    fail_message: String,
    distance: f32,
}

impl Player {
    fn new(pos: Vec2) -> Self {
        Self {
            pos,
            velocity: Vec2::new(0.0, 0.0),
            on_ground: false,
        }
    }

    fn update(&mut self) {
        if is_key_down(KeyCode::Space) {
            if self.on_ground {
                self.velocity.y = -800.0;
                self.on_ground = false;
            } else {
                //Limit player velocity while flying
                self.velocity.y = -350.0;
            }
        }

        self.velocity.y += 1000.0 * get_frame_time();
        self.pos += self.velocity * get_frame_time();

        if self.pos.y < 50.0 {
            //Prevent player from going above ceiling
            self.pos.y = 50.0;
            self.velocity.y = 0.0;
        } else if self.pos.y > screen_height() - 50.0 {
            //Prevent player from falling through ground
            self.pos.y = screen_height() - 50.0;
            self.velocity.y = 0.0;
            self.on_ground = true;
        } else {
            self.on_ground = false;
        }
    }

    fn draw(&self) {
        let body_color = GREEN;
        let head_color = YELLOW;

        let body_pos = self.pos;
        let head_pos = self.pos - Vec2::new(0.0, 25.0);

        draw_rectangle(body_pos.x - 17.5, body_pos.y - 25.0, 35.0, 50.0, body_color);
        draw_circle(head_pos.x, head_pos.y, 25.0, head_color);
    }
}

impl Obstacle {
    fn new(obstacle_type: ObstacleType) -> Self {
        //size of obstacle varies upon orientation
        //horizontals are drawn relative to width of the current screen
        //verticals are drawn relative to the height of the current screen
        let size = match obstacle_type {
            ObstacleType::Vertical => Vec2::new(50.0, gen_range(100.0, screen_height()- 300.0)),
            ObstacleType::Horizontal => Vec2::new(gen_range(50.0, screen_width() - 500.0), 50.0),
        };
        let pos = Vec2::new(screen_width(), gen_range(50.0, screen_height() - size.y - 200.0));

        Self {
            pos,
            size,
            obstacle_type,
        }
    }

    fn update(&mut self, player_speed: f32) {
        self.pos.x -= player_speed * get_frame_time();
    }

    fn draw(&self) {
        match self.obstacle_type {
            ObstacleType::Vertical => draw_rectangle(self.pos.x, self.pos.y, self.size.x, self.size.y, RED),
            ObstacleType::Horizontal => draw_rectangle(self.pos.x, self.pos.y, self.size.x, self.size.y, ORANGE),
        }
    }
}

struct Button {
    pos: Vec2,
    size: Vec2,
    color: Color,
    text: String,
}

impl Button {
    fn new(pos: Vec2, size: Vec2, color: Color, text: String) -> Self {
        Self {
            pos,
            size,
            color,
            text,
        }
    }

    fn contains_point(&self, point: Vec2) -> bool {
        point.x > self.pos.x - self.size.x / 2.0
            && point.x < self.pos.x + self.size.x / 2.0
            && point.y > self.pos.y - self.size.y / 2.0
            && point.y < self.pos.y + self.size.y / 2.0
    }

    fn draw(&self) {
        let text_size = measure_text(&self.text, None, 30, 1.0);
        let text_pos = self.pos - Vec2::new(text_size.width / 2.0, text_size.height / 2.0);

        draw_rectangle(
            self.pos.x - self.size.x / 2.0,
            self.pos.y - self.size.y / 2.0,
            self.size.x,
            self.size.y,
            self.color,
        );

        draw_text(&self.text, text_pos.x, text_pos.y, 30.0, BLACK);
    }
}

impl GameState {
    fn new() -> Self {
        let player_start_y = screen_height() - 100.0;
        let player = Player::new(Vec2::new(100.0, player_start_y));
        let start_button = Button::new(
            Vec2::new(screen_width() / 2.0, screen_height() / 2.0),
            Vec2::new(200.0, 100.0),
            BLUE,
            String::from("Start"),
        );

        Self {
            player,
            start_button,
            game_started: false,
            obstacles: Vec::new(),
            failed: false,
            fail_message: String::new(),
            distance: 0.0,
        }
    }

    fn update(&mut self) {
        if self.game_started {
            self.player.update();
            self.distance += get_frame_time() * 2.0;  
            for obstacle in &mut self.obstacles {
                obstacle.update(200.0); 
            }

            for obstacle in &self.obstacles {
                let player_bounds = Rect::new(self.player.pos.x - 15.0, self.player.pos.y - 10.0, 30.0, 50.0);  
                let obstacle_bounds = Rect::new(obstacle.pos.x, obstacle.pos.y + 16.0 , obstacle.size.x, obstacle.size.y + 16.0);

                if player_bounds.overlaps(&obstacle_bounds) {
                    self.failed = true;
                    self.fail_message = format!("You hit an obstacle! You travelled {:.2} meters. Press 'S' to restart.", self.distance);
                    self.game_started = false;
                }
            }

            self.obstacles.retain(|obstacle| obstacle.pos.x + obstacle.size.x > 0.0);
            
            //Randomly generates a number to decide between horizontal or vertical obstacle being drawn
            if gen_range(0.0, 1.0) < 0.01 {
                let obstacle_type = match gen_range(0, 2) {
                    0 => ObstacleType::Vertical,
                    _ => ObstacleType::Horizontal,
                };
                self.obstacles.push(Obstacle::new(obstacle_type));
            }
            
        } else if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pos = mouse_position();
            if self.start_button.contains_point(mouse_pos.into()) {
                self.game_started = true;
            }
        } else if self.failed && is_key_pressed(KeyCode::S) {
            *self = Self::new();
        }
    }

    fn draw(&self) {
        clear_background(SKYBLUE);

        draw_rectangle(0.0, screen_height() - 50.0, screen_width(), 50.0, DARKGREEN);

        self.player.draw();
        let score = format!("{:.2}m", self.distance);
        draw_text(&score,  20.0, 20.0, 30.0, BLACK);

        for obstacle in &self.obstacles {
            obstacle.draw();
        }

        if !self.game_started && !self.failed {
            self.start_button.draw();
        }

        if self.failed {
            let text_size = measure_text(&self.fail_message, None, 30, 1.0);
            let text_pos = Vec2::new(
                screen_width() / 2.0 - text_size.width / 2.0,
                screen_height() / 1.5 - text_size.height / 2.0,
            );
            draw_text(&self.fail_message, text_pos.x, text_pos.y, 30.0, BLACK);
        }
    }
}

#[macroquad::main("Harsh Patel CSC363 Final")]
async fn main() {
    let mut game_state = GameState::new();

    //Main Game Loop
    loop {
        game_state.update();
        game_state.draw();
        next_frame().await;
    }
}


