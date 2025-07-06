use crate::domain::economy::market::model::name::MarketName;
use rand::distr::weighted::WeightedIndex;
use rand::distr::Distribution;
use rand::seq::IndexedRandom;
use rand::Rng;

// --- Augmentation & Cybernetics Exchange ---
const AUG_CYBER_PREFIXES: &[&str] = &[
    "Cyber", "Tek", "Mecha", "Apex", "Omni", "Ronin", "Hexa", "Bio", "Exo", "Opti", "Sector",
    "Chrome", "Pulse",
];
const AUG_CYBER_CORES: &[&str] = &[
    "Aug", "Mod", "Link", "Circuit", "Plate", "Frame", "Core", "Node", "Sys", "Ware", "Gear",
    "Rig", "Chip", "Coil", "Limb", "Hand", "Eye", "Form", "Synth",
];
const AUG_CYBER_SUFFIXES: &[&str] = &[
    "Exchange",
    "Mods",
    "Cybernetics",
    "Works",
    "Foundry",
    "Den",
    "Bazaar",
    "Systems",
    "Solutions",
    "Labs",
    "Inc",
    "Ltd",
    "Corp",
    "Dynamics",
    "Implants",
];
const AUG_CYBER_ADJECTIVES: &[&str] = &[
    "Augmented",
    "Chrome",
    "Bio-Forged",
    "Street-Level",
    "Custom",
    "Bespoke",
    "Advanced",
    "Reinforced",
    "Integrated",
    "Tactical",
];
const AUG_CYBER_WEIGHTS: &[f64] = &[0.40, 0.30, 0.05, 0.25]; // Pref+Core+Suf, Adj+Core+Suf, C+C+Suf, Pref+Core

// --- Wetware & Neural Interface Market ---
const WETWARE_PREFIXES: &[&str] = &[
    "Neuro", "Syn", "Psy", "Sim", "Bio", "Mind", "Cortex", "Echo", "Deep", "Meta", "Chrono",
    "Opti", "Pulse",
];
const WETWARE_CORES: &[&str] = &[
    "Link", "Node", "Ware", "Mind", "Sim", "Sense", "Stream", "Dream", "Nerve", "Synapse",
    "Cortex", "Sequence", "Helix", "Logic", "Matrix", "Flow", "Coil", "Chip",
];
const WETWARE_SUFFIXES: &[&str] = &[
    "Interfaces",
    "Implants",
    "Wetware",
    "Labs",
    "Solutions",
    "Network",
    "Systems",
    "Dynamics",
    "Group",
    "Collective",
    "Simulations",
    "Connect",
    "Corp",
];
const WETWARE_ADJECTIVES: &[&str] = &[
    "Neuro-Linked",
    "Synthetic",
    "Adaptive",
    "Virtual",
    "Deep-Web",
    "Lucid",
    "Sensory",
    "Advanced",
    "Integrated",
    "Cognitive",
];
const WETWARE_WEIGHTS: &[f64] = &[0.40, 0.35, 0.05, 0.20];

// --- Syndicate Data Brokerage ---
const SYNDICATE_PREFIXES: &[&str] = &[
    "Data", "Syn", "Void", "Kuro", "Umbra", "Ghost", "Cipher", "Zero-Day", "Deep", "Sector",
    "Echo", "Apex", "Omni",
];
const SYNDICATE_CORES: &[&str] = &[
    "Data",
    "Net",
    "Sec",
    "Brokerage",
    "Syndicate",
    "Cipher",
    "Vector",
    "Shade",
    "Node",
    "Link",
    "Core",
    "Logic",
    "Matrix",
    "Index",
    "Cache",
    "Whisper",
    "Stream",
];
const SYNDICATE_SUFFIXES: &[&str] = &[
    "Brokerage",
    "Syndicate",
    "Collective",
    "Network",
    "Vault",
    "Intel",
    "Clearinghouse",
    "Circle",
    "Group",
    "Solutions",
    "Systems",
    "Exchange",
    "Corp",
    "Inc",
    "Holdings",
];
const SYNDICATE_ADJECTIVES: &[&str] = &[
    "Secure",
    "Encrypted",
    "Covert",
    "Clandestine",
    "Global",
    "Shadow",
    "Deep-Web",
    "Restricted",
    "Classified",
    "Sovereign",
    "Archival",
];
const SYNDICATE_WEIGHTS: &[f64] = &[0.35, 0.35, 0.15, 0.15]; // More corporate/structured names

// --- Black Market Bioware & Gene-Mods ---
const BLACK_BIO_PREFIXES: &[&str] = &[
    "Bio", "Gene", "Patho", "Toxi", "Viro", "Myco", "Rogue", "Blight", "Kuro", "Void", "Umbra",
    "Stray", "Pharm",
];
const BLACK_BIO_CORES: &[&str] = &[
    "Strain", "Splice", "Toxin", "Flesh", "Scar", "Blight", "Graft", "Chimera", "Mod", "Aug",
    "Sequence", "Helix", "Tissue", "Serum", "Spore", "Mycel", "Vat", "Clone", "Origin", "Evo",
];
const BLACK_BIO_SUFFIXES: &[&str] = &[
    "Labs",
    "Den",
    "Foundry",
    "Bazaar",
    "Mods",
    "Imports",
    "Collective",
    "Cult",
    "Strains",
    "Bioware",
    "Genemods",
    "Market",
    "Circle",
    "Works",
];
const BLACK_BIO_ADJECTIVES: &[&str] = &[
    "Illicit",
    "Volatile",
    "Unsanctioned",
    "Black",
    "Street-Level",
    "Rogue",
    "Grafted",
    "Vat-Grown",
    "Contraband",
    "Mutated",
    "Synthetic",
    "Toxic",
];
const BLACK_BIO_WEIGHTS: &[f64] = &[0.25, 0.20, 0.05, 0.50]; // Higher chance of simple Pref+Core

// --- Autonomous Systems & Drone Bazaar ---
const DRONE_PREFIXES: &[&str] = &[
    "Auto", "Mecha", "Sentry", "Apex", "Omni", "Hexa", "Sector", "Cyber", "Tek", "Grid", "Axiom",
    "Zone", "Pulse",
];
const DRONE_CORES: &[&str] = &[
    "Drone", "Bot", "Auton", "Sentry", "Logic", "Systems", "Dynamics", "Core", "Node", "AI",
    "Frame", "Rig", "Plate", "Swarm", "Hunter", "Eye", "Wing",
];
const DRONE_SUFFIXES: &[&str] = &[
    "Robotics",
    "Systems",
    "Dynamics",
    "Bazaar",
    "Security",
    "Logistics",
    "Platform",
    "Corp",
    "Inc",
    "Solutions",
    "Group",
    "Labs",
    "Industries",
    "Market",
    "Works",
    "Sentry",
];
const DRONE_ADJECTIVES: &[&str] = &[
    "Autonomous",
    "Sentient",
    "Reactive",
    "Advanced",
    "Global",
    "Sovereign",
    "Hunter",
    "Stealth",
    "Integrated",
    "Tactical",
    "Swarm",
    "Ariel",
];
const DRONE_WEIGHTS: &[f64] = &[0.40, 0.30, 0.10, 0.20];

// --- InfoSec & Counter-Intel Services ---
const INFOSEC_PREFIXES: &[&str] = &[
    "Sec", "Cipher", "Zero-Day", "Ghost", "Umbra", "Axiom", "Vex", "Counter", "Cyber", "Data",
    "Grid", "Void", "Apex", "Stealth",
];
const INFOSEC_CORES: &[&str] = &[
    "Sec", "Logic", "Cipher", "Shield", "Wall", "Breach", "Vector", "Trace", "Glitch", "Node",
    "Sys", "Ware", "Net", "Data", "Key", "Daemon", "Spike", "Flow", "Protocol",
];
const INFOSEC_SUFFIXES: &[&str] = &[
    "Security",
    "Solutions",
    "Analytics",
    "Intel",
    "Protocol",
    "Services",
    "Group",
    "Defense",
    "Systems",
    "Labs",
    "Inc",
    "Corp",
    "Network",
    "Consulting",
    "Audit",
];
const INFOSEC_ADJECTIVES: &[&str] = &[
    "Secure",
    "Stealth",
    "Encrypted",
    "Shielded",
    "Quantum",
    "Counter",
    "Deep-Web",
    "Zero-Day",
    "Advanced",
    "Global",
    "Proactive",
    "Defensive",
];
const INFOSEC_WEIGHTS: &[f64] = &[0.35, 0.40, 0.05, 0.20]; // More Adj+Core+Suf

// --- Virtual Constructs & SimSense Experiences ---
const VIRTUAL_PREFIXES: &[&str] = &[
    "Sim", "Virtual", "Echo", "Chrono", "Neuro", "Psy", "Dream", "Meta", "Void", "Data", "Grid",
    "Mind", "Syn", "Flux", "Pulse",
];
const VIRTUAL_CORES: &[&str] = &[
    "Construct",
    "Sim",
    "Sense",
    "Stream",
    "Dream",
    "Rift",
    "Veil",
    "Matrix",
    "Cortex",
    "Node",
    "Logic",
    "Ware",
    "Link",
    "Flow",
    "Frame",
    "Space",
    "World",
    "Mirage",
];
const VIRTUAL_SUFFIXES: &[&str] = &[
    "Simulations",
    "Experiences",
    "Constructs",
    "Labs",
    "Ventures",
    "Platform",
    "Network",
    "Dreams",
    "Corp",
    "Inc",
    "Solutions",
    "Systems",
    "Group",
    "Realities",
    "Worlds",
];
const VIRTUAL_ADJECTIVES: &[&str] = &[
    "Virtual",
    "Augmented",
    "Simulated",
    "Sensory",
    "Deep",
    "Custom",
    "Bespoke",
    "Lucid",
    "Hyper",
    "Neuro",
    "Advanced",
    "Haptic",
];
const VIRTUAL_WEIGHTS: &[f64] = &[0.40, 0.35, 0.05, 0.20];

// --- Street Pharm & Neuro-Enhancers ---
const PHARM_PREFIXES: &[&str] = &[
    "Pharm", "Neuro", "Psy", "Soma", "Chem", "Street", "Blight", "Neon", "Syn", "Bio", "Toxi",
    "Flux", "Pulse",
];
const PHARM_CORES: &[&str] = &[
    "Juice", "Stim", "Chem", "Serum", "Toxin", "Bloom", "Spark", "Rush", "Mod", "Nerve", "Synapse",
    "Cortex", "Vita", "Strain", "Dose", "Fix",
];
const PHARM_SUFFIXES: &[&str] = &[
    "Pharma",
    "Enhancers",
    "Chems",
    "Collective",
    "Den",
    "Imports",
    "Solutions",
    "Labs",
    "Market",
    "Bazaar",
    "Distro",
    "Co",
    "Group",
];
const PHARM_ADJECTIVES: &[&str] = &[
    "Volatile",
    "Synthetic",
    "Illicit",
    "Street-Level",
    "Boosted",
    "Reactive",
    "Neon",
    "Neuro",
    "Combat",
    "Focus",
    "Hyper",
    "Psycho",
];
const PHARM_WEIGHTS: &[f64] = &[0.25, 0.20, 0.05, 0.50]; // Higher chance of simple Pref+Core

// --- Zero-Day Exploit & Malware Market ---
const EXPLOIT_PREFIXES: &[&str] = &[
    "Zero-Day", "Glitch", "Vex", "Rogue", "Daemon", "Void", "Exploit", "Cyber", "Data", "Syn",
    "Kuro", "Umbra", "Ghost", "Scarab", "Serpent",
];
const EXPLOIT_CORES: &[&str] = &[
    "Exploit", "Breach", "Vector", "Glitch", "Cipher", "Key", "Daemon", "Scar", "Rift", "Ware",
    "Code", "Logic", "Spike", "Root", "Worm", "Virus",
];
const EXPLOIT_SUFFIXES: &[&str] = &[
    "Market",
    "Exploits",
    "Brokerage",
    "Solutions",
    "Vectors",
    "Collective",
    "Syndicate",
    "Vault",
    "Labs",
    "Group",
    "Inc",
    "Exchange",
    "Den",
];
const EXPLOIT_ADJECTIVES: &[&str] = &[
    "Volatile",
    "Illicit",
    "Zero-Day",
    "Deep-Web",
    "Covert",
    "Quantum",
    "Restricted",
    "Black",
    "Rogue",
    "Unsanctioned",
    "Weaponized",
    "Stealth",
];
const EXPLOIT_WEIGHTS: &[f64] = &[0.30, 0.30, 0.10, 0.30];

// --- Restricted Tech & Prototype Acquisition ---
const RESTRICTED_PREFIXES: &[&str] = &[
    "Apex",
    "Proto",
    "Arch",
    "Omega",
    "Void",
    "Classified",
    "Ronin",
    "Zero-Day",
    "Xeno",
    "Stealth",
    "Axiom",
    "Omni",
    "Deep",
    "Sector",
    "Quantum",
];
const RESTRICTED_CORES: &[&str] = &[
    "Prototype",
    "Logic",
    "Core",
    "Aug",
    "Sequence",
    "Factor",
    "Venture",
    "Node",
    "Sys",
    "Ware",
    "Data",
    "Matrix",
    "Helix",
    "Mod",
    "Frame",
    "Tech",
    "Engine",
    "Drive",
];
const RESTRICTED_SUFFIXES: &[&str] = &[
    "Acquisitions",
    "Prototypes",
    "Technologies",
    "Ventures",
    "Holdings",
    "Imports",
    "Logistics",
    "Project",
    "Initiative",
    "Group",
    "Labs",
    "Solutions",
    "Corp",
    "Inc",
    "Syndicate",
    "Clearinghouse",
];
const RESTRICTED_ADJECTIVES: &[&str] = &[
    "Restricted",
    "Classified",
    "Experimental",
    "Advanced",
    "Prototype",
    "Sovereign",
    "Black",
    "Stealth",
    "Illicit",
    "High-Orbit",
    "Quantum",
    "Secure",
];
const RESTRICTED_WEIGHTS: &[f64] = &[0.40, 0.30, 0.15, 0.15]; // More corporate/structured names

// --- Generic Fallback Lists ---
const GENERIC_PREFIXES: &[&str] = &[
    "Cyber", "Neuro", "Syn", "Bio", "Apex", "Omni", "Chrono", "Zero-Day", "Echo", "Ronin", "Hexa",
    "Void", "Kuro", "Gene", "Tek", "Mecha", "Data", "Grid", "Vex", "Sim", "Exo", "Cryo", "Opti",
    "Axiom", "Flux", "Quant", "Iso", "Auto", "Psy", "Soma", "Pharm", "Viro", "Myco", "Toxi",
    "Patho", "Umbra", "Noct", "Rogue", "Ghost", "Scarab", "Serpent", "Glitch", "Blight", "Stray",
    "Wraith", "Neo", "Sector", "Zone", "Arch", "Edge", "Deep", "Meta", "Kilo", "Giga", "Terra",
    "Sol", "Luna", "Astro", "Nova", "Pulse",
];
const GENERIC_CORES: &[&str] = &[
    "Link",
    "Core",
    "Node",
    "Sys",
    "Ware",
    "Net",
    "Data",
    "Sec",
    "Logic",
    "Matrix",
    "Pulse",
    "Circuit",
    "Flow",
    "Frame",
    "Tech",
    "Code",
    "Mind",
    "Drone",
    "Bot",
    "Auton",
    "Sentry",
    "AI",
    "Construct",
    "Sim",
    "Rift",
    "Veil",
    "Sense",
    "Stream",
    "Dream",
    "Exploit",
    "Cipher",
    "Key",
    "Shield",
    "Wall",
    "Breach",
    "Vector",
    "Daemon",
    "Spike",
    "Trace",
    "Protocol",
    "Array",
    "Rig",
    "Gear",
    "Chip",
    "Coil",
    "Plate",
    "Digit",
    "Relay",
    "Signal",
    "Byte",
    "Script",
    "Index",
    "Cache",
    "Gen",
    "Helix",
    "Sequence",
    "Mod",
    "Aug",
    "Vita",
    "Form",
    "Splice",
    "Strain",
    "Bloom",
    "Tissue",
    "Flesh",
    "Soma",
    "Serum",
    "Graft",
    "Chimera",
    "Synth",
    "Vein",
    "Spore",
    "Mycel",
    "Toxin",
    "Pathogen",
    "Viral",
    "Nerve",
    "Synapse",
    "Cortex",
    "Juice",
    "Stim",
    "Chem",
    "Nano",
    "Vat",
    "Clone",
    "Origin",
    "Evo",
    "Corp",
    "Dynamics",
    "Solutions",
    "Ventures",
    "Holdings",
    "Industries",
    "Syndicate",
    "Combine",
    "Factor",
    "Collective",
    "Group",
    "Labs",
    "Systems",
    "Security",
    "Network",
    "Guild",
    "Cartel",
    "Circle",
    "Union",
    "Exchange",
    "Brokerage",
    "Clearinghouse",
    "Foundry",
    "Den",
    "Sprawl",
    "Bazaar",
    "Vault",
    "Nexus",
    "Outfit",
    "Crew",
    "Firm",
    "Alliance",
    "Consortium",
    "Edge",
    "Rift",
    "Veil",
    "Glitch",
    "Echo",
    "Shade",
    "Cipher",
    "Vector",
    "Flux",
    "Nexus",
    "Scar",
    "Blight",
    "Ash",
    "Rust",
    "Mire",
    "Spire",
    "Gate",
    "Point",
    "Line",
    "Spark",
    "Drift",
    "Shift",
    "Fade",
    "Mirage",
    "Web",
    "Tangle",
    "Thorn",
    "Shard",
    "Whisper",
    "Silence",
    "Static",
    "Signal",
];
const GENERIC_SUFFIXES: &[&str] = &[
    "Corp",
    "Ltd",
    "Inc",
    "Group",
    "Labs",
    "Dynamics",
    "Industries",
    "Security",
    "Ventures",
    "Collective",
    "Syndicate",
    "Network",
    "Systems",
    "Solutions",
    "Exchange",
    "Brokerage",
    "Bazaar",
    "Market",
    "Services",
    "Mods",
    "Cybernetics",
    "Biotics",
    "Robotics",
    "Analytics",
    "Pharma",
    "Augments",
    "Constructs",
    "Simulations",
    "Exploits",
    "Acquisitions",
    "Intel",
    "Data",
    "Logistics",
    "Fabrications",
    "Imports",
    "Exports",
    "Technologies",
    "Protocol",
    "Initiative",
    "Project",
    "Den",
    "Vault",
    "Foundry",
    "Works",
    "Guild",
    "Cartel",
    "Circle",
    "Union",
    "Clearinghouse",
    "Sprawl",
    "Platform",
    "Station",
    "Terminal",
    "Zone",
    "Outfit",
    "Crew",
    "Firm",
    "Nexus",
];
const GENERIC_ADJECTIVES: &[&str] = &[
    "Advanced",
    "Secure",
    "Global",
    "Virtual",
    "Augmented",
    "Synthetic",
    "Rogue",
    "Shadow",
    "Neon",
    "Chrome",
    "Quantum",
    "Stealth",
    "Bespoke",
    "Custom",
    "Volatile",
    "Glitched",
    "Encrypted",
    "Shielded",
    "Sovereign",
    "Autonomous",
    "Sentient",
    "Adaptive",
    "Reactive",
    "Illicit",
    "Restricted",
    "Sanctioned",
    "Unsanctioned",
    "Black",
    "Grey",
    "Covert",
    "Clandestine",
    "Fugitive",
    "Street-Level",
    "High-Orbit",
    "Deep-Web",
    "Zero-Day",
    "Experimental",
    "Classified",
    "Archival",
    "Voltaic",
    "Cryo",
    "Bio-Forged",
    "Neuro-Linked",
    "Data-Woven",
    "Code-Bound",
    "Rust-Stained",
    "Ash-Covered",
    "Grime",
    "Prime",
    "Alpha",
    "Omega",
];
const GENERIC_WEIGHTS: &[f64] = &[0.35, 0.30, 0.10, 0.25]; // Balanced weights

/// Selects a random element from a slice, providing a default if the slice is empty.
fn choose_or_default<'a>(slice: &'a [&'a str], rng: &mut impl Rng, default: &'a str) -> &'a str {
    slice.choose(rng).copied().unwrap_or(default)
}

// --- Simplified Generator Function ---
pub fn generate_business_name(market_name: MarketName) -> String {
    let mut rng = rand::rng();

    // 1. Select the appropriate lists and weights based on the market
    let (prefixes, cores, suffixes, adjectives, weights) = match market_name {
        MarketName::AugmentationCybernetics => (
            AUG_CYBER_PREFIXES,
            AUG_CYBER_CORES,
            AUG_CYBER_SUFFIXES,
            AUG_CYBER_ADJECTIVES,
            AUG_CYBER_WEIGHTS,
        ),
        MarketName::WetwareNeural => (
            WETWARE_PREFIXES,
            WETWARE_CORES,
            WETWARE_SUFFIXES,
            WETWARE_ADJECTIVES,
            WETWARE_WEIGHTS,
        ),
        MarketName::SyndicateData => (
            SYNDICATE_PREFIXES,
            SYNDICATE_CORES,
            SYNDICATE_SUFFIXES,
            SYNDICATE_ADJECTIVES,
            SYNDICATE_WEIGHTS,
        ),
        MarketName::BlackMarketBio => (
            BLACK_BIO_PREFIXES,
            BLACK_BIO_CORES,
            BLACK_BIO_SUFFIXES,
            BLACK_BIO_ADJECTIVES,
            BLACK_BIO_WEIGHTS,
        ),
        MarketName::AutonomousDrone => (
            DRONE_PREFIXES,
            DRONE_CORES,
            DRONE_SUFFIXES,
            DRONE_ADJECTIVES,
            DRONE_WEIGHTS,
        ),
        MarketName::InfoSecCounterIntel => (
            INFOSEC_PREFIXES,
            INFOSEC_CORES,
            INFOSEC_SUFFIXES,
            INFOSEC_ADJECTIVES,
            INFOSEC_WEIGHTS,
        ),
        MarketName::VirtualSimSense => (
            VIRTUAL_PREFIXES,
            VIRTUAL_CORES,
            VIRTUAL_SUFFIXES,
            VIRTUAL_ADJECTIVES,
            VIRTUAL_WEIGHTS,
        ),
        MarketName::StreetPharm => (
            PHARM_PREFIXES,
            PHARM_CORES,
            PHARM_SUFFIXES,
            PHARM_ADJECTIVES,
            PHARM_WEIGHTS,
        ),
        MarketName::ZeroDayExploit => (
            EXPLOIT_PREFIXES,
            EXPLOIT_CORES,
            EXPLOIT_SUFFIXES,
            EXPLOIT_ADJECTIVES,
            EXPLOIT_WEIGHTS,
        ),
        MarketName::RestrictedTech => (
            RESTRICTED_PREFIXES,
            RESTRICTED_CORES,
            RESTRICTED_SUFFIXES,
            RESTRICTED_ADJECTIVES,
            RESTRICTED_WEIGHTS,
        ),
        MarketName::Generic => (
            GENERIC_PREFIXES,
            GENERIC_CORES,
            GENERIC_SUFFIXES,
            GENERIC_ADJECTIVES,
            GENERIC_WEIGHTS,
        ),
    };

    // 2. Choose a name structure template based on weights
    let templates = [0, 1, 2, 3]; // 0: P+C+S, 1: A+C+S, 2: C+C+S, 3: P+C
    let dist = WeightedIndex::new(weights).expect("Invalid weights for WeightedIndex");
    let template_choice = templates[dist.sample(&mut rng)];

    // Default words in case any list is somehow empty
    let default_prefix = "Omni";
    let default_core = "Core";
    let default_suffix = "Solutions";
    let default_adjective = "Advanced";

    // 3. Generate the name based on the chosen template and lists
    match template_choice {
        0 => {
            // Template: Prefix + Core + Suffix
            let prefix = choose_or_default(prefixes, &mut rng, default_prefix);
            let core = choose_or_default(cores, &mut rng, default_core);
            let suffix = choose_or_default(suffixes, &mut rng, default_suffix);

            if core.eq_ignore_ascii_case(suffix) || prefix.eq_ignore_ascii_case(core) {
                format!(
                    "{} {}", // Simple fallback if parts repeat
                    choose_or_default(GENERIC_PREFIXES, &mut rng, "Apex"),
                    choose_or_default(GENERIC_SUFFIXES, &mut rng, "Systems")
                )
            } else {
                format!("{prefix} {core} {suffix}") // CamelCase or similar
            }
        }
        1 => {
            // Template: Adjective + Core + Suffix
            let adjective = choose_or_default(adjectives, &mut rng, default_adjective);
            let core = choose_or_default(cores, &mut rng, default_core);
            let suffix = choose_or_default(suffixes, &mut rng, default_suffix);

            if adjective.eq_ignore_ascii_case(core) || adjective.eq_ignore_ascii_case(suffix) {
                format!(
                    "{} {} {}", // Simple fallback if parts repeat
                    choose_or_default(GENERIC_ADJECTIVES, &mut rng, "Prime"),
                    core,
                    suffix
                )
            } else {
                format!("{adjective} {core} {suffix}") // Space after adjective
            }
        }
        2 => {
            // Template: Core + Core + Suffix
            let core1 = choose_or_default(cores, &mut rng, default_core);
            // Ensure core2 has options even if core1 took the only one (unlikely)
            let core2 = choose_or_default(cores, &mut rng, "Data");
            let suffix = choose_or_default(suffixes, &mut rng, default_suffix);

            if core1.eq_ignore_ascii_case(core2) {
                format!(
                    "{} {}", // Simple fallback if parts repeat
                    choose_or_default(GENERIC_CORES, &mut rng, "Logic"),
                    suffix
                )
            } else {
                format!("{core1} {core2} {suffix}") // Space between cores
            }
        }
        _ => {
            // Template: Prefix + Core (Default/Fallback template)
            let prefix = choose_or_default(prefixes, &mut rng, default_prefix);
            let core = choose_or_default(cores, &mut rng, default_core);

            if prefix.eq_ignore_ascii_case(core) {
                choose_or_default(GENERIC_CORES, &mut rng, "Flow").to_string()
            // Simple fallback
            } else {
                format!("{prefix} {core}")
            }
        }
    }
}
