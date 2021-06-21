use chrono::Duration;
use conrod_core::Theme;
use piston_window::PistonWindow;
use sdl2::video::FullscreenType;
use sdl2_window::Sdl2Window;
use std::path::PathBuf;

trait Store {
    fn name(&self) -> String;
    fn location(&self) -> PathBuf { PathBuf::from(self.name()) }
    fn loc1(_: (&Self, String)) -> (Self, String)
    where
        Self: Store + Sized;
    fn save(&self);
    fn load() -> Self
    where
        Self: Default, {
        Self::default()
    }
}
#[inline]
pub fn theme() -> Theme {
    use conrod_core::position::{Align, Direction, Padding, Position, Relative};
    conrod_core::Theme {
        name:                   "Demo Theme".to_string(),
        padding:                Padding::none(),
        x_position:             Position::Relative(
            Relative::Align(Align::Start),
            None,
        ),
        y_position:             Position::Relative(
            Relative::Direction(Direction::Backwards, 20.0),
            None,
        ),
        background_color:       conrod_core::color::DARK_CHARCOAL,
        shape_color:            conrod_core::color::LIGHT_CHARCOAL,
        border_color:           conrod_core::color::BLACK,
        border_width:           0.0,
        label_color:            conrod_core::color::WHITE,
        font_id:                None,
        font_size_large:        26,
        font_size_medium:       18,
        font_size_small:        12,
        widget_styling:         conrod_core::theme::StyleMap::default(),
        mouse_drag_threshold:   0.0,
        double_click_threshold: std::time::Duration::from_millis(300),
    }
}
#[inline]
pub fn fullscreen(window: &mut PistonWindow<Sdl2Window>) {
    match window.window.window.fullscreen_state() {
        FullscreenType::Off => {
            &window.window.window.set_fullscreen(FullscreenType::Desktop)
        }
        FullscreenType::True => {
            &window.window.window.set_fullscreen(FullscreenType::Desktop)
        }
        FullscreenType::Desktop => {
            &window.window.window.set_fullscreen(FullscreenType::Off)
        }
    };
}
#[inline]
pub fn duration() -> Duration { Duration::seconds(2) }
