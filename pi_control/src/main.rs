// Copyright (c) 2018 Terkwood
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

extern crate crossbeam_channel as channel;
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
#[macro_use]
extern crate imgui;
extern crate imgui_gfx_renderer;
extern crate imgui_sys;
extern crate redis;

mod model;
mod redis_publish;
mod support_gfx;

use imgui::*;
use model::SetRGB;

use std::thread;

struct State {
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
            no_titlebar: false,
            no_resize: false,
            no_move: false,
            no_scrollbar: false,
            no_collapse: false,
            no_menu: true,
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
            color: [47.0 / 255.0, 181.0 / 255.0, 47.0 / 255.0, 255.0 / 255.0],
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
    let (redis_s, redis_r) = channel::bounded(5);
    thread::spawn(move || redis_publish::run(redis_r));

    support_gfx::run("RGB LEDs on Raspberry Pi".to_owned(), CLEAR_COLOR, |ui| {
        let orig_rgb_widget_color = &state.color_edit.color.clone()[..];

        let mut open = true;
        show_test_window(ui, &mut state, &mut open);

        if &orig_rgb_widget_color[..] != &state.color_edit.color {
            redis_s.send(SetRGB {
                color: state.color_edit.color,
            })
        }

        open
    });
}

fn show_test_window(ui: &Ui, state: &mut State, opened: &mut bool) {
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
        let s = &mut state.color_edit;
        let misc_flags = {
            let mut f = ImGuiColorEditFlags::empty();
            f.set(ImGuiColorEditFlags::AlphaPreview, true);
            f
        };
        ui.text(im_str!("Pick a color"));

        let mut b = ui
            .color_picker(im_str!("RGB##1"), &mut s.color)
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
