use std::{fmt::Display, ops::Range};

use super::shape::Shape;
use crate::{palette::Color, point::Point};

#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,

    pub color: Option<Color>,
}

impl Rectangle {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Rectangle {
        Rectangle {
            x,
            y,
            width,
            height,
            color: Rectangle::default().color,
        }
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = Some(color);
    }

    pub fn scale(&self, scale: f64) -> Rectangle {
        let width = self.width * scale;
        let height = self.height * scale;
        let x = self.x - (width - self.width) / 2.0;
        let y = self.y - (height - self.height) / 2.0;

        Rectangle {
            x,
            y,
            width,
            height,
            color: self.color,
        }
    }

    pub fn x_range(&self) -> Range<f64> {
        self.x..(self.x + self.width)
    }

    pub fn y_range(&self) -> Range<f64> {
        self.y..(self.y + self.height)
    }

    pub fn set_width(&mut self, width: f64) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: f64) {
        self.height = height;
    }
}

impl Shape for Rectangle {
    fn as_svg(&self) -> String {
        let fill = match self.color {
            Some(color) => format!(" fill=\"{}\"", color),
            _ => String::from(""),
        };

        format!(
            "<rect x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\"{}/>",
            self.x, self.y, self.width, self.height, fill
        )
    }

    fn contains(&self, point: super::point::Point) -> bool {
        (self.x..(self.x + self.width)).contains(&point.x)
            && (self.y..(self.y + self.height)).contains(&point.y)
    }

    fn center(&self) -> Point {
        Point {
            x: (self.x + self.width) / 2.0,
            y: (self.y + self.height) / 2.0,
        }
    }

    fn bounding_box(&self) -> Rectangle {
        Rectangle {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
            color: None,
        }
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        Rectangle {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            color: None,
        }
    }
}

impl PartialEq for Rectangle {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
            && self.y == other.y
            && self.width == other.width
            && self.height == other.height
    }
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.as_svg())
    }
}

#[cfg(test)]
mod test {
    use crate::{point::Point, shape::Shape};

    use super::Rectangle;

    #[test]
    fn does_not_contain() {
        let rect = Rectangle {
            x: 0.0,
            y: 0.0,
            width: 20.0,
            height: 20.0,
            color: None,
        };

        let point = Point { x: 10.0, y: 30.0 };

        assert_eq!(rect.contains(point), false);
    }

    #[test]
    fn scale_rect_up() {
        let rect = Rectangle {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
            color: None,
        };

        let scaled = rect.scale(1.1);
        assert_eq!(
            Rectangle {
                x: scaled.x.round(),
                y: scaled.y.round(),
                width: scaled.width.round(),
                height: scaled.height.round(),
                ..scaled
            },
            Rectangle {
                x: -5.0,
                y: -5.0,
                width: 110.0,
                height: 110.0,
                color: None
            }
        );
    }

    #[test]
    fn scale_rect_down() {
        let rect = Rectangle {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
            color: None,
        };

        let scaled = rect.scale(0.9);
        assert_eq!(
            Rectangle {
                x: scaled.x.round(),
                y: scaled.y.round(),
                width: scaled.width.round(),
                height: scaled.height.round(),
                ..scaled
            },
            Rectangle {
                x: 5.0,
                y: 5.0,
                width: 90.0,
                height: 90.0,
                color: None
            }
        );
    }
}
