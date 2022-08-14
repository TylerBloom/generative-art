use noise::{NoiseFn, OpenSimplex, Seedable};
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use rust_gen_art::{
    circle::Circle,
    group::{Group, GroupStyle},
    line::Line,
    palette::Color,
    point::Point,
    pointmap::PointMap,
    rectangle::Rectangle,
    Shape, SVG,
};

fn main() {
    const WIDTH: f64 = 3000.0;
    const HEIGHT: f64 = WIDTH * 1.4;
    const PADDING: f64 = WIDTH / 10.0;
    const MAX_LINE_LENGTH: f64 = 800.0;
    const MIN_LINE_LENGHT: f64 = 150.0;

    let mut svg = SVG::new("Forces", WIDTH, HEIGHT);
    let mut rng = ChaCha20Rng::from_entropy();

    let mut point_map: PointMap<Circle> = PointMap::new(WIDTH, HEIGHT, 30);

    let bounds = Rectangle {
        x: PADDING,
        y: PADDING,
        width: WIDTH - (PADDING * 2.0),
        height: HEIGHT - (PADDING * 2.0),
        color: Rectangle::default().color,
    };

    let noise = OpenSimplex::new();
    Seedable::set_seed(noise, rng.gen_range(1..100_000));

    let distort = rng.gen_range(1.5..3.2);
    let zoom = rng.gen_range(500.0..900.0);

    let mut group = Group::new();

    group.set_style(GroupStyle {
        fill: None,
        stroke: Some(Color::Hex("#111")),
        stroke_width: Some(15.0),
    });

    for _ in 0..1000 {
        let mut x: f64 = rng.gen_range(PADDING..WIDTH - PADDING);
        let mut y: f64 = rng.gen_range(PADDING..HEIGHT - PADDING);
        let r = 90.0;
        let line_padding = 120.0;
        let step_size = 25.0;
        let mut line = Line {
            points: vec![],
            stroke_width: r,
            color: None,
        };

        while bounds.contains(Point { x, y }) && line.length() < MAX_LINE_LENGTH {
            let n = noise.get([x / zoom, y / zoom]);
            x += (distort * n).cos() * step_size;
            y += (distort * n).sin() * step_size;
            let circle = Circle::new(x, y, r);

            if let Some(neighbors) = point_map.get_neighbors(circle) {
                let collides_with_any = neighbors
                    .iter()
                    .any(|point| circle.distance(point) < line_padding);

                if collides_with_any {
                    break;
                }
            }

            line.add_point(Point { x, y });
        }

        if line.length() > MIN_LINE_LENGHT {
            line.points.iter().for_each(|point| {
                let _ = point_map.insert(Circle::new(point.x, point.y, r));
            });

            group.add_shape(Box::new(line));
        }
    }

    svg.add_group(Box::new(group));

    svg.save();
}
