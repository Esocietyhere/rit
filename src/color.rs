use clap::Parser;
use std::io::Write;
use termcolor::{BufferWriter, Color as c, ColorChoice, ColorSpec, WriteColor};

#[derive(Debug, Parser)]
pub struct Color {
    pub color_name: Option<c>,
}

impl Color {
    pub fn green() -> Self {
        Color {
            color_name: Some(c::Green),
        }
    }

    pub fn red() -> Self {
        Color {
            color_name: Some(c::Red),
        }
    }

    pub fn yellow() -> Self {
        Color {
            color_name: Some(c::Yellow),
        }
    }

    pub fn paint(&self, text: &str) -> String {
        let bufwtr = BufferWriter::stderr(ColorChoice::Always);
        let mut buffer = bufwtr.buffer();

        let mut style = ColorSpec::new();
        style.set_fg(self.color_name).set_bold(true);

        buffer.set_color(&style).unwrap();
        buffer.write_all(text.as_bytes()).unwrap();
        buffer.reset().unwrap();

        String::from_utf8(buffer.as_slice().to_vec()).unwrap()
    }
}
