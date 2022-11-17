use glutin_window::GlutinWindow;
use graphics::{Context, Rectangle, Graphics};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::{EventLoop, RenderEvent, UpdateEvent, WindowSettings, UpdateArgs};


fn draw<G: Graphics>(c: &Context, g: &mut G) {
    Rectangle::new([1.0; 4]).draw(
        [100.0, 100.0, 50.0, 50.0],
        &c.draw_state,
        c.transform,
        g,
    );
}

fn update(args: &UpdateArgs) {
    // args.dt
}

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Sudoku", (640, 480))
        .exit_on_esc(true)
        .graphics_api(opengl)
        .vsync(true);
    let mut window: GlutinWindow =
        settings.build().expect("Could not create window");
    let mut events = Events::new(EventSettings::new().lazy(true));
    let mut gl = GlGraphics::new(opengl);

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::clear;

                clear([0.22, 0.22, 0.22, 1.0], g);
                draw(&c, g);
            });
        }

        if let Some(args) = e.update_args() {
            update(&args);
        }
    }
}