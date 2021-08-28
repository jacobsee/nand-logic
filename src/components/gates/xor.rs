use crate::components::wiring;
use crate::components::NANDGate;

pub struct XORGate {
    pub input1: wiring::Wire,
    pub input2: wiring::Wire,
    pub output: wiring::Wire,
    nand1_layer1: NANDGate,
    nand1_layer2: NANDGate,
    nand2_layer2: NANDGate,
    nand1_layer3: NANDGate,
}

impl Default for XORGate {
    fn default() -> Self {
        XORGate {
            input1: wiring::Wire::default(),
            input2: wiring::Wire::default(),
            output: wiring::Wire::default(),
            nand1_layer1: NANDGate::default(),
            nand1_layer2: NANDGate::default(),
            nand2_layer2: NANDGate::default(),
            nand1_layer3: NANDGate::default(),
        }
    }
}

impl XORGate {
    pub fn settle(&mut self) {
        wiring::connect(&mut self.nand1_layer1.input1, self.input1.clone());
        wiring::connect(&mut self.nand1_layer1.input2, self.input2.clone());
        wiring::connect(&mut self.nand1_layer2.input1, self.input1.clone());
        wiring::connect(&mut self.nand1_layer2.input2, self.nand1_layer1.output.clone());
        wiring::connect(&mut self.nand2_layer2.input1, self.nand1_layer1.output.clone());
        wiring::connect(&mut self.nand2_layer2.input2, self.input2.clone());
        wiring::connect(&mut self.nand1_layer3.input1, self.nand1_layer2.output.clone());
        wiring::connect(&mut self.nand1_layer3.input2, self.nand2_layer2.output.clone());
        wiring::connect(&mut self.nand1_layer3.output, self.output.clone());
        self.nand1_layer1.settle();
        self.nand1_layer2.settle();
        self.nand2_layer2.settle();
        self.nand1_layer3.settle();
    }
}

mod tests {
    use super::*;

    #[test]
    fn false_and_false() {
        let mut xor_gate = XORGate::default();
        wiring::set_low(&mut xor_gate.input1);
        wiring::set_low(&mut xor_gate.input2);
        xor_gate.settle();
        let output_lock = xor_gate.output.lock().unwrap();
        assert_eq!(*output_lock, false);
    }

    #[test]
    fn true_and_false() {
        let mut xor_gate = XORGate::default();
        wiring::set_high(&mut xor_gate.input1);
        wiring::set_low(&mut xor_gate.input2);
        xor_gate.settle();
        let output_lock = xor_gate.output.lock().unwrap();
        assert_eq!(*output_lock, true);
    }

    #[test]
    fn false_and_true() {
        let mut xor_gate = XORGate::default();
        wiring::set_low(&mut xor_gate.input1);
        wiring::set_high(&mut xor_gate.input2);
        xor_gate.settle();
        let output_lock = xor_gate.output.lock().unwrap();
        assert_eq!(*output_lock, true);
    }

    #[test]
    fn true_and_true() {
        let mut xor_gate = XORGate::default();
        wiring::set_high(&mut xor_gate.input1);
        wiring::set_high(&mut xor_gate.input2);
        xor_gate.settle();
        let output_lock = xor_gate.output.lock().unwrap();
        assert_eq!(*output_lock, false);
    }
}
