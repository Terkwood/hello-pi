use sys;
use sys::{ImDrawCornerFlags, ImDrawList, ImU32};

use super::{ImVec2, ImVec4, Ui};

use std::marker::PhantomData;

/// Wrap `ImU32` (a type typically used by ImGui to store packed colors)
/// This type is used to represent the color of drawing primitives in ImGui's
/// custom drawing API.
///
/// The type implements `From<ImU32>`, `From<ImVec4>`, `From<[f32; 4]>`,
/// `From<[f32; 3]>`, `From<(f32, f32, f32, f32)>` and `From<(f32, f32, f32)>`
/// for convenience. If alpha is not provided, it is assumed to be 1.0 (255).
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct ImColor(ImU32);

impl From<ImColor> for ImU32 {
    fn from(color: ImColor) -> Self { color.0 }
}

impl From<ImU32> for ImColor {
    fn from(color: ImU32) -> Self { ImColor(color) }
}

impl From<ImVec4> for ImColor {
    fn from(v: ImVec4) -> Self { ImColor(unsafe { sys::igColorConvertFloat4ToU32(v) }) }
}

impl From<[f32; 4]> for ImColor {
    fn from(v: [f32; 4]) -> Self { ImColor(unsafe { sys::igColorConvertFloat4ToU32(v.into()) }) }
}

impl From<(f32, f32, f32, f32)> for ImColor {
    fn from(v: (f32, f32, f32, f32)) -> Self {
        ImColor(unsafe { sys::igColorConvertFloat4ToU32(v.into()) })
    }
}

impl From<[f32; 3]> for ImColor {
    fn from(v: [f32; 3]) -> Self { [v[0], v[1], v[2], 1.0].into() }
}

impl From<(f32, f32, f32)> for ImColor {
    fn from(v: (f32, f32, f32)) -> Self { [v.0, v.1, v.2, 1.0].into() }
}

/// Object implementing the custom draw API.
///
/// Called from [`Ui::get_window_draw_list`]. No more than one instance of this
/// structure can live in a program at the same time.
/// The program will panic on creating a second instance.
pub struct WindowDrawList<'ui> {
    draw_list: *mut ImDrawList,
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

static mut WINDOW_DRAW_LIST_LOADED: bool = false;

impl<'ui> Drop for WindowDrawList<'ui> {
    fn drop(&mut self) {
        unsafe { WINDOW_DRAW_LIST_LOADED = false; }
    }
}

impl<'ui> WindowDrawList<'ui> {
    pub(crate) fn new(_: &Ui<'ui>) -> Self {
        unsafe {
            if WINDOW_DRAW_LIST_LOADED {
                panic!("WindowDrawList is already loaded! You can only load one instance of it!")
            }
            WINDOW_DRAW_LIST_LOADED = true;
        }
        Self {
            draw_list: unsafe { sys::igGetWindowDrawList() },
            _phantom: PhantomData,
        }
    }

    /// Split into *channels_count* drawing channels.
    /// At the end of the closure, the channels are merged. The objects
    /// are then drawn in the increasing order of their channel number, and not
    /// in the order they were called.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use imgui::*;
    /// fn custom_drawing(ui: &Ui) {
    ///     let draw_list = ui.get_window_draw_list();
    ///     draw_list.channels_split(2, |channels| {
    ///         channels.set_current(1);
    ///         // ... Draw channel 1
    ///         channels.set_current(0);
    ///         // ... Draw channel 0
    ///     });
    /// }
    /// ```
    pub fn channels_split<F: FnOnce(&ChannelsSplit)>(&self, channels_count: u32, f: F) {
        unsafe { sys::ImDrawList_ChannelsSplit(self.draw_list, channels_count as i32) };
        f(&ChannelsSplit {
            draw_list: self,
            channels_count,
        });
        unsafe { sys::ImDrawList_ChannelsMerge(self.draw_list) };
    }
}

/// Represent the drawing interface within a call to [`channels_split`].
///
/// [`channels_split`]: WindowDrawList::channels_split
pub struct ChannelsSplit<'ui> {
    draw_list: &'ui WindowDrawList<'ui>,
    channels_count: u32,
}

impl<'ui> ChannelsSplit<'ui> {
    /// Change current channel.
    ///
    /// Panic if channel_index overflows the number of channels.
    pub fn set_current(&self, channel_index: u32) {
        assert!(
            channel_index < self.channels_count,
            "Channel cannot be set! Provided channel index ({}) is higher than channel count ({}).",
            channel_index,
            self.channels_count
        );
        unsafe {
            sys::ImDrawList_ChannelsSetCurrent(self.draw_list.draw_list, channel_index as i32)
        };
    }
}

/// Drawing functions
impl<'ui> WindowDrawList<'ui> {
    /// Returns a line from point `p1` to `p2` with color `c`.
    pub fn add_line<P1, P2, C>(&'ui self, p1: P1, p2: P2, c: C) -> Line<'ui>
    where
        P1: Into<ImVec2>,
        P2: Into<ImVec2>,
        C: Into<ImColor>,
    {
        Line::new(self, p1, p2, c)
    }

    /// Returns a rectangle whose upper-left corner is at point `p1`
    /// and lower-right corner is at point `p2`, with color `c`.
    pub fn add_rect<P1, P2, C>(&'ui self, p1: P1, p2: P2, c: C) -> Rect<'ui>
    where
        P1: Into<ImVec2>,
        P2: Into<ImVec2>,
        C: Into<ImColor>,
    {
        Rect::new(self, p1, p2, c)
    }

    /// Draw a rectangle whose upper-left corner is at point `p1`
    /// and lower-right corner is at point `p2`.
    /// The remains parameters are the respective color of the corners
    /// in the counter-clockwise starting from the upper-left corner
    /// first.
    pub fn add_rect_filled_multicolor<P1, P2, C1, C2, C3, C4>(
        &self,
        p1: P1,
        p2: P2,
        col_upr_left: C1,
        col_upr_right: C2,
        col_bot_right: C3,
        col_bot_left: C4,
    ) where
        P1: Into<ImVec2>,
        P2: Into<ImVec2>,
        C1: Into<ImColor>,
        C2: Into<ImColor>,
        C3: Into<ImColor>,
        C4: Into<ImColor>,
    {
        unsafe {
            sys::ImDrawList_AddRectFilledMultiColor(
                self.draw_list,
                p1.into(),
                p2.into(),
                col_upr_left.into().into(),
                col_upr_right.into().into(),
                col_bot_right.into().into(),
                col_bot_left.into().into(),
            );
        }
    }

    /// Returns a triangle with the given 3 vertices `p1`, `p2` and `p3`
    /// and color `c`.
    pub fn add_triangle<P1, P2, P3, C>(&'ui self, p1: P1, p2: P2, p3: P3, c: C) -> Triangle<'ui>
    where
        P1: Into<ImVec2>,
        P2: Into<ImVec2>,
        P3: Into<ImVec2>,
        C: Into<ImColor>,
    {
        Triangle::new(self, p1, p2, p3, c)
    }

    /// Returns a circle with the given `center`, `radius` and `color`.
    pub fn add_circle<P, C>(&'ui self, center: P, radius: f32, color: C) -> Circle<'ui>
    where
        P: Into<ImVec2>,
        C: Into<ImColor>,
    {
        Circle::new(self, center, radius, color)
    }

    /// Draw a text whose upper-left corner is at point `pos`.
    pub fn add_text<P, C, T>(&self, pos: P, col: C, text: T)
    where
        P: Into<ImVec2>,
        C: Into<ImColor>,
        T: AsRef<str>,
    {
        use std::os::raw::c_char;

        let text = text.as_ref();
        unsafe {
            let start = text.as_ptr() as *const c_char;
            let end = (start as usize + text.len()) as *const c_char;
            sys::ImDrawList_AddText(self.draw_list, pos.into(), col.into().into(), start, end)
        }
    }

    /// Returns a Bezier curve stretching from `pos0` to `pos1`, whose
    /// curvature is defined by `cp0` and `cp1`.
    pub fn add_bezier_curve<P1, P2, P3, P4, C>(
        &'ui self,
        pos0: P1,
        cp0: P2,
        cp1: P3,
        pos1: P4,
        color: C,
    ) -> BezierCurve<'ui>
    where
        P1: Into<ImVec2>,
        P2: Into<ImVec2>,
        P3: Into<ImVec2>,
        P4: Into<ImVec2>,
        C: Into<ImColor>,
    {
        BezierCurve::new(self, pos0, cp0, cp1, pos1, color)
    }

    /// Push a clipping rectangle on the stack, run `f` and pop it.
    ///
    /// Clip all drawings done within the closure `f` in the given
    /// rectangle.
    pub fn with_clip_rect<P1, P2, F>(&self, min: P1, max: P2, f: F)
    where
        P1: Into<ImVec2>,
        P2: Into<ImVec2>,
        F: FnOnce(),
    {
        unsafe { sys::ImDrawList_PushClipRect(self.draw_list, min.into(), max.into(), false) }
        f();
        unsafe { sys::ImDrawList_PopClipRect(self.draw_list) }
    }

    /// Push a clipping rectangle on the stack, run `f` and pop it.
    ///
    /// Clip all drawings done within the closure `f` in the given
    /// rectangle. Intersect with all clipping rectangle previously on
    /// the stack.
    pub fn with_clip_rect_intersect<P1, P2, F>(&self, min: P1, max: P2, f: F)
    where
        P1: Into<ImVec2>,
        P2: Into<ImVec2>,
        F: FnOnce(),
    {
        unsafe { sys::ImDrawList_PushClipRect(self.draw_list, min.into(), max.into(), true) }
        f();
        unsafe { sys::ImDrawList_PopClipRect(self.draw_list) }
    }
}

/// Represents a line about to be drawn
#[must_use = "should call .build() to draw the object"]
pub struct Line<'ui> {
    p1: ImVec2,
    p2: ImVec2,
    color: ImColor,
    thickness: f32,
    draw_list: &'ui WindowDrawList<'ui>,
}

impl<'ui> Line<'ui> {
    fn new<P1, P2, C>(draw_list: &'ui WindowDrawList, p1: P1, p2: P2, c: C) -> Self
    where
        P1: Into<ImVec2>,
        P2: Into<ImVec2>,
        C: Into<ImColor>,
    {
        Self {
            p1: p1.into(),
            p2: p2.into(),
            color: c.into(),
            thickness: 1.0,
            draw_list,
        }
    }

    /// Set line's thickness (default to 1.0 pixel)
    pub fn thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    /// Draw the line on the window
    pub fn build(self) {
        unsafe {
            sys::ImDrawList_AddLine(
                self.draw_list.draw_list,
                self.p1,
                self.p2,
                self.color.into(),
                self.thickness,
            )
        }
    }
}

/// Represents a rectangle about to be drawn
#[must_use = "should call .build() to draw the object"]
pub struct Rect<'ui> {
    p1: ImVec2,
    p2: ImVec2,
    color: ImColor,
    rounding: f32,
    flags: ImDrawCornerFlags,
    thickness: f32,
    filled: bool,
    draw_list: &'ui WindowDrawList<'ui>,
}

impl<'ui> Rect<'ui> {
    fn new<P1, P2, C>(draw_list: &'ui WindowDrawList, p1: P1, p2: P2, c: C) -> Self
    where
        P1: Into<ImVec2>,
        P2: Into<ImVec2>,
        C: Into<ImColor>,
    {
        Self {
            p1: p1.into(),
            p2: p2.into(),
            color: c.into(),
            rounding: 0.0,
            flags: ImDrawCornerFlags::All,
            thickness: 1.0,
            filled: false,
            draw_list,
        }
    }

    /// Set rectangle's corner rounding (default to 0.0: no rounding).
    /// By default all corners are rounded if this value is set.
    pub fn rounding(mut self, rounding: f32) -> Self {
        self.rounding = rounding;
        self
    }

    /// Set flag to indicate if rectangle's top-left corner will be rounded.
    pub fn round_top_left(mut self, value: bool) -> Self {
        self.flags.set(ImDrawCornerFlags::TopLeft, value);
        self
    }

    /// Set flag to indicate if rectangle's top-right corner will be rounded.
    pub fn round_top_right(mut self, value: bool) -> Self {
        self.flags.set(ImDrawCornerFlags::TopRight, value);
        self
    }

    /// Set flag to indicate if rectangle's bottom-left corner will be rounded.
    pub fn round_bot_left(mut self, value: bool) -> Self {
        self.flags.set(ImDrawCornerFlags::BotLeft, value);
        self
    }

    /// Set flag to indicate if rectangle's bottom-right corner will be rounded.
    pub fn round_bot_right(mut self, value: bool) -> Self {
        self.flags.set(ImDrawCornerFlags::BotRight, value);
        self
    }

    /// Set rectangle's thickness (default to 1.0 pixel).
    pub fn thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    /// Set to `true` to make a filled rectangle (default to `false`).
    pub fn filled(mut self, filled: bool) -> Self {
        self.filled = filled;
        self
    }

    /// Draw the rectangle on the window.
    pub fn build(self) {
        if self.filled {
            unsafe {
                sys::ImDrawList_AddRectFilled(
                    self.draw_list.draw_list,
                    self.p1,
                    self.p2,
                    self.color.into(),
                    self.rounding,
                    self.flags,
                );
            }
        } else {
            unsafe {
                sys::ImDrawList_AddRect(
                    self.draw_list.draw_list,
                    self.p1,
                    self.p2,
                    self.color.into(),
                    self.rounding,
                    self.flags,
                    self.thickness,
                );
            }
        }
    }
}

/// Represents a triangle about to be drawn on the window
#[must_use = "should call .build() to draw the object"]
pub struct Triangle<'ui> {
    p1: ImVec2,
    p2: ImVec2,
    p3: ImVec2,
    color: ImColor,
    thickness: f32,
    filled: bool,
    draw_list: &'ui WindowDrawList<'ui>,
}

impl<'ui> Triangle<'ui> {
    fn new<P1, P2, P3, C>(draw_list: &'ui WindowDrawList, p1: P1, p2: P2, p3: P3, c: C) -> Self
    where
        P1: Into<ImVec2>,
        P2: Into<ImVec2>,
        P3: Into<ImVec2>,
        C: Into<ImColor>,
    {
        Self {
            p1: p1.into(),
            p2: p2.into(),
            p3: p3.into(),
            color: c.into(),
            thickness: 1.0,
            filled: false,
            draw_list,
        }
    }

    /// Set triangle's thickness (default to 1.0 pixel)
    pub fn thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    /// Set to `true` to make a filled triangle (default to `false`).
    pub fn filled(mut self, filled: bool) -> Self {
        self.filled = filled;
        self
    }

    /// Draw the triangle on the window.
    pub fn build(self) {
        if self.filled {
            unsafe {
                sys::ImDrawList_AddTriangleFilled(
                    self.draw_list.draw_list,
                    self.p1,
                    self.p2,
                    self.p3,
                    self.color.into(),
                )
            }
        } else {
            unsafe {
                sys::ImDrawList_AddTriangle(
                    self.draw_list.draw_list,
                    self.p1,
                    self.p2,
                    self.p3,
                    self.color.into(),
                    self.thickness,
                )
            }
        }
    }
}

/// Represents a circle about to be drawn
#[must_use = "should call .build() to draw the object"]
pub struct Circle<'ui> {
    center: ImVec2,
    radius: f32,
    color: ImColor,
    num_segments: u32,
    thickness: f32,
    filled: bool,
    draw_list: &'ui WindowDrawList<'ui>,
}

impl<'ui> Circle<'ui> {
    pub fn new<P, C>(draw_list: &'ui WindowDrawList, center: P, radius: f32, color: C) -> Self
    where
        P: Into<ImVec2>,
        C: Into<ImColor>,
    {
        Self {
            center: center.into(),
            radius,
            color: color.into(),
            num_segments: 12,
            thickness: 1.0,
            filled: false,
            draw_list,
        }
    }

    /// Set number of segment used to draw the circle, default to 12.
    /// Add more segments if you want a smoother circle.
    pub fn num_segments(mut self, num_segments: u32) -> Self {
        self.num_segments = num_segments;
        self
    }

    /// Set circle's thickness (default to 1.0 pixel)
    pub fn thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    /// Set to `true` to make a filled circle (default to `false`).
    pub fn filled(mut self, filled: bool) -> Self {
        self.filled = filled;
        self
    }

    /// Draw the circle on the window.
    pub fn build(self) {
        if self.filled {
            unsafe {
                sys::ImDrawList_AddCircleFilled(
                    self.draw_list.draw_list,
                    self.center,
                    self.radius,
                    self.color.into(),
                    self.num_segments as i32,
                )
            }
        } else {
            unsafe {
                sys::ImDrawList_AddCircle(
                    self.draw_list.draw_list,
                    self.center,
                    self.radius,
                    self.color.into(),
                    self.num_segments as i32,
                    self.thickness,
                )
            }
        }
    }
}

/// Represents a Bezier curve about to be drawn
#[must_use = "should call .build() to draw the object"]
pub struct BezierCurve<'ui> {
    pos0: ImVec2,
    cp0: ImVec2,
    pos1: ImVec2,
    cp1: ImVec2,
    color: ImColor,
    thickness: f32,
    /// If num_segments is not set, the bezier curve is auto-tessalated.
    num_segments: Option<u32>,
    draw_list: &'ui WindowDrawList<'ui>,
}

impl<'ui> BezierCurve<'ui> {
    fn new<P1, P2, P3, P4, C>(
        draw_list: &'ui WindowDrawList,
        pos0: P1,
        cp0: P2,
        cp1: P3,
        pos1: P4,
        c: C,
    ) -> Self
    where
        P1: Into<ImVec2>,
        P2: Into<ImVec2>,
        P3: Into<ImVec2>,
        P4: Into<ImVec2>,
        C: Into<ImColor>,
    {
        Self {
            pos0: pos0.into(),
            cp0: cp0.into(),
            pos1: pos1.into(),
            cp1: cp1.into(),
            color: c.into(),
            thickness: 1.0,
            num_segments: None,
            draw_list,
        }
    }

    /// Set curve's thickness (default to 1.0 pixel)
    pub fn thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    /// Set number of segments used to draw the Bezier curve. If not set, the
    /// bezier curve is auto-tessalated.
    pub fn num_segments(mut self, num_segments: u32) -> Self {
        self.num_segments = Some(num_segments);
        self
    }

    /// Draw the curve on the window.
    pub fn build(self) {
        unsafe {
            sys::ImDrawList_AddBezierCurve(
                self.draw_list.draw_list,
                self.pos0,
                self.cp0,
                self.cp1,
                self.pos1,
                self.color.into(),
                self.thickness,
                self.num_segments.unwrap_or(0) as i32,
            )
        }
    }
}
