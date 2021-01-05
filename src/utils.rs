use std::fs;

pub fn read_input(id: &str) -> String {
    fs::read_to_string(format!("inputs/i{}", id))
        .expect(&format!("Did not find `i{}` under `inputs/`", id))
}
