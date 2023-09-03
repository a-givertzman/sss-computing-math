mod arc;


#[cfg(test)]
mod tests {
    use std::f64::consts;
    use crate::geometric_primitives::{entity::Entity, point::Point, line::line::Ray, arc::arc::Arc};


    #[test]
    fn the_ray_crosses_arc() {
        let ray = Ray(Point::new(-50.0, 0.0), Point::new(1000.0, 0.0));
        let arc1 = Arc::new(Point::new(31.72077, -14.7916), Point::new(31.7207, 14.79163), Point::new(0.0, 0.0));
        let arc2 = Arc::new(Point::new(36.6934, -15.17271), Point::new(17.49279, -35.6458), Point::new(0.0, 0.0));
        assert_eq!(arc1.number_of_crossings_by_ray(&ray), 1);
        assert_eq!(arc2.number_of_crossings_by_ray(&ray), 2);
    }

    #[test]
    fn the_ray_does_not_crosse_arc() {
        let ray = Ray(Point::new(-50.0, 0.0), Point::new(1000.0, 0.0));
        let arc1 = Arc::new(Point::new(31.72077, -14.7916), Point::new(31.7207, 14.79163), Point::new(0.0, 0.0));
        let arc2 = Arc::new(Point::new(36.6934, -15.17271), Point::new(17.49279, -35.6458), Point::new(0.0, 0.0));
        assert_eq!(arc1.number_of_crossings_by_ray(&ray), 1);
        assert_eq!(arc2.number_of_crossings_by_ray(&ray), 2);
    }

    #[test]
    fn creating_an_arc() {
        let arc1 = Arc::new(Point::new(30.0, 0.0), Point::new(25.98076, 15.0), Point::new(0.0, 0.0));
        assert!((arc1.terminate_angle() - (30.0 * consts::PI) / 180.0).abs() <= 0.01);
        assert_eq!(arc1.start_angle(), 0.0);
        assert_eq!(arc1.arc_radius(), 30.0);


        let arc2 = Arc::new(Point::new(31.185228, 15.88966), Point::new(-32.88924, -11.9707), Point::new(0.0, 0.0));
        assert!((arc2.start_angle() - (27.0 * consts::PI) / 180.0).abs() <= 0.01);
        assert!((arc2.terminate_angle() - (200.0 * consts::PI) / 180.0).abs() <= 0.01);
        assert!((arc2.arc_radius() - 35.0).abs() <= 0.01);



        let arc3 = Arc::new(Point::new(31.72077, -14.7916), Point::new(31.7207, 14.79163), Point::new(0.0, 0.0));
        assert!((arc3.start_angle() - (335.0 * consts::PI) / 180.0).abs() <= 0.01);
        assert!((arc3.terminate_angle() - (25.0 * consts::PI) / 180.0).abs() <= 0.01);
        assert!((arc3.arc_radius() - 35.0).abs() <= 0.01);
    }
}