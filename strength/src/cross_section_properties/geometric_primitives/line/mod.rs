pub mod line;


#[cfg(test)]
mod tests {
    use crate::geometric_primitives::{entity::Entity, point::Point2D};
    use super::line::{Ray, Line};


    #[test]
    fn test_lines_crosse_the_ray() {
        let ray = Ray(Point2D::new(0.0, 7.0), Point2D::new(1000.0, 7.0));
        let lines = vec![
            Line::new(Point2D::new(5.0, 5.0), Point2D::new(5.0, 10.0)),
            Line::new(Point2D::new(5.0, 5.0), Point2D::new(10.0, 10.0)),
            Line::new(Point2D::new(5.0, 10.0), Point2D::new(10.0, 3.0)),
        ];
        for line in lines {
            assert_eq!(line.number_of_crossings_by_ray(&ray), 1);
        }
    }

    #[test]
    fn test_lines_do_not_crosse_the_ray() {
        let ray = Ray(Point2D::new(5.0, 7.0), Point2D::new(1000.0, 7.0));
        let lines = vec![
            Line::new(Point2D::new(3.0, 5.0), Point2D::new(3.0, 10.0)),
            Line::new(Point2D::new(6.0, 3.0), Point2D::new(10.0, 5.0)),
            Line::new(Point2D::new(6.0, 10.0), Point2D::new(10.0, 8.0)),
            Line::new(Point2D::new(3.0, 3.0), Point2D::new(10.0, 5.0)),
            Line::new(Point2D::new(1.0, 3.0), Point2D::new(3.0, 8.0)),
            Line::new(Point2D::new(3.0, 3.0), Point2D::new(4.0, 15.0)),
        ];
        for line in lines {
            assert_eq!(line.number_of_crossings_by_ray(&ray), 0);
        }

    }

    #[test]
    fn matching_lines_do_not_cross() {
        let ray = Ray(Point2D::new(0.0, 7.0), Point2D::new(1000.0, 7.0));
        let line = Line::new(Point2D::new(0.0, 7.0), Point2D::new(1000.0, 7.0));
        assert_eq!(line.number_of_crossings_by_ray(&ray), 0);

    }
}