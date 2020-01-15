mod game;

extern crate piston_window;

use piston_window::*;

const WINDOW_WIDTH: f64 = 1000.0;
const WINDOW_HEIGHT: f64 = 1600.0;

fn main() {
    let mut game = game::Game::new();

    let frame = 0.0;
    let mut window: PistonWindow =
        WindowSettings::new("piston: draw_state", [WINDOW_WIDTH, WINDOW_HEIGHT])
            .exit_on_esc(true)
            .samples(4)
            .build()
            .unwrap();

    let mut events = Events::new(EventSettings::new().ups(60));
    while let Some(e) = events.next(&mut window) {
        match e {
            Event::Input(input_event) => {
                //handle input events
                if let Input::Button(key) = input_event {
                    game.handle_keyboard_event(key)
                }
            }
            Event::Loop(loop_event) => match loop_event {
                Loop::Update(_) => game.update(),
                Loop::Render(_) => {
                    window.draw_2d(&e, |context, graphics| {
                        let transform = context.transform.trans(frame, 0.0);
                        game.render(context, transform, graphics);
                    });
                }
                _ => {}
            },
            _ => {}
        }
        //        frame = frame - 0.01;
    }
}
