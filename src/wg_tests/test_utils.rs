#[cfg(test)]
pub mod TestUtils {
    use crate::wg_utils::point_2d::Point2D;

    #[test]
    pub fn test_neighbouring_east() {
        let source: Point2D = Point2D::create(10, 10);
        let target: Point2D = Point2D::create(11, 10);
        assert_eq!(true, source.is_neighbour(target, false));
    }

    #[test]
    pub fn test_neighbouring_west() {
        let source: Point2D = Point2D::create(10, 10);
        let target: Point2D = Point2D::create(9, 10);
        assert_eq!(true, source.is_neighbour(target, false));
    }

    #[test]
    pub fn test_neighbouring_north() {
        let source: Point2D = Point2D::create(10, 10);
        let target: Point2D = Point2D::create(10, 9);
        assert_eq!(true, source.is_neighbour(target, false));
    }

    #[test]
    pub fn test_neighbouring_south() {
        let source: Point2D = Point2D::create(10, 10);
        let target: Point2D = Point2D::create(10, 11);
        assert_eq!(true, source.is_neighbour(target, false));
    }

    #[test]
    pub fn test_neighbouring_nw() {
        let source: Point2D = Point2D::create(10, 10);
        let target: Point2D = Point2D::create(9, 9);
        assert_eq!(true, source.is_neighbour(target, true));
    }

    #[test]
    pub fn test_neighbouring_ne() {
        let source: Point2D = Point2D::create(10, 10);
        let target: Point2D = Point2D::create(9, 11);
        assert_eq!(true, source.is_neighbour(target, true));
    }

    #[test]
    pub fn test_neighbouring_sw() {
        let source: Point2D = Point2D::create(10, 10);
        let target: Point2D = Point2D::create(9, 11);
        assert_eq!(true, source.is_neighbour(target, true));
    }

    #[test]
    pub fn test_neighbouring_se() {
        let source: Point2D = Point2D::create(10, 10);
        let target: Point2D = Point2D::create(11, 11);
        assert_eq!(true, source.is_neighbour(target, true));
    }

    #[test]
    pub fn test_not_neighbour() {
        let source: Point2D = Point2D::create(10, 10);
        let target: Point2D = Point2D::create(5, 30);
        assert_eq!(false, source.is_neighbour(target, true));
    }
}