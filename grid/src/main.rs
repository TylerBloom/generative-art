#![warn(rust_2018_idioms)]
#![deny(
    dead_code,
    // NOTE: This is very helpful to include
    //missing_docs,
    unused_variables,
    unused_imports,
    unused_import_braces,
    rustdoc::broken_intra_doc_links,
    missing_debug_implementations,
    unreachable_pub
)]

use palette::Color;
use shapes::{circle::Circle, point::Point};

use std::sync::mpsc::channel;
use threadpool::ThreadPool;

use rand::{thread_rng, Rng};
use shapes::rectangle::Rectangle;
use svg::svg::SVG;

fn main() {
    let bounds = Rectangle {
        x: 0.0,
        y: 0.0,
        width: 1000.0,
        height: 1000.0 * 1.4,
        color: None,
    };

    let inner_bounds = bounds.scale(0.9);
    let mut rects = vec![];
    let mut document = SVG::new("Grid".into(), bounds);
    let mut rng = rand::thread_rng();

    let mut x = inner_bounds.x;

    while inner_bounds.x_range().contains(&x) {
        let block_width = rng.gen_range(bounds.width * 0.003..bounds.width * 0.04);
        let mut y = inner_bounds.y;

        while inner_bounds.y_range().contains(&y) {
            let block_height = if rng.gen_bool(0.2) {
                bounds.height * rng.gen_range(0.03..0.045)
            } else {
                bounds.height * rng.gen_range(0.002..0.01)
            };

            let rect = Rectangle::new(x, y, block_width, block_height);
            rects.push(rect);
            y += block_height;
        }
        x += block_width;
    }

    let count = rects.len();
    let pool = ThreadPool::new(count);
    let (sender, receiver) = channel::<Vec<Circle>>();
    for rect in rects {
        let sender = sender.clone();
        pool.execute(move || {
            let mut thread_rng = thread_rng();
            let mut points = vec![];
            let dots = get_dot_count(&rect, bounds.height);
            for _ in 0..dots {
                let mut circle = Circle::new(
                    Point {
                        x: thread_rng.gen_range(rect.x_range()),
                        y: thread_rng.gen_range(rect.y_range()),
                    },
                    0.5,
                );

                circle.set_color(Color::Hex("#1115"));

                points.push(circle);
            }
            sender.send(points).expect("error");
        });
    }

    for circle in receiver.iter().take(count).flatten() {
        document.add_shape(Box::new(circle));
    }

    document.save(None);
}

fn get_dot_count(rect: &Rectangle, render_height: f64) -> i32 {
    let area_str = format!("{}", rect.area());

    let max_str_len = std::cmp::min(area_str.len(), 4);

    let normalized_area = area_str
        .get(0..max_str_len)
        .unwrap_or("0.0")
        .parse::<f64>()
        .unwrap_or(0.);

    let mut rng = rand::thread_rng();
    let count = (render_height - rect.y) * rng.gen_range(2.0..4.0) + normalized_area;

    (count as i32).min(999)
}
