use glutin_window::GlutinWindow;
use graphics::{Context, Rectangle, Graphics};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::{RenderEvent, MouseCursorEvent, WindowSettings};
use vecmath::{vec2_add, vec2_normalized, vec2_scale, vec2_sub};
use rand::Rng;


struct App {
    mouse: [f64; 2],
    birds: Vec<[f64; 2]>
}

impl App {
    fn draw<G: Graphics>(&mut self, c: &Context, g: &mut G) {
        let size = 50.0;
        for bird in &self.birds {
            Rectangle::new([1.0; 4]).draw(
                [bird[0] - size / 2.0, bird[1] - size / 2.0, size, size],
                &c.draw_state,
                c.transform,
                g,
            );
        }
    }

    fn update(&mut self) {
        // let speed = 1.0 * args.dt;
        let mut rng = rand::thread_rng();
        let speed = 1.5;
        let rand_speed = 1.1;
        for bird in &mut self.birds {
            let rand_direction = vec2_scale(vec2_normalized([rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)]), rand_speed);
            let direction = vec2_scale(vec2_normalized(vec2_sub(self.mouse, *bird)), speed);
            *bird = vec2_add(*bird, vec2_add(direction, rand_direction));
        }
    }

    fn update_mouse(&mut self, pos: [f64; 2]) {
        self.mouse = pos;
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("The Swarm", (640, 480))
        .exit_on_esc(true)
        .graphics_api(opengl)
        .vsync(true);
    let mut window: GlutinWindow =
        settings.build().expect("Could not create window");
    let mut events = Events::new(EventSettings::new());
    let mut gl = GlGraphics::new(opengl);

    let mut app = App {mouse: [0.0; 2], birds: Vec::new()};
    app.birds.push([50.0, 50.0]);
    app.birds.push([250.0, 250.0]);
    app.birds.push([250.0, 50.0]);

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