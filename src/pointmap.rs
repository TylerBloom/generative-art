use std::vec;

use crate::{point::Point, rectangle::Rectangle, shape::Shape};

pub struct PointMap<'a, T> {
    bounds: &'a Rectangle,
    cells: Vec<Vec<T>>,
    grid_resolution: usize,
}

impl<'a, T: Shape + Clone + Copy> PointMap<'a, T> {
    pub fn new<S>(bounds: &Rectangle, resolution: usize) -> PointMap<T> {
        let map = vec![vec![]; resolution.pow(2)];

        PointMap {
            bounds,
            cells: map,
            grid_resolution: resolution,
        }
    }

    pub fn insert(&mut self, shape: T) -> Result<usize, T> {
        let i = self.get_index(shape.center());

        if let Some(points) = self.cells.get_mut(i) {
            points.push(shape);
            return Ok(i);
        }

        Err(shape)
    }

    pub fn get_items(&self) -> Vec<T> {
        self.cells.iter().fold(vec![], |mut points, cell| {
            cell.iter().for_each(|item| {
                points.push(*item);
            });
            points
        })
    }

    /**
     * The general idea here is to do some simple math to be able
     * to overlay the bounds with cell after cell until we are
     * out of bounds, when the bounds have been hit, we loop back
     * to a new row using modulo and restart the overlaying.
     *
     *  -------------------------
     *  | 0 | 1 | 2 | 3 | 4 | 5 |
     *  | 6 | 7 | 8 | 9 | 10| . |
     *  | . |   |   |   |   |   |
     *  -------------------------
     *
     * This allows us to use <Vec<Vec<T>>> instead of Vec<Vec<Vec<T>>>
     * meaning we don't have to think of the bounds as a grid,
     * but rather a list of cells.
     *
     * We also get all the surrounding cells to avoid collissions at nodes
     * close to the one where we pop over to a neigboring grid cell.
     *
     * this makes the search space larger, but yields a more accurate
     * result.
     *
     *  ----------------------
     *  |  |  |  |  |  |  |  |
     *  |xx|xx|xx|  |  |  |  |
     *  |xx|oo|xx|  |  |  |  |
     *  |xx|xx|xx|  |  |  |  |
     *  ----------------------
     */
    pub fn get_neighbors(&self, shape: T) -> Result<Vec<T>, &str> {
        if !self.bounds.contains(shape.center()) {
            return Err("out of bounds call for this pointmap");
        }

        let i = self.get_index(shape.center());

        let items = self
            .get_neigboring_cells(i)
            .iter()
            .fold(vec![], |mut list, index| {
                match self.cells.get(index.to_owned()) {
                    Some(cell_items) => {
                        cell_items
                            .iter()
                            .for_each(|item| list.push(item.to_owned()));
                        list
                    }
                    None => list,
                }
            });

        Ok(items)
    }

    fn get_index(&self, point: Point) -> usize {
        let resolution = self.grid_resolution as f64;

        let x = ((point.x / (self.bounds.x + self.bounds.width)) * resolution).floor();
        let y = ((point.y / (self.bounds.y + self.bounds.height)) * resolution).floor();

        (y * resolution + x - 1.0) as usize
    }

    fn get_neigboring_cells(&self, index: usize) -> Vec<usize> {
        let i = index as i32;
        let step = (self.cells.len() as f64).sqrt() as i32;
        let over = i - step;
        let under = i + step;

        vec![
            over - 1,
            over,
            over + 1,
            i - 1,
            i,
            i + 1,
            under - 1,
            under,
            under + 1,
        ]
        .into_iter()
        .filter(|cell| cell > &0 || (cell.to_owned() as usize) < self.cells.len())
        .map(|cell| cell as usize)
        .collect::<Vec<usize>>()
    }
}

#[cfg(test)]
mod test {

    use crate::{circle::Circle, point::Point, pointmap::PointMap, rectangle::Rectangle};

    #[test]
    fn get_index() {
        let bounds = Rectangle {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
            color: Rectangle::default().color,
        };
        let point_map: PointMap<Circle> = PointMap::new::<Circle>(&bounds, 10);
        assert_eq!(point_map.get_index(Point { x: 9.0, y: 0.0 }), 0);
        assert_eq!(point_map.get_index(Point { x: 11.0, y: 0.0 }), 0);
        assert_eq!(point_map.get_index(Point { x: 20.0, y: 0.0 }), 1);
        assert_eq!(point_map.get_index(Point { x: 34.0, y: 0.0 }), 2);
        assert_eq!(point_map.get_index(Point { x: 99.999, y: 0.0 }), 8);
    }

    #[test]
    fn insert_point() {
        let bounds = Rectangle {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
            color: Rectangle::default().color,
        };
        let mut point_map: PointMap<Circle> = PointMap::new::<Circle>(&bounds, 10);
        let circle = Circle::new(Point { x: 11.0, y: 11.0 }, 10.0);
        let result = point_map.insert(circle);
        if let Some(points) = point_map.cells.get_mut(1) {
            assert_eq!(points.len(), 0);
            assert_eq!(result, Ok(10));
        }
    }

    #[test]
    fn insert_point_fail() {
        let bounds = Rectangle {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
            color: Rectangle::default().color,
        };
        let mut point_map: PointMap<Circle> = PointMap::new::<Circle>(&bounds, 10);
        let circle = Circle::new(
            Point {
                x: 1000.0,
                y: 100.0,
            },
            10.0,
        );
        let result = point_map.insert(circle);
        assert_eq!(result, Err(circle));
    }

    #[test]
    fn get_neighbors() {
        let bounds = Rectangle {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
            color: Rectangle::default().color,
        };
        let mut point_map: PointMap<Circle> = PointMap::new::<Circle>(&bounds, 10);
        let circle = Circle::new(Point { x: 11.0, y: 11.0 }, 5.0);
        let non_neighbor = Circle::new(Point { x: 30.3, y: 50.4 }, 10.0);

        let _ = point_map.insert(circle);
        let __ = point_map.insert(non_neighbor);

        if let Ok(neighbors) = point_map.get_neighbors(circle) {
            assert_eq!(neighbors.len(), 1);
            assert_eq!(neighbors.first().unwrap().to_owned(), circle);
        }
    }

    #[test]
    fn get_neighbors_edgecase() {
        let bounds = Rectangle {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
            color: Rectangle::default().color,
        };
        let mut point_map: PointMap<Circle> = PointMap::new::<Circle>(&bounds, 10);
        let circle = Circle::new(Point { x: 99.0, y: 11.0 }, 5.0);
        let non_neighbor = Circle::new(Point { x: 101.1, y: 50.4 }, 10.0);

        let _ = point_map.insert(circle);
        let __ = point_map.insert(non_neighbor);

        let neighbors = point_map.get_neighbors(circle).unwrap();

        assert_eq!(neighbors.len(), 1);
        assert_eq!(neighbors.first().unwrap().to_owned(), circle);
    }

    #[test]
    fn get_all_items() {
        let bounds = Rectangle {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
            color: Rectangle::default().color,
        };

        let mut point_map: PointMap<Point> = PointMap::new::<Point>(&bounds, 10);
        let point = Point { x: 0., y: 0. };
        let _ = point_map.insert(point);
        let points = point_map.get_items();

        assert_eq!(points, vec![Point { x: 0., y: 0. }]);
    }

    #[test]
    fn get_surrounding_cells() {
        let bounds = Rectangle {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
            color: Rectangle::default().color,
        };

        let map: PointMap<Point> = PointMap::new::<Point>(&bounds, 10);

        let indicies = map.get_neigboring_cells(25);
        println!("{}", map.cells.len());

        assert_eq!(indicies, vec![14, 15, 16, 24, 25, 26, 34, 35, 36]);
    }
}
