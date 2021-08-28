mod nand;
mod not;
mod and;
mod or;
mod xor;
mod nor;

pub use self::nand::NANDGate;
pub use self::not::NOTGate;
pub use self::and::ANDGate;
pub use self::or::ORGate;
pub use self::xor::XORGate;
pub use self::nor::NORGate;

mod gates {}
