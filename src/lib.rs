pub mod common {
    use std::fs;
    pub fn read_input(day: u8) -> String {
        fs::read_to_string(format!("./inputs/input{:02}.txt", day))
            .expect(&format!("./inputs/input{:02}.txt", day))
    }
}
