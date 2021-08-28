use crate::components::wiring;
use crate::components::NANDGate;

pub struct ANDGate {
    pub input1: wiring::Wire,
    pub input2: wiring::Wire,
    pub output: wiring::Wire,
    nand1: NANDGate,
    nand2: NANDGate,
}

impl Default for ANDGate {
    fn default() -> Self {
        let mut nand1 = NANDGate::default();
        let mut nand2 = NANDGate::default();
        let input1 = wiring::Wire::default();
        let input2 = wiring::Wire::default();
        let output = wiring::Wire::default();
        wiring::connect(&mut nand1.input1, input1.clone());
        wiring::connect(&mut nand1.input2, input2.clone());
        wiring::connect(&mut nand2.input1, nand1.output.clone());
        wiring::connect(&mut nand2.input2, nand1.output.clone());
        wiring::connect(&mut nand2.output, output.clone());
        ANDGate {
            input1: input1,
            input2: input2,
            output: output,
            nand1: nand1,
            nand2: nand2,
        }
    }
}

impl ANDGate {
    pub fn settle(&mut self) {
        self.nand1.settle();
        self.nand2.settle();
    }
}

mod tests {
    use super::*;

    #[test]
    fn false_and_false() {
        let mut and_gate = ANDGate::default();
        wiring::set_low(&mut and_gate.input1);
        wiring::set_low(&mut and_gate.input2);
        and_gate.settle();
        let output_lock = and_gate.output.lock().unwrap();
        assert_eq!(*output_lock, false);
    }

    #[test]
    fn true_and_false() {
        let mut and_gate = ANDGate::default();
        wiring::set_high(&mut and_gate.input1);
        wiring::set_low(&mut and_gate.input2);
        and_gate.settle();
        let output_lock = and_gate.output.lock().unwrap();
        assert_eq!(*output_lock, false);
    }

    #[test]
    fn false_and_true() {
        let mut and_gate = ANDGate::default();
        wiring::set_low(&mut and_gate.input1);
        wiring::set_high(&mut and_gate.input2);
        and_gate.settle();
        let output_lock = and_gate.output.lock().unwrap();
        assert_eq!(*output_lock, false);
    }

    #[test]
    fn true_and_true() {
        let mut and_gate = ANDGate::default();
        wiring::set_high(&mut and_gate.input1);
        wiring::set_high(&mut and_gate.input2);
        and_gate.settle();
        let output_lock = and_gate.output.lock().unwrap();
        assert_eq!(*output_lock, true);
    }
}
