//! Provides the `Ice40ChipType` struct to explicitly model the available iCE40 chips according to their specifications.
//! Also provides some constants of known iCE40 chip types taken from the family data sheet.

/// Describes the different iCE40 chip types
pub struct Ice40ChipType {
    pub series: &'static str,
    pub logic_capacity: &'static str,
    pub package_code: &'static str,
}

/// This chip is used on the `iCEstick Evaluation Kit`
pub const HX1K_TQ144: Ice40ChipType = Ice40ChipType {
    series: "hx",
    logic_capacity: "1k",
    package_code: "tq144",
};

pub const HX1K_CB132: Ice40ChipType = Ice40ChipType {
    series: "hx",
    logic_capacity: "1k",
    package_code: "cb132",
};

pub const HX1K_VQ100: Ice40ChipType = Ice40ChipType {
    series: "hx",
    logic_capacity: "1k",
    package_code: "vq100",
};

pub const HX4K_TQ144: Ice40ChipType = Ice40ChipType {
    series: "hx",
    logic_capacity: "4k",
    package_code: "TQ144",
};

pub const HX8K_CT256: Ice40ChipType = Ice40ChipType {
    series: "hx",
    logic_capacity: "8k",
    package_code: "ct256",
};
