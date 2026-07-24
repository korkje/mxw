use clap::{ Parser, Subcommand, ValueEnum };
use clap::builder::{ PossibleValuesParser, TypedValueParser };
use clap::builder::styling::{ AnsiColor, Styles };
use clap_complete::Shell;
use crate::util::color::{ self, Color };
use crate::util::key::{ self, Key };

// Colored help, roughly matching clap 3's old default (clap 4 ships plain)
const HELP_STYLES: Styles = Styles::styled()
    .header(AnsiColor::Yellow.on_default().bold())
    .usage(AnsiColor::Yellow.on_default().bold())
    .literal(AnsiColor::Green.on_default())
    .placeholder(AnsiColor::Green.on_default());

#[derive(Parser)]
#[command(
    // From Cargo.toml
    author, about, version,

    // Don't need a subcommand when we have flags
    disable_help_subcommand = true,

    // Same reported version for subcommands
    propagate_version = true,

    // Restore colored help output
    styles = HELP_STYLES,
)]
pub struct Args {
    /// Print the feature reports that would be sent instead of touching the device
    #[arg(long, global = true, hide = true)]
    pub dry: bool,

    #[command(subcommand)]
    pub kind: Kind,
}

#[derive(Subcommand)]
pub enum Kind {
    /// Retrieve information about the device
    #[command(subcommand)]
    Report(Report),

    /// Change the device's various settings
    #[command(subcommand)]
    Config(Config),

    /// Generate shell completions
    Completions {
        #[arg(value_enum)]
        shell: Shell,
    },
}

#[derive(Subcommand)]
pub enum Report {
    /// Battery percentage (if available)
    Battery,

    /// Device information
    Device,

    /// Device firmware version
    Firmware,
}

#[derive(Subcommand)]
pub enum Config {
    /// Active profile by id
    Profile {
        #[arg(value_parser = clap::value_parser!(u8).range(1..=3))]
        id: u8,
    },

    /// LED Effect
    LEDEffect {
        /// Profile id (1-3)
        #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=3))]
        profile: u8,

        #[command(subcommand)]
        effect: Effect,
    },

    /// LED brightness value[s] (0-255)
    LEDBrightness {
        wired: u8,

        #[arg(help = "[default: <WIRED>]")]
        wireless: Option<u8>,
    },

    /// Sleep delay in minutes [and seconds]
    Sleep {
        minutes: u8,

        #[arg(help = "[default: 0]")]
        seconds: Option<u8>,
    },

    /// Active DPI stage by id
    DPIStage {
        /// Profile id (1-3)
        #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=3))]
        profile: u8,

        #[arg(value_parser = clap::value_parser!(u8).range(1..=4))]
        id: u8,
    },

    /// Set DPI stages (200-19000)
    DPIStages {
        /// Profile id (1-3)
        #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=3))]
        profile: u8,

        #[arg(
            value_name = "STAGE",
            num_args = 4,
            value_parser = clap::value_parser!(u16).range(100..=19000),
            default_values = ["400", "800", "1600", "3200"],
        )]
        stages: Vec<u16>,
    },

    /// Set DPI stage colors
    DPIColors {
        /// Profile id (1-3)
        #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=3))]
        profile: u8,

        #[arg(
            value_name = "COLOR",
            num_args = 4,
            value_parser = color::parse_hex,
            default_values = ["FFFF00", "0000FF", "FF0000", "00FF00"],
        )]
        colors: Vec<Color>,
    },

    /// Lift-off distance in mm
    LiftOff {
        #[arg(value_parser = clap::value_parser!(u8).range(1..=2))]
        mm: u8,
    },

    /// Polling rate in ms
    PollingRate {
        #[arg(value_parser = PossibleValuesParser::new(["1", "2", "4", "8"]).map(|s| s.parse::<u8>().unwrap()))]
        ms: u8,
    },

    /// Debounce in ms (0-16)
    Debounce {
        /// Profile id (1-3)
        #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=3))]
        profile: u8,

        #[arg(value_parser = clap::value_parser!(u8).range(0..=16))]
        ms: u8,
    },

    /// Key binding
    Bind {
        /// Profile id (1-3)
        #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=3))]
        profile: u8,

        /// Mouse button
        #[arg(value_enum)]
        button: Button,

        #[command(subcommand)]
        binding: Binding,
    },

    /// Scroll inversion
    Scroll {
        #[arg(value_enum)]
        direction: ScrollDirection,
    },
}

#[derive(Clone, ValueEnum)]
pub enum ScrollDirection {
    Default,
    Invert,
}

#[derive(Subcommand)]
pub enum Effect {
    /// Name says it all
    Glorious {
        /// Effect rate, 0-100
        #[arg(short, long, help = "[default: 40]", value_parser = clap::value_parser!(u8).range(0..=100))]
        rate: Option<u8>,
    },

    /// Cycle through all colors
    Cycle {
        /// Effect rate, 0-100
        #[arg(short, long, help = "[default: 40]", value_parser = clap::value_parser!(u8).range(0..=100))]
        rate: Option<u8>,
    },

    /// Pulse on/off through given colors
    Pulse {
        /// Effect rate, 0-100
        #[arg(short, long, help = "[default: 40]", value_parser = clap::value_parser!(u8).range(0..=100))]
        rate: Option<u8>,

        /// From 2 to 6 colors in hex format
        #[arg(required = true, num_args = 2..=6, value_parser = color::parse_hex)]
        colors: Vec<Color>,
    },

    /// Solid color
    Solid {
        /// Color in hex format
        #[arg(value_parser = color::parse_hex)]
        color: Color,
    },

    /// Pulse on/off one color
    PulseOne {
        /// Effect rate, 0-100
        #[arg(short, long, help = "[default: 40]", value_parser = clap::value_parser!(u8).range(0..=100))]
        rate: Option<u8>,

        #[arg(value_parser = color::parse_hex)]
        color: Color,
    },

    /// Glorious, but colors don't "move"
    Tail {
        /// Effect rate, 0-100
        #[arg(short, long, help = "[default: 40]", value_parser = clap::value_parser!(u8).range(0..=100))]
        rate: Option<u8>,
    },

    /// Strobe-like effect
    Rave {
        /// Effect rate, 0-100
        #[arg(short, long, help = "[default: 40]", value_parser = clap::value_parser!(u8).range(0..=100))]
        rate: Option<u8>,

        /// 1 or 2 colors in hex format
        #[arg(required = true, num_args = 1..=2, value_parser = color::parse_hex)]
        colors: Vec<Color>,
    },

    /// Glorious, but more circus
    Wave {
        /// Effect rate, 0-100
        #[arg(short, long, help = "[default: 40]", value_parser = clap::value_parser!(u8).range(0..=100))]
        rate: Option<u8>,
    },

    /// No effect, LED off
    Off,
}

#[derive(Clone, ValueEnum)]
pub enum Button {
    Left,
    Right,
    Scroll,
    Forward,
    Back,
    DPI,
    ScrollUp,
    ScrollDown,
}

#[derive(Subcommand)]
pub enum Binding {
    /// Single key
    Key {
        #[command(subcommand)]
        kind: KeyKind,
    },

    /// Keyboard function
    #[command(subcommand)]
    Keyboard(KeyboardFn),

    /// Mouse function
    #[command(subcommand)]
    Mouse(MouseFn),

    /// DPI modifier
    #[command(subcommand)]
    DPI(DPIFn),

    /// (not implemented) Macro
    Macro,

    /// Multimedia
    #[command(subcommand)]
    Media(MediaFn),

    /// (not implemented) Launch applications etc.
    Shortcut,

    /// Do nothing
    None,
}

#[derive(Subcommand)]
pub enum KeyKind {
    /// Hardware scan code
    ScanCode {
        #[arg(value_parser = key::parse_scan_code)]
        key: Key,

        /// Optional modifier
        #[arg(short, long, value_parser = key::parse_scan_code_mod)]
        modifier: Option<Key>,
    },

    /// JS-style KeyCode
    KeyCode {
        #[arg(value_parser = key::parse_key_code)]
        key: Key,

        /// Optional modifier
        #[arg(short, long, value_parser = key::parse_key_code_mod)]
        modifier: Option<Key>,
    },

    /// JS-style Code
    Code {
        #[arg(value_parser = key::parse_code)]
        key: Key,

        /// Optional modifier
        #[arg(short, long, value_parser = key::parse_code_mod)]
        modifier: Option<Key>,
    },
}

#[derive(Subcommand)]
pub enum MouseFn {
    Left,
    Right,
    Scroll,
    Forward,
    Back,
    ScrollUp,
    ScrollDown,
    ProfileCycleUp,
    ProfileCycleDown,
    BatteryStatus,
}

#[derive(Subcommand)]
pub enum KeyboardFn {
    ProfileCycleUp,
    ProfileCycleDown,
    LayerCycleUp,
    LayerCycleDown,
}

#[derive(Subcommand)]
pub enum DPIFn {
    StageUp,
    StageDown,
    CycleUp,
    CycleDown,
}

#[derive(Subcommand)]
pub enum MediaFn {
    Player,
    PlayPause,
    Next,
    Previous,
    Stop,
    Mute,
    VolumeUp,
    VolumeDown,
}
