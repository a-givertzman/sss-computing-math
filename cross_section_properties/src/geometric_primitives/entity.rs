use super::line::line::Ray;

pub trait Entity {
    fn number_of_crossings_by_ray(&self, ray: &Ray) -> i32;

}