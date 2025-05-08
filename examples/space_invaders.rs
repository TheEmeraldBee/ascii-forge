/*
An Example of Something slightly more complex that could be achieved.
This is not a complete example, it is not fun, it is purely to show how things COULD be made.

Requires the "keyboard" feature to be enabled
*/

use std::{
    io,
    time::{Duration, SystemTime},
};

use crossterm::event::*;
use crossterm::style::*;

use ascii_forge::prelude::*;

pub struct Projectile<E: Render> {
    loc: (f32, f32),
    velocity: f32,
    element: E,
}

impl<E: Render> Projectile<E> {
    pub fn new(loc: Vec2, velocity: f32, element: E) -> Self {
        Self {
            loc: (loc.x as f32, loc.y as f32),
            velocity,
            element,
        }
    }

    pub fn update(&mut self) {
        self.loc.1 += self.velocity;
    }

    pub fn draw_loc(&self) -> Vec2 {
        vec2(self.loc.0.floor() as u16, self.loc.1.floor() as u16)
    }

    pub fn draw(&self, window: &mut Window) {
        render!(window,
            self.draw_loc() => [ self.element ]
        );
    }

    pub fn alive(&self, window: &Window) -> bool {
        self.loc.1 >= 2.0 && self.loc.1 < (window.size().y - 2) as f32
    }
}

pub struct Player<E: Render> {
    loc: Vec2,
    element: E,
    input: i32,
}

impl<E: Render> Player<E> {
    pub fn new(window: &Window, element: E) -> Self {
        Self {
            loc: vec2(window.size().x / 2, window.size().y - 3),
            input: 0,
            element,
        }
    }

    pub fn draw(&self, window: &mut Window) {
        render!(window, self.loc => [ self.element ]);
    }

    pub fn update(&mut self, window: &mut Window) {
        self.input = 0;
        if event!(window, Event::Key(e) => e.code == KeyCode::Right) {
            self.input = 1;
        }
        if event!(window, Event::Key(e) => e.code == KeyCode::Right) {
            self.input = -1;
        }

        self.loc.x = (self.loc.x as i32 + self.input).clamp(0, window.size().x as i32) as u16;
    }

    pub fn hit<R: Render>(&mut self, projectiles: &[Projectile<R>]) -> bool {
        projectiles.iter().any(|x| x.draw_loc() == self.loc)
    }
}

pub struct Enemy<E: Render> {
    loc: Vec2,
    right: bool,
    element: E,
    score: u32,
}

impl<E: Render> Enemy<E> {
    pub fn new(loc: Vec2, element: E, score: u32) -> Self {
        Self {
            loc,
            right: true,
            element,
            score,
        }
    }

    pub fn draw(&mut self, window: &mut Window) {
        render!(window, self.loc => [ self.element ]);
    }

    pub fn hit<R: Render>(&mut self, projectiles: &[Projectile<R>]) -> bool {
        projectiles.iter().any(|x| {
            let loc = x.draw_loc();
            loc.y == self.loc.y && ((loc.x)..=(loc.x + 2)).contains(&self.loc.x)
        })
    }

    pub fn enemy_move(&mut self, window: &Window) -> bool {
        if self.loc.y >= window.size().y - 4 {
            true
        } else {
            match self.right {
                true => {
                    self.loc.x += 1;
                    if self.loc.x >= window.size().x {
                        self.right = false;
                        self.loc.y += 1;
                    }
                }
                false => {
                    self.loc.x -= 1;
                    if self.loc.x == 0 {
                        self.right = true;
                        self.loc.y += 1;
                    }
                }
            }

            false
        }
    }
}

pub fn main() -> io::Result<()> {
    // Create the window, and ask the engine to catch a panic
    let mut window = Window::init()?;

    // Require kitty keyboard support to be enabled.
    window.keyboard()?;

    handle_panics();

    // Run the application
    // Store the result so restore happens no matter what.
    let result = app(&mut window);

    // Restore the previous screen on the terminal
    // Since a print statement will come after this, we want to restore the window.
    window.restore()?;

    // Now check if error
    let result = result?;

    // Print Exit Message
    println!("{}", result);
    Ok(())
}

pub fn app(window: &mut Window) -> io::Result<String> {
    let mut score = 0;
    let mut player = Player::new(window, 'W'.green());

    let mut projectiles = vec![];

    let mut enemies = vec![];

    let mut delta = Duration::ZERO;

    let mut spawner = Duration::from_millis(800);

    let mut move_timer = Duration::from_millis(200);

    let mut shoot_timer = Duration::from_millis(500);

    let info_text = Buffer::sized_element("Press C-q to quit");

    // Main Game Loop
    loop {
        let start = SystemTime::now();
        // update the window, without blocking the screen
        window.update(Duration::from_secs_f64(1.0 / 60.0))?;

        if event!(window, Event::Key(e) => e.code == KeyCode::Char(' ')) {
            projectiles.push(Projectile::new(
                vec2(player.loc.x - 1, player.loc.y - 1),
                -0.3,
                "|||".green(),
            ))
        }

        // Render and update projectiles
        projectiles.retain(|x| x.alive(window));

        projectiles.iter_mut().for_each(|x| {
            x.update();
            x.draw(window);
        });

        // Render and update the player.
        player.update(window);
        player.draw(window);

        match spawner.checked_sub(delta) {
            Some(s) => spawner = s,
            None => {
                enemies.push(Enemy::new(vec2(0, 3), 'M'.red(), 10));
                spawner = Duration::from_secs(2);
            }
        }

        match move_timer.checked_sub(delta) {
            Some(m) => move_timer = m,
            None => {
                if enemies.iter_mut().any(|x| x.enemy_move(window)) {
                    return Ok(format!("Game Over\nScore was: {}", score));
                }
                move_timer = Duration::from_millis(200);
            }
        }

        if player.hit(&projectiles) {
            return Ok(format!("Game Over\nScore was: {}", score));
        }

        match shoot_timer.checked_sub(delta) {
            Some(s) => shoot_timer = s,
            None => {
                enemies.iter_mut().for_each(|x| {
                    projectiles.push(Projectile::new(vec2(x.loc.x, x.loc.y + 1), 0.3, "|".red()))
                });
                shoot_timer = Duration::from_millis(500);
            }
        }

        enemies.retain_mut(|x| {
            if x.hit(&projectiles) {
                score += x.score;
                false
            } else {
                true
            }
        });

        enemies.iter_mut().for_each(|x| x.draw(window));

        render!(
            window,
            vec2(0, 0) => [ format!("Score: {}", score) ],
            vec2(window.size().x - info_text.size().x, 0) => [ info_text ],
        );

        if event!(window, Event::Key(e) => *e == KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL))
        {
            break;
        }

        delta = SystemTime::now().duration_since(start).unwrap();
    }

    Ok("Game Exited".to_string())
}
