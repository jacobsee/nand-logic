use crate::components::wiring;
use crate::components::NANDGate;

pub struct NOTGate {
    pub input: wiring::Wire,
    pub output: wiring::Wire,
    nand: NANDGate,
}

impl Default for NOTGate {
    fn default() -> Self {
        NOTGate {
            input: wiring::Wire::default(),
            output: wiring::Wire::default(),
            nand: NANDGate::default(),
        }
    }
}

impl NOTGate {
    pub fn settle(&mut self) {
        wiring::connect(&mut self.nand.input1, self.input.clone());
        wiring::connect(&mut self.nand.input2, self.input.clone());
        wiring::connect(&mut self.nand.output, self.output.clone());
        self.nand.settle();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_false() {
        let mut not_gate = NOTGate::default();
        wiring::set_low(&mut not_gate.input);
        not_gate.settle();
        let output_lock = not_gate.output.lock().unwrap();
        assert_eq!(*output_lock, true);
    }

    #[test]
    fn input_true() {
        let mut not_gate = NOTGate::default();
        wiring::set_high(&mut not_gate.input);
        not_gate.settle();
        let output_lock = not_gate.output.lock().unwrap();
        assert_eq!(*output_lock, false);
    }
}
