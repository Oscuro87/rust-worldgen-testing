#[derive(Debug, Clone, Copy)]
pub struct Point2D {
    pub x: i32,
    pub y: i32,
}

impl PartialEq<Point2D> for Point2D {
    fn eq(&self, other: &Point2D) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialEq<(i32, i32)> for Point2D {
    fn eq(&self, other: &(i32, i32)) -> bool {
        self.x == other.0 && self.y == other.1
    }
}

impl From<(i32, i32)> for Point2D {
    fn from(source: (i32, i32)) -> Self {
        Point2D::create(source.0, source.1)
    }
}

impl Into<(i32, i32)> for Point2D {
    fn into(self) -> (i32, i32) {
        (self.x, self.y)
    }
}

impl Point2D {
    pub fn create(x: i32, y: i32) -> Self {
        Point2D { x, y }
    }

    /**
     * Formula: SQRT((x2 - x1)^2 + (y2 - y1)^2)
     */
    pub fn distance_to(&self, other: Point2D) -> f32 {
        let source: (f32, f32) = (self.x as f32, self.y as f32);
        let target: (f32, f32) = (other.x as f32, other.y as f32);
        let left_part: f32 = (target.0 - source.0).powf(2.0);
        let right_part: f32 = (target.1 - source.1).powf(2.0);
        (left_part + right_part).sqrt()
    }

    pub fn is_neighbour(&self, other: Point2D, accept_diagonals: bool) -> bool {
        self.x - 1 == other.x
            || self.x + 1 == other.x
            || self.y - 1 == other.y
            || self.y + 1 == other.y
            || (accept_diagonals && self.x - 1 == other.x && self.y - 1 == other.y)
            || (accept_diagonals && self.x + 1 == other.x && self.y - 1 == other.y)
            || (accept_diagonals && self.x - 1 == other.x && self.y + 1 == other.y)
            || (accept_diagonals && self.x + 1 == other.x && self.y + 1 == other.y)
    }

    pub fn distance_between(source: Point2D, target: Point2D) -> f32 {
        source.distance_to(target)
    }

    pub fn calc_point_from_index(index: i32, map_width: i32) -> Point2D {
        let x = index % map_width;
        let y: i32 = ((index / map_width) as f32).floor() as i32;
        Point2D::create(x, y)
    }

    pub fn calc_index_from_point(point: Point2D, map_width: i32) -> i32 {
        point.x + map_width * point.y
    }
}
