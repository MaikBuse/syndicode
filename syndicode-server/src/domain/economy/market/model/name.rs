use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] // Added derives for potential future use (like HashMap key)
#[repr(i16)]
pub enum MarketName {
    AugmentationCybernetics,
    WetwareNeural,
    SyndicateData,
    BlackMarketBio,
    AutonomousDrone,
    InfoSecCounterIntel,
    VirtualSimSense,
    StreetPharm,
    ZeroDayExploit,
    RestrictedTech,
    Generic,
}

impl From<i16> for MarketName {
    fn from(value: i16) -> Self {
        match value {
            1 => Self::AugmentationCybernetics,
            2 => Self::WetwareNeural,
            3 => Self::SyndicateData,
            4 => Self::BlackMarketBio,
            5 => Self::AutonomousDrone,
            6 => Self::InfoSecCounterIntel,
            7 => Self::VirtualSimSense,
            8 => Self::StreetPharm,
            9 => Self::ZeroDayExploit,
            10 => Self::RestrictedTech,
            _ => Self::Generic,
        }
    }
}

impl From<MarketName> for i16 {
    fn from(value: MarketName) -> Self {
        match value {
            MarketName::AugmentationCybernetics => 1,
            MarketName::WetwareNeural => 2,
            MarketName::SyndicateData => 3,
            MarketName::BlackMarketBio => 4,
            MarketName::AutonomousDrone => 5,
            MarketName::InfoSecCounterIntel => 6,
            MarketName::VirtualSimSense => 7,
            MarketName::StreetPharm => 8,
            MarketName::ZeroDayExploit => 9,
            MarketName::RestrictedTech => 10,
            MarketName::Generic => 11,
        }
    }
}

impl Display for MarketName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MarketName::AugmentationCybernetics => {
                write!(f, "Augmentation & Cybernetics Exchange")
            }
            MarketName::WetwareNeural => write!(f, "Wetware & Neural Interface Market"),
            MarketName::SyndicateData => write!(f, "Syndicate Data Brokerage"),
            MarketName::BlackMarketBio => write!(f, "Black Market Bioware & Gene-Mods"),
            MarketName::AutonomousDrone => write!(f, "Autonomous Systems & Drone Bazaar"),
            MarketName::InfoSecCounterIntel => write!(f, "InfoSec & Counter-Intel Services"),
            MarketName::VirtualSimSense => {
                write!(f, "Virtual Constructs & SimSense Experiences")
            }
            MarketName::StreetPharm => write!(f, "Street Pharm & Neuro-Enhancers"),
            MarketName::ZeroDayExploit => write!(f, "Zero-Day Exploit & Malware Market"),
            MarketName::RestrictedTech => {
                write!(f, "Restricted Tech & Prototype Acquisition")
            }
            MarketName::Generic => write!(f, "Generic"),
        }
    }
}

impl From<String> for MarketName {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Augmentation & Cybernetics Exchange" => Self::AugmentationCybernetics,
            "Wetware & Neural Interface Market" => Self::WetwareNeural,
            "Syndicate Data Brokerage" => Self::SyndicateData,
            "Black Market Bioware & Gene-Mods" => Self::BlackMarketBio,
            "Autonomous Systems & Drone Bazaar" => Self::AutonomousDrone,
            "InfoSec & Counter-Intel Services" => Self::InfoSecCounterIntel,
            "Virtual Constructs & SimSense Experiences" => Self::VirtualSimSense,
            "Street Pharm & Neuro-Enhancers" => Self::StreetPharm,
            "Zero-Day Exploit & Malware Market" => Self::ZeroDayExploit,
            "Restricted Tech & Prototype Acquisition" => Self::RestrictedTech,
            _ => Self::Generic,
        }
    }
}
