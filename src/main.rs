use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler};

use rand::Rng;

const SCREEN_SIZE: f32 = 100.0;

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE, SCREEN_SIZE))
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = MyGame::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game);
}

struct MyGame {
    // Your state here...
    world: Vec<bool>
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.

        // Init world randomly
        let mut new_w: Vec<bool> = Vec::new();

        for _ in 0..1000 {
            let num: f64 = rand::thread_rng().gen();
            if num < 0.30 {
                new_w.push(true);
            } else {
                new_w.push(false)
            }
        }

        MyGame {
            world: new_w
        }
    }
}

impl EventHandler<ggez::GameError> for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...

        let mut new_w: Vec<bool> = Vec::new();

        for (pos, alive) in self.world.iter().enumerate(){

            // Count neighboors
            let mut cntneig = 0;

            if pos % 10 != 0 && pos >= 11 {
                let tl = pos - 10 - 1;
                if self.world[tl] { cntneig += 1; }
            }
            if pos > 10 {
                let t = pos-10;
                if self.world[t] { cntneig += 1; }
            }
            if pos % 10 != (10-1) && pos > 10 {
                let tr = pos - 10 + 1;
                if self.world[tr] { cntneig += 1; }
            }
            if pos % 10 != 0 {
                let l = pos - 1;
                if self.world[l] { cntneig += 1; } 
            }
            if pos % 10 != (10-1) {
                let r = pos + 1;
                if self.world[r] { cntneig += 1; } 
            }
            if pos % 10 != 0 && pos < (10*10)-10 {
                let bl = pos + 10 - 1;
                if self.world[bl] { cntneig += 1; }
            }
            if pos < (10*10)-10 {
                let b = pos + 10;
                if self.world[b] { cntneig += 1; }
            }
            if pos % 10 != (10-1) && pos < (10*10)-10 {
                let br = pos + 10 + 1;
                if self.world[br] { cntneig += 1; }
            }

            // Alive condition
            if cntneig == 3 || (cntneig == 2 && *alive){
                new_w.push(true);
            } else { // Dead
                new_w.push(false);
            }

        }

        self.world = new_w;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK);
        // Draw code here...

        for (pos, alive) in self.world.iter().enumerate(){
            if *alive {

                let rectpos = graphics::Rect{
                    x: (10 * (pos % 10)) as f32,
                    y: (10 * (pos / 10)) as f32,
                    h: 10.0,
                    w: 10.0
                };

                let rectangle = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    rectpos,
                    [0.7, 0.7, 0.7, 1.0].into(),
                )?;

                graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
            }
        }


        graphics::present(ctx)
    }
}