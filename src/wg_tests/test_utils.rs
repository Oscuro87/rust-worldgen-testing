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

    #[test]
    pub fn test_tuple_equality() {
        let point: Point2D = Point2D::create(10, 11);
        let tuple: (i32, i32) = (10, 11);
        assert_eq!(point, tuple);
    }

    #[test]
    pub fn test_points_equality() {
        let point: Point2D = Point2D::create(10, 11);
        let point_2: Point2D = Point2D::create(10, 11);
        assert_eq!(point, point_2);
    }

    #[test]
    pub fn test_calculate_point_from_index() {
        let index: i32 = 56;
        let width: i32 = 10;
        let expected = (6, 5);
        assert_eq!(Point2D::calc_point_from_index(index, width), expected);

        let index: i32 = 30;
        let width: i32 = 10;
        let expected = (0, 3);
        assert_eq!(Point2D::calc_point_from_index(index, width), expected);
    }

    #[test]
    pub fn test_calculate_index_from_point() {
        let point: Point2D = Point2D::create(6, 5);
        let width = 10;
        let expected: i32 = 56;
        assert_eq!(Point2D::calc_index_from_point(point, width), expected);

        let point: Point2D = Point2D::create(0, 3);
        let width = 10;
        let expected: i32 = 30;
        assert_eq!(Point2D::calc_index_from_point(point, width), expected);
    }
}
