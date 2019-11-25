#![deny(missing_docs)]

extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::{Events, EventLoop, EventSettings};
use piston::input::RenderEvent;
use glutin_window::GlutinWindow;
use opengl_graphics::{OpenGL, GlGraphics};

pub use crate::gameboard::Gameboard;
pub use crate::gameboard_controller::GameboardController;
pub use crate::gameboard_view::{GameboardView, GameboardViewSettings};

mod gameboard;
mod gameboard_controller;
mod gameboard_view;

fn main() {
  let settings = WindowSettings::new("Sudoku", [512; 2])
      .exit_on_esc(true);
  let window: GlutinWindow = settings.build()
      .expect("Could not create window");
  
  let opengl = OpenGL::V3_2;
  let settings = WindowSettings::new("Sudoku", [512; 2])
      .graphic_api(opengl)
      .exit_on_esc(true);
  
  let mut events = Events::new(EventSettings::new().lazy(true));
  let mut gl = GlGraphics::new(opengl);
  
  let gameboard = Gameboard::new();
  let mut gameboard_controller = GameboardController::new(gameboard);
  let gameboard_view_settings = GameboardViewSettings::new();
  let gameboard_view = GameboardView::new(gameboard_view_settings);

  while let Some(e) = events.next(&mut window) {
      gameboard_controller.event(&e);
      if let Some(args) = e.render_args() {
          gl.draw(args.viewport(), |c, g| {
              use graphics::{clear};

              clear([1.0; 4], g);
              gameboard_view.draw(&gameboard_controller, &c, g);
          });
      }
  }
}
