#[derive(Debug, Clone, Copy)]
pub struct Point2D {
    x: i32,
    y: i32,
}

impl Point2D {
    pub fn create(x: i32, y: i32) -> Self {
        Point2D { x, y }
    }

    /**
     * Formula: V¯( (x2 - x1 )^2 + (y2 - y1)^2 )
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
}