#[cfg(test)]
mod tests {
    use crate::{ship::loads::lightweight::lightweight::Lightweight, core::system_of_units::Newton};

    #[test]
    fn create_lightweight_from_json_file_successfully() {
        let lightweight = Lightweight::from_json_file("./src/tests/lightweight/data/correct_data.json".to_string()).unwrap();
        let weight = Newton::from(1750 as f64);
        assert_eq!(lightweight.lightweight.0, weight.0);
    }

    #[test]
    fn create_lightweight_from_json_file_invalid_type() {
        let lightweight = Lightweight::from_json_file("./src/tests/lightweight/data/invalid_type.json".to_string());
        assert!(lightweight.is_err());
        assert!(lightweight.unwrap_err().contains("invalid type"));
    }

    #[test]
    fn create_lightweight_from_json_file_missing_field() {
        let lightweight = Lightweight::from_json_file("./src/tests/lightweight/data/empty_field.json".to_string());
        assert!(lightweight.is_err());
        assert!(lightweight.unwrap_err().contains("missing field `lightweight"));
    }
    #[test]
    fn test_count_spatiums() {
        let lightweight = Lightweight::from_json_file("./src/tests/lightweight/data/correct_data.json".to_string()).unwrap();
        let spatiums = lightweight.lightweight_intensity();
        let ship = lightweight.ship;
        assert_eq!(spatiums.len(), ship.number_spatiums() as usize);
    }

    #[test]
    fn test_lightweight_intensity() {
        let lightweight = Lightweight::from_json_file("./src/tests/lightweight/data/correct_data.json".to_string()).unwrap();
        let spatiums = lightweight.lightweight_intensity();
        let mut weight = 0.0;
        for spatium in spatiums {
            weight += spatium.square();
            //*println!("{:?}", spatium)
        }
        let err = {
            if weight > lightweight.lightweight.0 {
                ((weight - lightweight.lightweight.0) / lightweight.lightweight.0) * 100.0

            } else if lightweight.lightweight.0 > weight {
                ((lightweight.lightweight.0 - weight) / weight) * 100.0
            } else {
                0.0
            }
        };
        println!("Ошибка = {} %", err);
        assert!(err <= 5.0, "Ошибка более 5% error = {}%.", err);
    }
}
