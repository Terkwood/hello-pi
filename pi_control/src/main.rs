//MIT License

//Copyright (c) 2018 Terkwood

//Permission is hereby granted, free of charge, to any person obtaining a copy
//of this software and associated documentation files (the "Software"), to deal
//in the Software without restriction, including without limitation the rights
//to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//copies of the Software, and to permit persons to whom the Software is
//furnished to do so, subject to the following conditions:

//The above copyright notice and this permission notice shall be included in all
//copies or substantial portions of the Software.

//THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//SOFTWARE.

extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
#[macro_use]
extern crate imgui;
extern crate imgui_gfx_renderer;
extern crate imgui_sys;

use imgui::*;

mod support_gfx;

struct State {
    show_app_about: bool,
    no_titlebar: bool,
    no_resize: bool,
    no_move: bool,
    no_scrollbar: bool,
    no_collapse: bool,
    no_menu: bool,
    no_close: bool,
    color_edit: ColorEditState,
}

impl Default for State {
    fn default() -> Self {
        State {
            show_app_about: false,
            no_titlebar: false,
            no_resize: false,
            no_move: false,
            no_scrollbar: false,
            no_collapse: false,
            no_menu: false,
            no_close: false,
            color_edit: ColorEditState::default(),
        }
    }
}

struct ColorEditState {
    color: [f32; 4],
    alpha: bool,
    alpha_bar: bool,
    side_preview: bool,
    ref_color: bool,
    ref_color_v: [f32; 4],
}

impl Default for ColorEditState {
    fn default() -> Self {
        ColorEditState {
            color: [0.0 / 255.0, 255.0 / 255.0, 94.0 / 255.0, 255.0 / 255.0],
            alpha: true,
            alpha_bar: true,
            side_preview: true,
            ref_color: false,
            ref_color_v: [1.0, 0.0, 1.0, 0.5],
        }
    }
}

const CLEAR_COLOR: [f32; 4] = [114.0 / 255.0, 144.0 / 255.0, 154.0 / 255.0, 1.0];

fn main() {
    let mut state = State::default();

    support_gfx::run("RGB LEDs on Raspberry Pi".to_owned(), CLEAR_COLOR, |ui| {
        let mut open = true;
        show_test_window(ui, &mut state, &mut open);
        open
    });
}

fn show_test_window(ui: &Ui, state: &mut State, opened: &mut bool) {
    if state.show_app_about {
        ui.window(im_str!("About ImGui"))
            .always_auto_resize(true)
            .opened(&mut state.show_app_about)
            .build(|| {
                ui.text(format!("dear imgui, {}", imgui::get_version()));
                ui.separator();
                ui.text("By Omar Cornut and all github contributors.");
                ui.text(
                    "ImGui is licensed under the MIT License, see LICENSE for more \
                     information.",
                );
            });
    }

    let mut window = ui
        .window(im_str!("Raspberry Pi"))
        .title_bar(!state.no_titlebar)
        .resizable(!state.no_resize)
        .movable(!state.no_move)
        .scroll_bar(!state.no_scrollbar)
        .collapsible(!state.no_collapse)
        .menu_bar(!state.no_menu)
        .size((700.0, 680.0), ImGuiCond::FirstUseEver);
    if !state.no_close {
        window = window.opened(opened)
    }
    window.build(|| {
        ui.push_item_width(-140.0);
        ui.menu_bar(|| {
            ui.menu(im_str!("Help")).build(|| {
                ui.menu_item(im_str!("About ImGui"))
                    .selected(&mut state.show_app_about)
                    .build();
            });
        });
        ui.spacing();

        let s = &mut state.color_edit;
        let misc_flags = {
            let mut f = ImGuiColorEditFlags::empty();
            f.set(ImGuiColorEditFlags::AlphaPreview, true);
            f
        };
        ui.text(im_str!("Pick a color"));

        let mut b = ui
            .color_picker(im_str!("MyColor##4"), &mut s.color)
            .flags(misc_flags)
            .alpha(s.alpha)
            .alpha_bar(s.alpha_bar)
            .side_preview(s.side_preview)
            .rgb(true);

        if s.ref_color {
            b = b.reference_color(&s.ref_color_v)
        }
        b.build();
    })
}
