use nitro::App;
use nitro::component::Component;
use nitro::component::Message;
use nitro::GameObject;
use axes::AxisId;
use actions::ActionId;

pub struct Spinny {

}

impl Component for Spinny {
    fn receive_message(&mut self, app : &mut App, game_object : &mut GameObject, message : &Message) {
        match *message {
            Message::Update{delta_time} => {
                if let Some(horizontal) = app.get_axis_value(AxisId::Horizontal as i32) {
                    *game_object.transform.mut_x() += 100.0 * delta_time * horizontal;
                }
                if let Some(vertical) = app.get_axis_value(AxisId::Vertical as i32) {
                    *game_object.transform.mut_y() += 100.0 * delta_time * vertical;
                }
                if let Some(true) = app.action_released(ActionId::Blink as i32) {
                    *game_object.transform.mut_x() += 50.0;
                }
            }
            _ => {}
        }
    }
}
