// Listed below are backends with tier two support

pub use super::backends::csat_3_plonk_aztec::Plonk as CSAT_3_PLONK_AZTEC;

use crate::Backend;

#[derive(Debug, Copy, Clone)]
pub enum TierThree {
    Csat3PlonkAztec,
}

impl TierThree {
    pub(crate) fn fetch_backend(&self) -> Box<dyn Backend> {
        match self {
            TierThree::Csat3PlonkAztec => Box::new(CSAT_3_PLONK_AZTEC),
        }
    }
}

pub const TIER_THREE_MAP: [(&str, TierThree); 1] =
    [("csat_3_plonk_aztec", TierThree::Csat3PlonkAztec)];
