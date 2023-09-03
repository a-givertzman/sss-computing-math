mod entity;
mod point;
mod line;
mod arc;
mod utilities;


#[cfg(test)]
mod tests {
    use std::f64::consts;

    use super::{point::Point, line::{Line, Ray}, entity::Entity, arc::Arc};

    #[test]
    fn test_lines_crosse_the_ray() {
        let ray = Ray(Point::new(0.0, 7.0), Point::new(1000.0, 7.0));
        let lines = vec![
            Line::new(Point::new(5.0, 5.0), Point::new(5.0, 10.0)),
            Line::new(Point::new(5.0, 5.0), Point::new(10.0, 10.0)),
            Line::new(Point::new(5.0, 10.0), Point::new(10.0, 3.0)),
        ];
        for line in lines {
            assert_eq!(line.number_of_crossings_by_ray(&ray), 1);
        }
    }

    #[test]
    fn test_lines_do_not_crosse_the_ray() {
        let ray = Ray(Point::new(5.0, 7.0), Point::new(1000.0, 7.0));
        let lines = vec![
            Line::new(Point::new(3.0, 5.0), Point::new(3.0, 10.0)),
            Line::new(Point::new(6.0, 3.0), Point::new(10.0, 5.0)),
            Line::new(Point::new(6.0, 10.0), Point::new(10.0, 8.0)),
            Line::new(Point::new(3.0, 3.0), Point::new(10.0, 5.0)),
            Line::new(Point::new(1.0, 3.0), Point::new(3.0, 8.0)),
            Line::new(Point::new(3.0, 3.0), Point::new(4.0, 15.0)),
        ];
        for line in lines {
            assert_eq!(line.number_of_crossings_by_ray(&ray), 0);
        }

    }

    #[test]
    fn matching_lines_do_not_cross() {
        let ray = Ray(Point::new(0.0, 7.0), Point::new(1000.0, 7.0));
        let line = Line::new(Point::new(0.0, 7.0), Point::new(1000.0, 7.0));
        assert_eq!(line.number_of_crossings_by_ray(&ray), 0);

    }

    #[test]
    fn the_ray_crosses_arc() {
        let ray = Ray(Point::new(0.0, 7.0), Point::new(1000.0, 7.0));
        let arc1 =Arc::new(Point::new(2.0, 6.3), Point::new(2.0, 3.6), Point::new(1.2, 2.3));
        let arc2 = Arc::new(Point::new(2.0, 6.3), Point::new(2.0, 3.6), Point::new(1.2, 2.3));
    }

    #[test]
    fn creating_an_arc() {
        let arc1 = Arc::new(Point::new(30.0, 0.0), Point::new(25.98076, 15.0), Point::new(0.0, 0.0));
        assert!((arc1.terminate_angle() - (30.0 * consts::PI) / 180.0).abs() <= 0.01);
        assert_eq!(arc1.start_angle(), 0.0);
        assert_eq!(arc1.arc_radius(), 30.0);
        assert!((arc1.arc_angle() - (30.0 * consts::PI) / 180.0).abs() <= 0.01);

        let arc2 = Arc::new(Point::new(31.185228, 15.88966), Point::new(-32.88924, -11.9707), Point::new(0.0, 0.0));
        assert!((arc2.start_angle() - (27.0 * consts::PI) / 180.0).abs() <= 0.01);
        assert!((arc2.terminate_angle() - (200.0 * consts::PI) / 180.0).abs() <= 0.01);
        assert!((arc2.arc_radius() - 35.0).abs() <= 0.01);
        assert!((arc2.arc_angle() - (173.0 * consts::PI) / 180.0).abs() <= 0.01);


        let arc3 = Arc::new(Point::new(31.72077, -14.7916), Point::new(31.7207, 14.79163), Point::new(0.0, 0.0));
        assert!((arc3.start_angle() - (335.0 * consts::PI) / 180.0).abs() <= 0.01);
        assert!((arc3.terminate_angle() - (25.0 * consts::PI) / 180.0).abs() <= 0.01);
        assert!((arc3.arc_radius() - 35.0).abs() <= 0.01);
        assert!((arc3.arc_angle() - (50.0 * consts::PI) / 180.0).abs() <= 0.01);
    

    }
}