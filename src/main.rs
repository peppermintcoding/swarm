use glutin_window::GlutinWindow;
use graphics::{Context, Rectangle, Graphics};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::{RenderEvent, MouseCursorEvent, WindowSettings};
use vecmath::{vec2_add, vec2_normalized, vec2_scale, vec2_sub, vec2_len};
use rand::Rng;


struct Bird {
    pos: [f64; 2],
    color: [f32; 4],
    min_size: f64,
    speed: f64
}

struct App {
    window: [f64; 2],
    mouse: [f64; 2],
    birds: Vec<Bird>
}

fn get_random_color() -> [f32; 4] {
    let mut rng = rand::thread_rng();
    [rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), 1.0]
}

impl App {
    fn draw<G: Graphics>(&mut self, c: &Context, g: &mut G) {
        for bird in &self.birds {
            let distance = vec2_len(vec2_sub(bird.pos, self.mouse));
            let multiplier = (distance / self.window[0] + distance / self.window[1]) / 2.0;
            let size = bird.min_size + multiplier * 80.0;
            Rectangle::new(bird.color).draw(
                [bird.pos[0] - size / 2.0, bird.pos[1] - size / 2.0, size, size],
                &c.draw_state,
                c.transform,
                g,
            );
        }
    }

    fn update(&mut self) {
        // let speed = 1.0 * args.dt;
        let mut rng = rand::thread_rng();
        for bird in &mut self.birds {
            let rand_speed = bird.speed - 0.4;
            let rand_direction = vec2_scale(vec2_normalized([rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)]), rand_speed);
            let direction = vec2_scale(vec2_normalized(vec2_sub(self.mouse, bird.pos)), bird.speed);
            bird.pos = vec2_add(bird.pos, vec2_add(direction, rand_direction));
        }
    }

    fn update_mouse(&mut self, pos: [f64; 2]) {
        self.mouse = pos;
    }

    fn spawn_birds(&mut self, n: i64) {
        let mut rng = rand::thread_rng();
        for _ in 0..n {
            self.birds.push(Bird {
                pos: [rng.gen_range(0.0..self.window[0]), rng.gen_range(0.0..self.window[1])],
                color: get_random_color(),
                min_size: rng.gen_range(10.0..25.0),
                speed: rng.gen_range(0.6..2.5)
            })
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    let width = 1200.0;
    let height = 650.0;
    let settings = WindowSettings::new("The Swarm", (width, height))
        .exit_on_esc(true)
        .graphics_api(opengl)
        .vsync(true);
    let mut window: GlutinWindow =
        settings.build().expect("Could not create window");
    let mut events = Events::new(EventSettings::new());
    let mut gl = GlGraphics::new(opengl);

    let mut app = App {window: [width, height], mouse: [0.0; 2], birds: Vec::new()};
    app.spawn_birds(50);

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::clear;

                clear([0.22, 0.22, 0.22, 1.0], g);
                app.draw(&c, g);
            });
            app.update();
        }

        // if let Some(args) = e.update_args() {
        //     app.update(&args);
        // }

        if let Some(pos) = e.mouse_cursor_args() {
            app.update_mouse(pos);
        }
    }
}