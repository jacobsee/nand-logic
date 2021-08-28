use crate::components::wiring;
use crate::components::NANDGate;

pub struct NOTGate {
    pub input: wiring::Wire,
    pub output: wiring::Wire,
    nand: NANDGate,
}

impl Default for NOTGate {
    fn default() -> Self {
        let mut nand_gate = NANDGate::default();
        let input = wiring::Wire::default();
        let output = wiring::Wire::default();
        wiring::connect(&mut nand_gate.input1, input.clone());
        wiring::connect(&mut nand_gate.input2, input.clone());
        wiring::connect(&mut nand_gate.output, output.clone());
        NOTGate {
            input: input,
            output: output,
            nand: nand_gate,
        }
    }
}

impl NOTGate {
    pub fn settle(&mut self) {
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
