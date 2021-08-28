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
        let mut nand1_layer1 = NANDGate::default();
        let mut nand2_layer1 = NANDGate::default();
        let mut nand1_layer2 = NANDGate::default();
        let input1 = wiring::Wire::default();
        let input2 = wiring::Wire::default();
        let output = wiring::Wire::default();
        wiring::connect(&mut nand1_layer1.input1, input1.clone());
        wiring::connect(&mut nand1_layer1.input2, input1.clone());
        wiring::connect(&mut nand2_layer1.input1, input2.clone());
        wiring::connect(&mut nand2_layer1.input2, input2.clone());
        wiring::connect(&mut nand1_layer2.input1, nand1_layer1.output.clone());
        wiring::connect(&mut nand1_layer2.input2, nand2_layer1.output.clone());
        wiring::connect(&mut nand1_layer2.output, output.clone());
        ORGate {
            input1: input1,
            input2: input2,
            output: output,
            nand1_layer1: nand1_layer1,
            nand2_layer1: nand2_layer1,
            nand1_layer2: nand1_layer2,
        }
    }
}

impl ORGate {
    pub fn settle(&mut self) {
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
