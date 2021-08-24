use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, MeshBuilder};
use ggez::event::{self, EventHandler};
use ggez::timer;
use glam::*;

use rand::Rng;

const SCREEN_SIZE: f32 = 1000.0;

const GRID_SIZE: usize = 100;

const CELL_SIZE: f32 = SCREEN_SIZE / (GRID_SIZE as f32);

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("Game of Life", "Nara")
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

        for _ in 0..(GRID_SIZE * GRID_SIZE) {
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

            if pos % GRID_SIZE != 0 && pos >= GRID_SIZE+1 {
                let tl = pos - GRID_SIZE - 1;
                if self.world[tl] { cntneig += 1; }
            }
            if pos > GRID_SIZE {
                let t = pos-GRID_SIZE;
                if self.world[t] { cntneig += 1; }
            }
            if pos % GRID_SIZE != (GRID_SIZE-1) && pos > GRID_SIZE {
                let tr = pos - GRID_SIZE + 1;
                if self.world[tr] { cntneig += 1; }
            }
            if pos % GRID_SIZE != 0 {
                let l = pos - 1;
                if self.world[l] { cntneig += 1; } 
            }
            if pos % GRID_SIZE != (GRID_SIZE-1) {
                let r = pos + 1;
                if self.world[r] { cntneig += 1; } 
            }
            if pos % GRID_SIZE != 0 && pos < (GRID_SIZE*GRID_SIZE)-GRID_SIZE {
                let bl = pos + GRID_SIZE - 1;
                if self.world[bl] { cntneig += 1; }
            }
            if pos < (GRID_SIZE*GRID_SIZE)-GRID_SIZE {
                let b = pos + GRID_SIZE;
                if self.world[b] { cntneig += 1; }
            }
            if pos % GRID_SIZE != (GRID_SIZE-1) && pos < (GRID_SIZE*GRID_SIZE)-GRID_SIZE {
                let br = pos + GRID_SIZE + 1;
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

        println!("FPS: {}", timer::fps(_ctx));

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK);
        // Draw code here...

        let mut mesh = MeshBuilder::new();

        // Loop is bottleneck
        for (pos, alive) in self.world.iter().enumerate(){
            if *alive {

                let rectpos = graphics::Rect{
                    x: CELL_SIZE * ((pos % GRID_SIZE) as f32),
                    y: CELL_SIZE * ((pos / GRID_SIZE) as f32),
                    h: CELL_SIZE,
                    w: CELL_SIZE
                };

                mesh.rectangle(
                    graphics::DrawMode::fill(), 
                    rectpos, 
                    [0.7, 0.7, 0.7, 1.0].into()
                )?;
            }
        }

        let bmesh = mesh.build(ctx)?;

        graphics::draw(ctx, &bmesh, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;


        graphics::present(ctx)
    }
}