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

extern crate winapi;
extern crate ole32;
extern crate uuid;

use conrod::{Background, Button, Color, Colorable, Label, Labelable, Positionable, Sizeable, TextBox, Theme, Ui, Widget};
use conrod::color::{rgb};
use glutin_window::{GlutinWindow};
use opengl_graphics::{GlGraphics, OpenGL};
use opengl_graphics::glyph_cache::GlyphCache;
use piston::event::*;
use piston::window::{WindowSettings, Size};
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
            gl.draw(args.viewport(), |c, gl| {
                // Draw the background
                Background::new().color(rgb(0.082, 0.090, 0.094)).draw(ui, gl);

                // Draw a path textbox
                TextBox::new(&mut path)
                    .xy(-50.0, 50.0)
                    .dimensions(300.0, 30.0)
                    .font_size(14)
                    .react(|_string: &mut String|{})
                    .set(0, ui);

                Button::new()
                    .xy(150.0, 50.0)
                    .dimensions(100.0, 30.0)
                    .label("Browse")
                    .react(|| path = browse_pressed())
                    .set(1, ui);

                if path != "" {
                    Button::new()
                        .xy(-100.0, 0.0)
                        .dimensions(100.0, 30.0)
                        .label("New")
                        .react(|| status_label = new_pressed(&path))
                        .set(2, ui);

                    Button::new()
                        .xy(100.0, 0.0)
                        .dimensions(100.0, 30.0)
                        .label("Open")
                        .react(|| status_label = open_pressed(&path))
                        .set(3, ui);
                }

                Label::new(&status_label)
                    .xy(0.0, -50.0)
                    .font_size(16)
                    .color(rgb(1.0, 1.0, 1.0))
                    .set(4, ui);

                // Draw our UI
                ui.draw(c, gl);
            });
        }
    }
}

fn browse_pressed() -> String {
    unsafe {
        // TODO: Add cleanup code

        check_result(ole32::CoInitializeEx(::std::ptr::null_mut(), 0));

        // Create the dialog
        let file_dialog = {
            let mut file_dialog: *mut winapi::IFileOpenDialog = std::mem::uninitialized();
            let hresult = ole32::CoCreateInstance(
                &uuid::CLSID_FileOpenDialog,
                std::ptr::null_mut(), winapi::CLSCTX_ALL,
                &uuid::IID_IFileOpenDialog,
                std::mem::transmute(&mut file_dialog));

            check_result(hresult);
            &mut *file_dialog
        };

        check_result(file_dialog.SetOptions(winapi::FILEOPENDIALOGOPTIONS::PICKFOLDERS));

        // Show the dialog
        if file_dialog.Show(0 as winapi::HWND) < 0 {
            // Got canceled
            return String::from("")
        }

        // Get the file name from the dialog
        let path = {
            let mut item: *mut winapi::IShellItem = std::mem::uninitialized();
            check_result(file_dialog.GetResult(std::mem::transmute(&mut item)));

            let mut raw_path: *mut winapi::LPWSTR = std::mem::uninitialized();
            check_result((*item).GetDisplayName(winapi::SIGDN::FILESYSPATH, std::mem::transmute(&mut raw_path)));

            let os_path: std::ffi::OsString = OsStringFromPtr::from_wide_ptr(std::mem::transmute(raw_path));

            String::from(os_path.to_str().unwrap())
        };

        path
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

fn check_result(result: winapi::HRESULT) {
    if result < 0 {
        return panic!("Error in winapi call: {:x}", result);
    }
}

pub trait OsStringFromPtr {
    unsafe fn from_wide_ptr(winapi::LPWSTR) -> Self;
}

impl OsStringFromPtr for std::ffi::OsString {
    unsafe fn from_wide_ptr(ptr: winapi::LPWSTR) -> Self {
        use std::ffi::OsString;
        use std::os::windows::ffi::OsStringExt;
        assert!(!ptr.is_null());
        let mut len = 0;
        while *ptr.offset(len) != 0 {
            len += 1
        }
        let slice = std::slice::from_raw_parts(ptr, len as usize);
        OsString::from_wide(slice)
    }
}
