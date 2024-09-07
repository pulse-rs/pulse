use termcolor::{Buffer, Color, ColorSpec, WriteColor};

pub struct ColorHandler;

impl ColorHandler {
    pub fn set_color(buff: &mut Buffer, color: Color, intense: bool) {
        buff.set_color(ColorSpec::new().set_fg(Some(color)).set_intense(intense))
            .expect("Error setting color");
    }

    pub fn set_dimmed_color(buff: &mut Buffer) {
        buff.set_color(ColorSpec::new().set_dimmed(true))
            .expect("Error setting color");
    }

    pub fn reset_color(buff: &mut Buffer) {
        buff.reset().expect("Error resetting color");
    }
}
