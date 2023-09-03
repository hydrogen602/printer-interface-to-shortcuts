use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[allow(clippy::upper_case_acronyms)]
pub enum Filament {
    PLA,
    PETG,
    TPU,
}

impl FromStr for Filament {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PLA" => Ok(Filament::PLA),
            "PETG" => Ok(Filament::PETG),
            "TPU" => Ok(Filament::TPU),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct HotEndTemperature(u32);

impl HotEndTemperature {
    pub fn new(temp: u32) -> Option<Self> {
        if temp > 0 && temp <= 250 {
            Some(Self(temp))
        } else {
            None
        }
    }

    pub fn within_5_degrees_of(&self, other: f64) -> bool {
        (self.0 as f64 - other).abs() <= 5.
    }
}

impl From<HotEndTemperature> for u32 {
    fn from(temp: HotEndTemperature) -> Self {
        temp.0
    }
}

impl From<HotEndTemperature> for i64 {
    fn from(temp: HotEndTemperature) -> Self {
        temp.0.into()
    }
}

impl From<HotEndTemperature> for f64 {
    fn from(temp: HotEndTemperature) -> Self {
        temp.0.into()
    }
}

impl From<Filament> for HotEndTemperature {
    fn from(filament: Filament) -> Self {
        match filament {
            Filament::PLA => Self::new(200).unwrap(),
            Filament::PETG => Self::new(230).unwrap(),
            Filament::TPU => Self::new(220).unwrap(),
        }
    }
}
