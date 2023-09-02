mod entity;
mod point;
mod line;
mod arc;
mod utilities;


#[cfg(test)]
mod tests {
    use super::{point::Point, line::{Line, Ray}, entity::Entity};

    #[test]
    fn test_crosse_line() {
        let ray = Ray(Point::new(0.0, 7.0), Point::new(10.0, 7.0));
        let line = Line::new(Point::new(5.0, 5.0), Point::new(5.0, 10.0));
        assert_eq!(line.crossed(&ray), true);
    }
}
