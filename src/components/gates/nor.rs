use crate::components::{wiring, NANDGate, ORGate};

pub struct NORGate {
    pub input1: wiring::Wire,
    pub input2: wiring::Wire,
    pub output: wiring::Wire,
    or: ORGate,
    nand: NANDGate,
}

impl Default for NORGate {
    fn default() -> Self {
        NORGate {
            input1: wiring::Wire::default(),
            input2: wiring::Wire::default(),
            output: wiring::Wire::default(),
            or: ORGate::default(),
            nand: NANDGate::default(),
        }
    }
}

impl NORGate {
    pub fn settle(&mut self) {
        wiring::connect(&mut self.or.input1, self.input1.clone());
        wiring::connect(&mut self.or.input2, self.input2.clone());
        wiring::connect(&mut self.nand.input1, self.or.output.clone());
        wiring::connect(&mut self.nand.input2, self.or.output.clone());
        wiring::connect(&mut self.nand.output, self.output.clone());
        self.or.settle();
        self.nand.settle();
    }
}

mod tests {
    use super::*;

    #[test]
    fn false_and_false() {
        let mut nor_gate = NORGate::default();
        wiring::set_low(&mut nor_gate.input1);
        wiring::set_low(&mut nor_gate.input2);
        nor_gate.settle();
        let output_lock = nor_gate.output.lock().unwrap();
        assert_eq!(*output_lock, true);
    }

    #[test]
    fn true_and_false() {
        let mut nor_gate = NORGate::default();
        wiring::set_high(&mut nor_gate.input1);
        wiring::set_low(&mut nor_gate.input2);
        nor_gate.settle();
        let output_lock = nor_gate.output.lock().unwrap();
        assert_eq!(*output_lock, false);
    }

    #[test]
    fn false_and_true() {
        let mut nor_gate = NORGate::default();
        wiring::set_low(&mut nor_gate.input1);
        wiring::set_high(&mut nor_gate.input2);
        nor_gate.settle();
        let output_lock = nor_gate.output.lock().unwrap();
        assert_eq!(*output_lock, false);
    }

    #[test]
    fn true_and_true() {
        let mut nor_gate = NORGate::default();
        wiring::set_high(&mut nor_gate.input1);
        wiring::set_high(&mut nor_gate.input2);
        nor_gate.settle();
        let output_lock = nor_gate.output.lock().unwrap();
        assert_eq!(*output_lock, false);
    }
}
