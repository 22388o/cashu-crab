pub mod nut00;
pub mod nut01;
pub mod nut02;
pub mod nut03;
pub mod nut04;
pub mod nut05;
pub mod nut06;
#[cfg(feature = "nut07")]
pub mod nut07;
#[cfg(feature = "nut08")]
pub mod nut08;
#[cfg(feature = "nut09")]
pub mod nut09;
#[cfg(feature = "nut12")]
pub mod nut12;

pub use nut00::BlindedMessage;
#[cfg(not(feature = "nut12"))]
pub use nut00::{BlindedSignature, Proof};
#[cfg(feature = "nut12")]
pub use nut12::{BlindedSignature, DleqProof, Proof};

/// List of proofs
pub type Proofs = Vec<Proof>;
