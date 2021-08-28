use crate::components::wiring;
use crate::components::NANDGate;

pub struct ORGate {
    pub input1: wiring::Wire,
    pub input2: wiring::Wire,
    pub output: wiring::Wire,
    nand1_layer1: NANDGate,
    nand2_layer1: NANDGate,
    nand1_layer2: NANDGate,
}

impl Default for ORGate {
    fn default() -> Self {
        ORGate {
            input1: wiring::Wire::default(),
            input2: wiring::Wire::default(),
            output: wiring::Wire::default(),
            nand1_layer1: NANDGate::default(),
            nand2_layer1: NANDGate::default(),
            nand1_layer2: NANDGate::default(),
        }
    }
}

impl ORGate {
    pub fn settle(&mut self) {
        wiring::connect(&mut self.nand1_layer1.input1, self.input1.clone());
        wiring::connect(&mut self.nand1_layer1.input2, self.input1.clone());
        wiring::connect(&mut self.nand2_layer1.input1, self.input2.clone());
        wiring::connect(&mut self.nand2_layer1.input2, self.input2.clone());
        wiring::connect(&mut self.nand1_layer2.input1, self.nand1_layer1.output.clone());
        wiring::connect(&mut self.nand1_layer2.input2, self.nand2_layer1.output.clone());
        wiring::connect(&mut self.nand1_layer2.output, self.output.clone());
        self.nand1_layer1.settle();
        self.nand2_layer1.settle();
        self.nand1_layer2.settle();
    }
}

mod tests {
    use super::*;

    #[test]
    fn false_and_false() {
        let mut or_gate = ORGate::default();
        wiring::set_low(&mut or_gate.input1);
        wiring::set_low(&mut or_gate.input2);
        or_gate.settle();
        let output_lock = or_gate.output.lock().unwrap();
        assert_eq!(*output_lock, false);
    }

    #[test]
    fn true_and_false() {
        let mut or_gate = ORGate::default();
        wiring::set_high(&mut or_gate.input1);
        wiring::set_low(&mut or_gate.input2);
        or_gate.settle();
        let output_lock = or_gate.output.lock().unwrap();
        assert_eq!(*output_lock, true);
    }

    #[test]
    fn false_and_true() {
        let mut or_gate = ORGate::default();
        wiring::set_low(&mut or_gate.input1);
        wiring::set_high(&mut or_gate.input2);
        or_gate.settle();
        let output_lock = or_gate.output.lock().unwrap();
        assert_eq!(*output_lock, true);
    }

    #[test]
    fn true_and_true() {
        let mut or_gate = ORGate::default();
        wiring::set_high(&mut or_gate.input1);
        wiring::set_high(&mut or_gate.input2);
        or_gate.settle();
        let output_lock = or_gate.output.lock().unwrap();
        assert_eq!(*output_lock, true);
    }
}
