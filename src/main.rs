mod lib;
use ggez::{ContextBuilder, event};
use ggez::conf::WindowMode;
pub use crate::lib::MyGame;

fn main() {
    let n = 0.0;
    let d = 0.0;
    let window_mode = WindowMode::default().dimensions(800.0, 600.0);
    // Make a Context.
    let (mut ctx, mut event_loop) = ContextBuilder::new("Maurer Rose", "Alirezava4idi")
        .window_mode(window_mode)
		.build()
		.expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game = MyGame::new(&mut ctx, n, d);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}
