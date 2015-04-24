// Copyright 2015 The Athena Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate zeus;
extern crate conrod;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate piston;
extern crate elmesque;

use conrod::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{ GlGraphics, OpenGL };
use opengl_graphics::glyph_cache::GlyphCache;
use piston::event::*;
use piston::window::{ WindowSettings, Size };
use std::path::{Path, PathBuf};
use zeus::project::*;

fn main() {
    let opengl = OpenGL::_3_2;
    let window = GlutinWindow::new(
        opengl,
        WindowSettings::new(
            "Athena".to_string(),
            Size { width: 500, height: 200 }
        )
        .exit_on_esc(true)
        .samples(4)
    );
    let event_iter = window.events().ups(180).max_fps(60);
    let mut gl = GlGraphics::new(opengl);

    let font_path = Path::new("./assets/NotoSans/NotoSans-Regular.ttf");
    let theme = Theme::default();
    let glyph_cache = GlyphCache::new(&font_path).unwrap();
    let ui = &mut Ui::new(glyph_cache, theme);

    // UI model variables
    let mut path = String::new();
    let mut status_label = String::new();

    for event in event_iter {
        ui.handle_event(&event);
        if let Some(args) = event.render_args() {
            gl.draw(args.viewport(), |_, gl| {
                // Draw the background
                Background::new().rgb(0.082, 0.090, 0.094).draw(ui, gl);

                // Draw a path textbox
                TextBox::new(&mut path)
                    .xy(0.0, 50.0)
                    .dimensions(400.0, 30.0)
                    .font_size(14)
                    .react(|_string: &mut String|{})
                    .set(0, ui);

                Button::new()
                    .xy(-100.0, 0.0)
                    .dimensions(100.0, 30.0)
                    .label("New")
                    .react(|| status_label = new_pressed(&path))
                    .set(1, ui);

                Button::new()
                    .xy(100.0, 0.0)
                    .dimensions(100.0, 30.0)
                    .label("Open")
                    .react(|| status_label = open_pressed(&path))
                    .set(2, ui);

                Label::new(&status_label)
                    .xy(0.0, -50.0)
                    .size(16)
                    .color(elmesque::color::Color::Rgba(1.0, 1.0, 1.0, 1.0))
                    .set(3, ui);

                // Draw our UI
                ui.draw(gl);
            });
        }
    }
}

fn new_pressed(path: &str) -> String {
    match ZeusProject::create(PathBuf::from(path)) {
        Ok(_) => String::from(format!("Created new at {}", path)),
        Err(e) => String::from(format!("Error: {}", e))
    }
}

fn open_pressed(path: &str) -> String {
    match ZeusProject::open(PathBuf::from(path)) {
        Ok(p) => String::from(format!("Opened: {}", p.game_name())),
        Err(e) => String::from(format!("Error: {}", e))
    }
}
