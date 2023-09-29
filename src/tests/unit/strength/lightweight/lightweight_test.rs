#[cfg(test)]
mod tests {
    use log::debug;

    use crate::strength::{ship::loads::lightweight::lightweight::Lightweight, output::type_output::TypeOutput};

    #[test]
    fn create_lightweight_from_json_file_successfully() {
        let lightweight = Lightweight::from_json_file("./src/tests/unit/strength/lightweight/data/correct_data.json".to_string());
        assert!(lightweight.is_ok());
    }

    #[test]
    fn create_lightweight_from_json_file_invalid_type() {
        let lightweight = Lightweight::from_json_file("./src/tests/unit/strength/lightweight/data/invalid_type.json".to_string());
        assert!(lightweight.is_err());
        assert!(lightweight.unwrap_err().contains("invalid type"));
    }

    #[test]
    fn create_lightweight_from_json_file_missing_field() {
        let lightweight = Lightweight::from_json_file("./src/tests/unit/strength/lightweight/data/empty_field.json".to_string());
        assert!(lightweight.is_err());
        assert!(lightweight.unwrap_err().contains("missing field `lightweight"));
    }
    #[test]
    fn test_number_spatiums() {
        let lightweight = Lightweight::from_json_file("./src/tests/unit/strength/lightweight/data/correct_data.json".to_string()).unwrap();
        let output = lightweight.lightweight_intensity();
        let ship = lightweight.ship;
        assert_eq!(output.spatiums.len(), ship.number_spatiums() as usize);
        assert_eq!(output.type_output, TypeOutput::LightweightIntensity);
    }

    #[test]
    fn test_type_output() {
        let lightweight = Lightweight::from_json_file("./src/tests/unit/strength/lightweight/data/correct_data.json".to_string()).unwrap();
        let output = lightweight.lightweight_intensity();
        assert_eq!(output.type_output, TypeOutput::LightweightIntensity);
    }

    #[test]
    fn test_lightweight_intensity() {
        let lightweight = Lightweight::from_json_file("./src/tests/unit/strength/lightweight/data/correct_data.json".to_string()).unwrap();
        let output = lightweight.lightweight_intensity();
        let mut weight = 0.0;
        for spatium in output.spatiums {
            weight += spatium.square();
            //println!("{:?}", spatium)
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
        debug!("Ошибка численного интегрирования интенсивности веса корпуса корабля по его длине = {} %", err);
        assert!(err < 5.0, "Error more than 5% = {}%.", err);
    }
}