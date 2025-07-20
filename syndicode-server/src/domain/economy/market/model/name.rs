use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] // Added derives for potential future use (like HashMap key)
#[repr(i16)]
pub enum MarketName {
    ProstheticAssembly,
    CyberbrainInterfaces,
    MemoryExperiences,
    OrganFabrications,
    AutonomousAISystems,
    BarrierSecurity,
    SimulatedStimulations,
    NeurochemicalAdjustments,
    ThirdPartyOperations,
    QuantumGrayTechnologies,
    Generic,
}

impl From<i16> for MarketName {
    fn from(value: i16) -> Self {
        match value {
            1 => Self::ProstheticAssembly,
            2 => Self::CyberbrainInterfaces,
            3 => Self::MemoryExperiences,
            4 => Self::OrganFabrications,
            5 => Self::AutonomousAISystems,
            6 => Self::BarrierSecurity,
            7 => Self::SimulatedStimulations,
            8 => Self::NeurochemicalAdjustments,
            9 => Self::ThirdPartyOperations,
            10 => Self::QuantumGrayTechnologies,
            _ => Self::Generic,
        }
    }
}

impl From<MarketName> for i16 {
    fn from(value: MarketName) -> Self {
        match value {
            MarketName::ProstheticAssembly => 1,
            MarketName::CyberbrainInterfaces => 2,
            MarketName::MemoryExperiences => 3,
            MarketName::OrganFabrications => 4,
            MarketName::AutonomousAISystems => 5,
            MarketName::BarrierSecurity => 6,
            MarketName::SimulatedStimulations => 7,
            MarketName::NeurochemicalAdjustments => 8,
            MarketName::ThirdPartyOperations => 9,
            MarketName::QuantumGrayTechnologies => 10,
            MarketName::Generic => 11,
        }
    }
}

impl Display for MarketName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MarketName::ProstheticAssembly => {
                write!(f, "Prosthetic Body Assembly")
            }
            MarketName::CyberbrainInterfaces => write!(f, "Cyberbrain Interfaces"),
            MarketName::MemoryExperiences => write!(f, "Memories & Experiences"),
            MarketName::OrganFabrications => write!(f, "Organ Fabrications"),
            MarketName::AutonomousAISystems => write!(f, "Autonomous AI Systems"),
            MarketName::BarrierSecurity => write!(f, "Barrier Mazed Security"),
            MarketName::SimulatedStimulations => {
                write!(f, "Simulated Stimulations")
            }
            MarketName::NeurochemicalAdjustments => write!(f, "Neurochemical Adjustments"),
            MarketName::ThirdPartyOperations => write!(f, "Third Party Operations"),
            MarketName::QuantumGrayTechnologies => {
                write!(f, "Quantum Gray Technologies")
            }
            MarketName::Generic => write!(f, "Generic"),
        }
    }
}

impl From<String> for MarketName {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Prosthetic Body Assembly" => Self::ProstheticAssembly,
            "Cyberbrain Interfaces" => Self::CyberbrainInterfaces,
            "Memories & Experiences" => Self::MemoryExperiences,
            "Organ Fabrications" => Self::OrganFabrications,
            "Autonomous AI Systems" => Self::AutonomousAISystems,
            "Barrier Mazed Security" => Self::BarrierSecurity,
            "Simulated Stimulations" => Self::SimulatedStimulations,
            "Neurochemical Adjustments" => Self::NeurochemicalAdjustments,
            "Third Party Operations" => Self::ThirdPartyOperations,
            "Quantum Gray Technologies" => Self::QuantumGrayTechnologies,
            _ => Self::Generic,
        }
    }
}
