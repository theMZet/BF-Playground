use std::io::{Write, Read};

#[allow(dead_code)]
pub struct BFPlayground {
    array: [u8; 30000],
    pointer: usize,
}

#[allow(dead_code)]
impl BFPlayground {
    pub fn new() -> Self {
        BFPlayground {
            array: [0; 30000],
            pointer: 0,
        }
    }

    pub fn get_pointer_position(&self) -> usize {
        self.pointer
    }

    pub fn set_pointer_position(&mut self, position: usize) {
        self.pointer = position;
    }

    pub fn increment_pointer_position(&mut self) {
        if self.pointer == 29999 {
            self.pointer = 0;
        } else {
            self.pointer += 1;
        }
    }

    pub fn decrement_pointer_position(&mut self) {
        if self.pointer == 0 {
            self.pointer = 29999;
        } else {
            self.pointer -= 1;
        }
    }

    pub fn set_value_at(&mut self, position: usize, value: u8) {
        if position > 29999 {
            panic!("Index out of bounds");
        }

        self.array[position] = value;
    }

    pub fn get_value_at(&mut self, position: usize) -> u8 {
        if position > 29999 {
            panic!("Index out of bounds");
        }

        self.array[position]
    }

    pub fn get_pointer_value(&self) -> u8 {
        self.array[self.pointer]
    }

    pub fn set_pointer_value(&mut self, value: u8) {
        self.array[self.pointer] = value;
    }

    pub fn increment_pointer_value(&mut self) {
        if self.array[self.pointer] == 255 {
            self.array[self.pointer] = 0;
        } else {
            self.array[self.pointer] += 1;
        }
    }

    pub fn decrement_pointer_value(&mut self) {
        if self.array[self.pointer] == 0 {
            self.array[self.pointer] = 255;
        } else {
            self.array[self.pointer] -= 1;
        }
    }

    pub fn print_pointer_value(&self, output: &mut impl Write) -> Result<(), std::io::Error> {
        
        output.write_all((self.array[self.pointer] as char).to_string().as_bytes())?;
        Ok(())
    }

    pub fn read_pointer_value(&mut self, input: &mut impl Read) -> Result<(), std::io::Error> {
        let mut buffer = [0; 1];
        input.read_exact(&mut buffer)?;
        self.set_pointer_value(buffer[0]);
        Ok(())
    }

    pub fn execute(&mut self, code: &str, input: &mut impl Read, output: &mut impl Write) {
        let mut code_pointer: usize = 0;

        while let Some(character) = code.chars().nth(code_pointer) {
            match character {
                '>' => self.increment_pointer_position(),
                '<' => self.decrement_pointer_position(),
                '+' => self.increment_pointer_value(),
                '-' => self.decrement_pointer_value(),
                '.' => self.print_pointer_value(output).unwrap_or(()),
                ',' => self.read_pointer_value(input).unwrap_or(()),
                '[' => {
                    if self.get_pointer_value() == 0 {
                        let mut bracket_counter = 1;
                        while bracket_counter != 0 {
                            code_pointer += 1;
                            if code.chars().nth(code_pointer).unwrap() == '[' {
                                bracket_counter += 1;
                            } else if code.chars().nth(code_pointer).unwrap() == ']' {
                                bracket_counter -= 1;
                            }
                        }
                    }
                },
                ']' => {
                    if self.get_pointer_value() != 0 {
                        let mut bracket_counter = 1;
                        while bracket_counter != 0 {
                            code_pointer -= 1;
                            if code.chars().nth(code_pointer).unwrap() == ']' {
                                bracket_counter += 1;
                            } else if code.chars().nth(code_pointer).unwrap() == '[' {
                                bracket_counter -= 1;
                            }
                        }
                    }
                }
                _ => {}
            };
            code_pointer += 1;
        }
    }
}

#[cfg(test)]
#[warn(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn should_create_array_full_of_zeros() {
        let bf_playground = BFPlayground::new();
        assert_eq!(bf_playground.array, [0; 30000]);
    }

    #[test]
    fn should_create_pointer_set_to_zero() {
        let bf_playground = BFPlayground::new();
        assert_eq!(bf_playground.pointer, 0);
    }

    #[test]
    fn get_pointer_position() {
        let bf_playground = BFPlayground{
            array: [0; 30000],
            pointer: 23123,
        };
        assert_eq!(bf_playground.get_pointer_position(), 23123);
    }

    #[test]
    fn set_pointer_position() {
        let mut bf_playground = BFPlayground::new();
        assert_eq!(bf_playground.get_pointer_position(), 0);
        bf_playground.set_pointer_position(123);
        assert_eq!(bf_playground.get_pointer_position(), 123);
    }

    #[test]
    fn increment_pointer_position() {
        let mut bf_playground = BFPlayground::new();
        assert_eq!(bf_playground.get_pointer_position(), 0);
        bf_playground.increment_pointer_position();
        assert_eq!(bf_playground.get_pointer_position(), 1);
    }

    #[test]
    fn increment_overflow_pointer_position() {
        let mut bf_playground = BFPlayground::new();
        bf_playground.set_pointer_position(29999);
        assert_eq!(bf_playground.get_pointer_position(), 29999);
        bf_playground.increment_pointer_position();
        assert_eq!(bf_playground.get_pointer_position(), 0);
    }

    #[test]
    fn decrement_pointer_position() {
        let mut bf_playground = BFPlayground::new();
        bf_playground.set_pointer_position(1);
        assert_eq!(bf_playground.get_pointer_position(), 1);
        bf_playground.decrement_pointer_position();
        assert_eq!(bf_playground.get_pointer_position(), 0);
    }
    
    #[test]
    fn decrement_underflow_pointer_position() {
        let mut bf_playground = BFPlayground::new();
        assert_eq!(bf_playground.get_pointer_position(), 0);
        bf_playground.decrement_pointer_position();
        assert_eq!(bf_playground.get_pointer_position(), 29999);
    }

    #[test]
    fn set_value_at() {
        let mut bf_playground = BFPlayground::new();
        assert_eq!(bf_playground.get_value_at(0), 0);
        bf_playground.set_value_at(0, 12);
        assert_eq!(bf_playground.get_value_at(0), 12);
    }

    #[test]
    #[should_panic]
    fn set_value_at_out_of_bounds_exacly() {
        let mut bf_playground = BFPlayground::new();
        bf_playground.get_value_at(30000);
    }

    #[test]
    #[should_panic]
    fn set_value_at_out_of_bounds_more() {
        let mut bf_playground = BFPlayground::new();
        bf_playground.get_value_at(30001);
    }

    #[test]
    fn get_value_at() {
        let mut bf_playground = BFPlayground::new();
        bf_playground.set_value_at(43, 12);
        assert_eq!(bf_playground.get_value_at(43), 12);
    }

    #[test]
    #[should_panic]
    fn get_value_at_out_of_bounds_exacly() {
        let mut bf_playground = BFPlayground::new();
        bf_playground.get_value_at(30000);
    }

    #[test]
    #[should_panic]
    fn get_value_at_out_of_bounds_more() {
        let mut bf_playground = BFPlayground::new();
        bf_playground.get_value_at(30001);
    }

    #[test]
    fn get_pointer_value() {
        let bf_playground = BFPlayground{
            array: [12; 30000],
            pointer: 0,
        };
        assert_eq!(bf_playground.get_pointer_value(), 12);
    }

    #[test]
    fn set_pointer_value() {
        let mut bf_playground = BFPlayground::new();
        assert_eq!(bf_playground.get_pointer_value(), 0);
        bf_playground.set_pointer_value(12);
        assert_eq!(bf_playground.get_pointer_value(), 12);
    }

    #[test]
    fn increment_pointer_value() {
        let mut bf_playground = BFPlayground::new();
        assert_eq!(bf_playground.get_pointer_value(), 0);
        bf_playground.increment_pointer_value();
        assert_eq!(bf_playground.get_pointer_value(), 1);
    }

    #[test]
    fn increment_overflow_pointer_value() {
        let mut bf_playground = BFPlayground::new();
        bf_playground.set_pointer_value(255);
        assert_eq!(bf_playground.get_pointer_value(), 255);
        bf_playground.increment_pointer_value();
        assert_eq!(bf_playground.get_pointer_value(), 0);
    }

    #[test]
    fn decrement_pointer_value() {
        let mut bf_playground = BFPlayground::new();
        bf_playground.set_pointer_value(1);
        assert_eq!(bf_playground.get_pointer_value(), 1);
        bf_playground.decrement_pointer_value();
        assert_eq!(bf_playground.get_pointer_value(), 0);
    }

    #[test]
    fn decrement_underflow_pointer_value() {
        let mut bf_playground = BFPlayground::new();
        assert_eq!(bf_playground.get_pointer_value(), 0);
        bf_playground.decrement_pointer_value();
        assert_eq!(bf_playground.get_pointer_value(), 255);
    }

    #[test]
    fn print_pointer_value() {
        let mut output = Vec::new();
        let bf_playground = BFPlayground{
            array: ['a' as u8; 30000],
            pointer: 0,
        };
        bf_playground.print_pointer_value(&mut output).unwrap();
        assert_eq!(String::from_utf8(output).unwrap(), "a");
    }
    
    #[test]
    fn read_pointer_value() {
        let mut input = "a".as_bytes();
        let mut bf_playground = BFPlayground::new();
        assert_eq!(bf_playground.get_pointer_value(), 0);
        bf_playground.read_pointer_value(&mut input).unwrap();
        assert_eq!(bf_playground.get_pointer_value(), 'a' as u8);
    }

    #[test]
    fn execute_increment() {
        let mut bf_playground = BFPlayground::new();
        let code = "+";
        bf_playground.execute(code, &mut "".as_bytes(), &mut Vec::new());
        assert_eq!(bf_playground.get_pointer_value(), 1);
    }

    #[test]
    fn execute_decrement() {
        let mut bf_playground = BFPlayground::new();
        let code = "-";
        bf_playground.execute(code, &mut "".as_bytes(), &mut Vec::new());
        assert_eq!(bf_playground.get_pointer_value(), 255);
    }

    #[test]
    fn execute_move_left() {
        let mut bf_playground = BFPlayground::new();
        let code = "<";
        bf_playground.execute(code, &mut "".as_bytes(), &mut Vec::new());
        assert_eq!(bf_playground.get_pointer_position(), 29999);
    }

    #[test]
    fn execute_move_right() {
        let mut bf_playground = BFPlayground::new();
        let code = ">";
        bf_playground.execute(code, &mut "".as_bytes(), &mut Vec::new());
        assert_eq!(bf_playground.get_pointer_position(), 1);
    }

    #[test]
    fn execute_print() {
        let mut output = Vec::new();
        let mut bf_playground = BFPlayground::new();
        let code = ".";
        bf_playground.set_pointer_value('a' as u8);
        bf_playground.execute(code, &mut "".as_bytes(), &mut output);
        assert_eq!(String::from_utf8(output).unwrap(), "a");
    }

    #[test]
    fn execute_read() {
        let mut input = "a".as_bytes();
        let mut bf_playground = BFPlayground::new();
        let code = ",";
        bf_playground.execute(code, &mut input, &mut Vec::new());
        assert_eq!(bf_playground.get_pointer_value(), 'a' as u8);
    }

    #[test]
    fn execute_loop() {
        let mut bf_playground = BFPlayground::new();
        let code = "[-]";
        bf_playground.set_pointer_value(255);
        bf_playground.execute(code, &mut "".as_bytes(), &mut Vec::new());
        assert_eq!(bf_playground.get_pointer_value(), 0);
    }

    #[test]
    fn execute_nested_loop() {
        let mut bf_playground = BFPlayground::new();
        let code = "[[-]]";
        bf_playground.set_pointer_value(255);
        bf_playground.execute(code, &mut "".as_bytes(), &mut Vec::new());
        assert_eq!(bf_playground.get_pointer_value(), 0);
    }

}