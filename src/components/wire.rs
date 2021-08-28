pub mod wiring {
    use std::sync::{Arc, Mutex};
    pub type Wire = Arc<Mutex<bool>>;
    pub fn is_high(wire: Wire) -> bool {
        let wire_lock = wire.lock().unwrap();
        return *wire_lock;
    }
    pub fn is_low(wire: Wire) -> bool {
        let wire_lock = wire.lock().unwrap();
        return !*wire_lock;
    }
    pub fn set_high(wire: &mut Wire) {
        let mut wire_lock = wire.lock().unwrap();
        *wire_lock = true;
    }
    pub fn set_low(wire: &mut Wire) {
        let mut wire_lock = wire.lock().unwrap();
        *wire_lock = false;
    }
    pub fn new_high() -> Wire {
        Arc::new(Mutex::new(true))
    }
    pub fn new_low() -> Wire {
        Arc::new(Mutex::new(false))
    }
    pub fn connect(left: &mut Wire, right: Wire) {
        *left = right;
    }
}
