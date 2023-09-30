#[cfg(test)]
mod tests {
    use std::{sync::Once, env};
    use log::debug;
    use crate::strength::{ship::loads::lightweight_tonnage::lightweight_tonnage::LightweightTonnage, output::type_output::TypeOutput};


    static INIT: Once = Once::new();

    fn call_once() {
        INIT.call_once(|| {
                env::set_var("RUST_LOG", "off");  // off / error / warn / info / debug / trace
                // env::set_var("RUST_BACKTRACE", "1");
                env::set_var("RUST_BACKTRACE", "full");
                env_logger::init();
            }
        )
    }


    #[test]
    fn create_lightweight_from_json_file_successfully() {
        call_once();
        let lightweight = LightweightTonnage::from_json_file("./src/tests/unit/strength/lightweight_tonnage/data/correct_data.json".to_string());
        assert!(lightweight.is_ok());
    }

    #[test]
    fn create_lightweight_from_json_file_invalid_type() {
        call_once();
        let lightweight = LightweightTonnage::from_json_file("./src/tests/unit/strength/lightweight_tonnage/data/invalid_type.json".to_string());
        assert!(lightweight.is_err());
        assert!(lightweight.unwrap_err().contains("invalid type"));
    }

    #[test]
    fn create_lightweight_from_json_file_missing_field() {
        call_once();
        let lightweight = LightweightTonnage::from_json_file("./src/tests/unit/strength/lightweight_tonnage/data/empty_field.json".to_string());
        assert!(lightweight.is_err());
        assert!(lightweight.unwrap_err().contains("missing field `lightweight"));
    }
    #[test]
    fn test_number_spatiums() {
        call_once();
        let lightweight = LightweightTonnage::from_json_file("./src/tests/unit/strength/lightweight_tonnage/data/correct_data.json".to_string()).unwrap();
        let output = lightweight.lightweight_tonnage_intensity();
        let ship = lightweight.ship;
        assert_eq!(output.spatiums.len(), ship.number_spatiums() as usize);
        assert_eq!(output.type_output, TypeOutput::LightweightTonnageIntensity);
    }

    #[test]
    fn test_type_output() {
        call_once();
        let lightweight = LightweightTonnage::from_json_file("./src/tests/unit/strength/lightweight_tonnage/data/correct_data.json".to_string()).unwrap();
        let output = lightweight.lightweight_tonnage_intensity();
        assert_eq!(output.type_output, TypeOutput::LightweightTonnageIntensity);
    }

    #[test]
    fn test_lightweight_intensity() {
        call_once();
        let lightweight = LightweightTonnage::from_json_file("./src/tests/unit/strength/lightweight_tonnage/data/correct_data.json".to_string()).unwrap();
        let output = lightweight.lightweight_tonnage_intensity();
        let mut weight = 0.0;
        for spatium in output.spatiums {
            weight += spatium.square();
        }
        let err = {
            if weight > lightweight.lightweight_tonnage {
                ((weight - lightweight.lightweight_tonnage) / lightweight.lightweight_tonnage) * 100.0

            } else if lightweight.lightweight_tonnage > weight {
                ((lightweight.lightweight_tonnage - weight) / weight) * 100.0
            } else {
                0.0
            }
        };
        debug!("\nЭталонный вес корпуса корабля(Lightweight tonnage): {} [Тонн]
Вес корпуса корабля(Lightweight tonnage), полученный в результате численного интегрирования методом трапеций: {} [Тонн]", lightweight.lightweight_tonnage, weight);
        debug!("Ошибка численного интегрирования интенсивности веса корпуса корабля по его длине = {} %", err);
        assert!(err < 5.0, "Error more than 5% = {}%.", err);
    }
}