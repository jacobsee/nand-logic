
use crate::components::wire::wiring;

pub struct NANDGate {
    pub input1: wiring::Wire,
    pub input2: wiring::Wire,
    pub output: wiring::Wire,
}

impl Default for NANDGate {
    fn default() -> Self {
        NANDGate {
            input1: wiring::new_low(),
            input2: wiring::new_low(),
            output: wiring::new_low(),
        }
    }
}

impl NANDGate {
    pub fn settle(&mut self) {
        // This should be the only "hardcoded" logic - everything else is based on combinations of this
        if !(wiring::is_high(self.input1.clone()) && wiring::is_high(self.input2.clone())) {
            wiring::set_high(&mut self.output.clone())
        } else {
            wiring::set_low(&mut self.output.clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn false_and_false() {
        let mut nand_gate = NANDGate::default();
        wiring::set_low(&mut nand_gate.input1);
        wiring::set_low(&mut nand_gate.input2);
        nand_gate.settle();
        let output_lock = nand_gate.output.lock().unwrap();
        assert_eq!(*output_lock, true);
    }

    #[test]
    fn true_and_false() {
        let mut nand_gate = NANDGate::default();
        wiring::set_high(&mut nand_gate.input1);
        wiring::set_low(&mut nand_gate.input2);
        nand_gate.settle();
        let output_lock = nand_gate.output.lock().unwrap();
        assert_eq!(*output_lock, true);
    }

    #[test]
    fn false_and_true() {
        let mut nand_gate = NANDGate::default();
        wiring::set_low(&mut nand_gate.input1);
        wiring::set_high(&mut nand_gate.input2);
        nand_gate.settle();
        let output_lock = nand_gate.output.lock().unwrap();
        assert_eq!(*output_lock, true);
    }

    #[test]
    fn true_and_true() {
        let mut nand_gate = NANDGate::default();
        wiring::set_high(&mut nand_gate.input1);
        wiring::set_high(&mut nand_gate.input2);
        nand_gate.settle();
        let output_lock = nand_gate.output.lock().unwrap();
        assert_eq!(*output_lock, false);
    }
}
