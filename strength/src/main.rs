use equivalent_beam::input::Input;

mod equivalent_beam;


fn main() {
    let input = Input::new("./equ_beam/equ_beam.csv".to_string());
    input.run();

}
