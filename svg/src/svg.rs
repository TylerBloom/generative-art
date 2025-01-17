use std::{fmt::Write, fs::File};

use chrono::{Datelike, Utc};
use shapes::{rectangle::Rectangle, shape::Shape};

use crate::group::Group;

#[derive(Debug, Clone, PartialEq)]
pub struct SVG {
    pub name: String,
    bounds: Rectangle,
    document: String,
}

impl SVG {
    pub fn new(name: String, bounds: Rectangle) -> SVG {
        SVG {
            name,
            bounds,
            document: format!(
                "<svg viewBox=\"0 0 {} {}\" xmlns=\"http://www.w3.org/2000/svg\">",
                bounds.width, bounds.height
            ),
        }
    }

    pub fn add_shape(&mut self, shape: Box<dyn Shape>) {
        self.document.push_str(&shape.as_svg());
    }

    pub fn add_group(&mut self, group: Group) {
        self.document.push_str(&group.as_svg());
    }

    pub fn get_bounds(&self) -> Rectangle {
        self.bounds
    }

    pub fn save(&mut self, config: Option<String>) {
        self.document.push_str("</svg>");

        if let Some(comment) = config {
            let _ = write!(self.document, "{comment}");
        }

        let now = Utc::now();
        let time = format!(
            "{}-{}-{}-{}",
            now.year(),
            now.month(),
            now.day(),
            now.timestamp_millis()
        );

        let path = format!(
            "./output/{}/{}-{}.svg",
            self.name.to_lowercase(),
            self.name,
            time
        );

        let _result = std::fs::create_dir(format!("./output/{}", self.name.to_lowercase()));

        let mut f = File::create(&path).expect("could not open file for writing");

        use std::io::Write;
        f.write_all(self.document.as_bytes())
            .expect("Could not write to file");
    }
}
