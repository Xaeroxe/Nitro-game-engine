use graphics::Canvas;
use app::App;
use game_object::GameObject;
use component::Message;

/// Allows GameObjects to be extended with additional behavior.
///
/// This trait and understanding of it is essential to working in Nitro, most of your game logic
/// will live in structures that implement this trait.  You can then attach those structures to
/// GameObjects and Nitro will distribute messages to them which the Components can then respond
/// to.
pub trait Component {
    /// This function is called whenever the app has a message for your Component.  By using a
    /// match statement on the message you can make your component respond to a variety of events
    /// generated by the engine.
    fn receive_message(&mut self, _: &mut App, _: &mut GameObject, _: &Message) {}

    /// Called whenever the game is ready for you to render a gui.  If you have no gui to render
    /// then do nothing in this function.  This will be called for every component on every
    /// GameObject every frame.
    fn render_gui(&self, _: &mut Canvas, _: &GameObject) {}
}
