use ggez::{graphics, Context, GameResult, nalgebra as na};
use ggez::graphics::{DrawMode};
use ggez::event::EventHandler;


pub struct MyGame {
    // Your state here...
    n: f64,
    d: f64
}

impl MyGame {
    pub fn new(_ctx: &mut Context, n: f64, d: f64) -> MyGame {
        // Load/create resources such as images here.
        MyGame {
            // ...
            n,
            d
        }
    }
}

impl EventHandler for MyGame {
    
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        self.d += 0.03;
        self.n += 0.01;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        
        // Draw code here...
        let window = graphics::screen_coordinates(ctx);

        let mut points = Vec::new();
        
        let mut i = 0.0;

        while i < std::f64::consts::PI * 2.0{
            let k = i * self.d;
            let r = 150.0 * (self.n*k).sin();
            let x = r * k.cos();
            let y = r * k.sin();

            points.push(na::Point2::new(x as f32 , y as f32));
            i += std::f64::consts::PI / 180.0;
        }

        let mut route_points = Vec::new();
        let mut j = 0.0;
        while j < std::f64::consts::PI * 2.0{
            let k = j;
            let r = 150.0 * (self.n*k).sin();
            let x = r * k.cos();
            let y = r * k.sin();

            route_points.push(na::Point2::new(x as f32 , y as f32));
            j += std::f64::consts::PI / 180.0;
        }

        let circle = graphics::Mesh::new_polygon(ctx,DrawMode::stroke(1.0), &points, graphics::Color::new(0.0, 0.0, 255.0, 1.0))?;
        let route = graphics::Mesh::new_polygon(ctx,DrawMode::stroke(2.5), &route_points, graphics::Color::new(255.0, 0.0, 255.0, 1.0))?;
        
        graphics::draw(ctx, &circle, (na::Point2::new(window.w / 2.0, window.h / 2.0),))?;
        graphics::draw(ctx, &route, (na::Point2::new(window.w / 2.0, window.h / 2.0),))?;
        graphics::present(ctx)
    }
}