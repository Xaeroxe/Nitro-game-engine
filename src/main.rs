extern crate nitro;

mod spinny;
mod axes;
mod actions;

use nitro::App;
use nitro::GameObject;
use nitro::input::Axis;
use nitro::input::Button;
use nitro::input::keyboard::Key;
use spinny::Spinny;
use axes::AxisId;
use actions::ActionId;

fn main() {
    // Create a new game and run it.
    let mut app = App::new("Halera", false, true);
    app.add_axis(
        Axis::new(
            Button::Keyboard(Key::D),
            Button::Keyboard(Key::A)
        ),
        AxisId::Horizontal as i32
    );
    app.add_axis(
        Axis::new(
            Button::Keyboard(Key::S),
             Button::Keyboard(Key::W)
         ),
         AxisId::Vertical as i32
    );
    app.add_axis(
        Axis::new(
            Button::Keyboard(Key::E),
            Button::Keyboard(Key::Q)
        ),
        AxisId::Rotation as i32
    );
    app.add_action(Button::Keyboard(Key::F), ActionId::Blink as i32);
    let mut game_obj = GameObject::new(&mut app);
    game_obj.texture = app.fetch_texture("nitro.png");
    game_obj.add_component(Box::new(Spinny{}));
    app.add_gameobject(game_obj);
    app.run();
}
