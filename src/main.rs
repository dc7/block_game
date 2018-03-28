extern crate ggez;
extern crate rand;

use ggez::conf;
use ggez::event::{EventHandler, run};
use ggez::graphics::Point2;
use ggez::graphics;
use ggez::timer;
use ggez::{Context, ContextBuilder, GameResult};
use rand::Rng;
use std::path;

#[derive(Clone, Debug)]
enum BlockColor {
    Blue,
    Green,
    Red,
}

#[derive(Debug)]
struct Block {
    color: BlockColor,
}

const BLOCK_SIZE: f32 = 40.0;
const BOARD_HEIGHT: usize = 10;
const BOARD_WIDTH: usize = 8;

fn random_block() -> Block {
    let colors = [BlockColor::Blue, BlockColor::Green, BlockColor::Red];
    Block {
        color: rand::thread_rng().choose(&colors).unwrap().clone(),
    }
}

fn random_board() -> Vec<Vec<Option<Block>>> {
    let mut board = Vec::new();
    let empty_chance = 0.5;
    for _x in 0..BOARD_WIDTH {
        let mut col = Vec::new();
        for _y in 0..BOARD_HEIGHT {
            if empty_chance <= rand::thread_rng().next_f32() {
                col.push(Some(random_block()));
            }
            else {
                col.push(None);
            }
        }
        board.push(col);
    }
    board
}

struct Assets {
    blue_image: graphics::Image,
    green_image: graphics::Image,
    red_image: graphics::Image,
}

impl Assets {
    fn new(ctx: &mut Context) -> GameResult<Assets> {
        let blue = graphics::Color::new(0.0, 0.0, 1.0, 1.0);
        let green = graphics::Color::new(0.0, 1.0, 0.0, 1.0);
        let red = graphics::Color::new(1.0, 0.0, 0.0, 1.0);
        Ok(Assets {
            blue_image: graphics::Image::solid(ctx, BLOCK_SIZE as u16, blue)?,
            green_image: graphics::Image::solid(ctx, BLOCK_SIZE as u16, green)?,
            red_image: graphics::Image::solid(ctx, BLOCK_SIZE as u16, red)?,
        })
    }

    fn image_from_block_color(&mut self, color: &BlockColor) -> &mut graphics::Image {
        match *color {
            BlockColor::Blue => &mut self.blue_image,
            BlockColor::Green => &mut self.green_image,
            BlockColor::Red => &mut self.red_image,
        }
    }
}

struct MainState {
    assets: Assets,
    board: Vec<Vec<Option<Block>>>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        graphics::set_background_color(ctx, (0, 0, 0, 255).into());
        Ok(MainState {
            assets: Assets::new(ctx)?,
            board: random_board(),
        })
    }

    fn drop_board(&mut self) -> () {
        for x in 0..BOARD_WIDTH {
            for y in (0..BOARD_HEIGHT).rev() {
                if self.board[x][y].is_none() {
                    self.board[x].remove(y);
                    self.board[x].insert(0, None);
                }
            }
        }
    }
}

fn draw_block(assets: &mut Assets, ctx: &mut Context, block: &Block, pos: Point2) -> GameResult<()> {
    let drawparams = graphics::DrawParam {
        dest: pos,
        ..Default::default()
    };
    graphics::draw_ex(ctx, assets.image_from_block_color(&block.color), drawparams)
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.drop_board();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        let assets = &mut self.assets;
        for xi in 0..BOARD_WIDTH {
            for yi in 0..BOARD_HEIGHT {
                let x = xi as f32 * BLOCK_SIZE;
                let y = yi as f32 * BLOCK_SIZE;
                if let Some(ref block) = self.board[xi][yi] {
                    draw_block(assets, ctx, &block, Point2::new(x, y))?;
                }
            }
        }
        graphics::present(ctx);
        timer::yield_now();
        Ok(())
    }
}

pub fn main() {
    let cb = ContextBuilder::new("block_game", "ggez")
        .window_setup(conf::WindowSetup::default().title("Block Game"))
        .window_mode(conf::WindowMode::default().dimensions(640, 480))
        .add_resource_path(path::PathBuf::from("resources"));
    let ctx = &mut cb.build().unwrap();
    let ref mut game = MainState::new(ctx).unwrap();
    run(ctx, game).unwrap();
}
