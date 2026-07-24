use clap::{ self, Parser, Subcommand, ArgEnum };
use crate::util::color::{ self, Color};
use crate::util::key::{ self, Key };
use crate::util::range::in_range;

#[derive(Parser)]
#[clap(
    // From Cargo.toml
    author, version, about,

    // Don't need a subcommand when we have flags
    disable_help_subcommand = true,

    // Same reported version for subcommands
    propagate_version = true,

    // Prevent automatic uppercase short version
    mut_arg("version", |x| { x.short('v') }),
)]
pub struct Args {
    #[clap(subcommand)]
    pub kind: Kind,
}

#[derive(Subcommand)]
pub enum Kind {
    /// Retrieve information about the device
    #[clap(subcommand)]
    Report(Report),

    /// Change the device's various settings
    #[clap(subcommand)]
    Config(Config),
}

#[derive(Subcommand)]
pub enum Report {
    /// Battery percentage (if available)
    Battery,

    /// Device firmware version
    Firmware,
}

#[derive(Subcommand)]
pub enum Config {
    /// Active profile by id
    Profile {
        #[clap(possible_values(["1", "2", "3"]))]
        id: u8,
    },

    /// LED Effect
    LEDEffect {
        /// Profile id (1-3)
        #[clap(
            short, long,
            possible_values(["1", "2", "3"]),
        )]
        profile: u8,

        #[clap(subcommand)]
        effect: Effect,
    },

    /// LED brightness value[s] (0-255)
    LEDBrightness {
        wired: u8,

        #[clap(help = "[default: <WIRED>]")]
        wireless: Option<u8>,
    },

    /// Sleep delay in minutes [and seconds]
    Sleep {
        minutes: u8,

        #[clap(help = "[default: 0]")]
        seconds: Option<u8>,
    },

    /// Active DPI stage by id
    DPIStage {
        /// Profile id (1-3)
        #[clap(
            short, long,
            possible_values(["1", "2", "3"]),
        )]
        profile: u8,

        #[clap(possible_values(["1", "2", "3", "4"]))]
        id: u8,
    },

    /// Set DPI stages (200-19000)
    DPIStages {
        /// Profile id (1-3)
        #[clap(
            short, long,
            possible_values(["1", "2", "3"])
        )]
        profile: u8,

        #[clap(
            name = "stage",
            number_of_values = 4,
            validator = in_range(&(100..=19000)),
            default_values(&["400", "800", "1600", "3200"]),
        )]
        stages: Vec<u16>,
    },

    /// Set DPI stage colors
    DPIColors {
        /// Profile id (1-3)
        #[clap(
            short, long,
            possible_values(["1", "2", "3"])
        )]
        profile: u8,

        #[clap(
            name = "COLOR",
            number_of_values = 4,
            parse(try_from_str = color::parse_hex),
            default_values(&["FFFF00", "0000FF", "FF0000", "00FF00"]),
        )]
        colors: Vec<Color>,
    },

    /// Lift-off distance in mm
    LiftOff {
        #[clap(possible_values(["1", "2"]))]
        mm: u8,
    },

    /// Polling rate in ms
    PollingRate {
        #[clap(possible_values(["1", "2", "4", "8"]))]
        ms: u8,
    },

    /// Debounce in ms (0-16)
    Debounce {
        /// Profile id (1-3)
        #[clap(
            short, long,
            possible_values(["1", "2", "3"]),
        )]
        profile: u8,

        #[clap(validator = in_range(&(0..=16)))]
        ms: u8,
    },

    /// Key binding
    Bind {
        /// Profile id (1-3)
        #[clap(
            short, long,
            possible_values(["1", "2", "3"]),
        )]
        profile: u8,

        /// Mouse button
        #[clap(arg_enum)]
        button: Button,

        #[clap(subcommand)]
        binding: Binding,
    },

    /// Scroll inversion
    Scroll {
        #[clap(arg_enum)]
        direction: ScrollDirection,
    },
}

#[derive(Clone, ArgEnum)]
pub enum ScrollDirection {
    Default,
    Invert,
}

#[derive(Subcommand)]
pub enum Effect {
    /// Name says it all
    Glorious {
        /// Effect rate, 0-100
        #[clap(
            short, long,
            help = "[default: 40]",
            validator = in_range(&(0..=100)),
        )]
        rate: Option <u8>,
    },

    /// Cycle through all colors
    Cycle {
        /// Effect rate, 0-100
        #[clap(
            short, long,
            help = "[default: 40]",
            validator = in_range(&(0..=100)),
        )]
        rate: Option<u8>,
    },

    /// Pulse on/off through given colors
    Pulse {
        /// Effect rate, 0-100
        #[clap(
            short, long,
            help = "[default: 40]",
            validator = in_range(&(0..=100)),
        )]
        rate: Option<u8>,

        /// From 2 to 6 colors in hex format
        #[clap(
            required = true,
            min_values = 2, max_values = 6,
            parse(try_from_str = color::parse_hex),
        )]
        colors: Vec<Color>,
    },

    /// Solid color
    Solid {
        /// Color in hex format
        #[clap(parse(try_from_str = color::parse_hex))]
        color: Color,
    },

    /// Pulse on/off one color
    PulseOne {
        /// Effect rate, 0-100
        #[clap(
            short, long,
            help = "[default: 40]",
            validator = in_range(&(0..=100)),
        )]
        rate: Option<u8>,

        #[clap(parse(try_from_str = color::parse_hex))]
        color: Color,
    },

    /// Glorious, but colors don't "move"
    Tail {
        /// Effect rate, 0-100
        #[clap(
            short, long,
            help = "[default: 40]",
            validator = in_range(&(0..=100)),
        )]
        rate: Option<u8>,
    },

    /// Strobe-like effect
    Rave {
        /// Effect rate, 0-100
        #[clap(
            short, long,
            help = "[default: 40]",
            validator = in_range(&(0..=100)),
        )]
        rate: Option<u8>,

        /// 1 or 2 colors in hex format
        #[clap(
            required = true,
            min_values = 1, max_values = 2,
            parse(try_from_str = color::parse_hex),
        )]
        colors: Vec<Color>,
    },

    /// Glorious, but more circus
    Wave {
        /// Effect rate, 0-100
        #[clap(
            short, long,
            help = "[default: 40]",
            validator = in_range(&(0..=100)),
        )]
        rate: Option<u8>,
    },

    /// No effect, LED off
    Off,
}

#[derive(Clone, ArgEnum)]
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
        #[clap(subcommand)]
        kind: KeyKind,
    },

    /// Keyboard function
    #[clap(subcommand)]
    Keyboard(KeyboardFn),

    /// Mouse function
    #[clap(subcommand)]
    Mouse(MouseFn),

    /// DPI modifier
    #[clap(subcommand)]
    DPI(DPIFn),

    /// (not implemented) Macro
    Macro,

    /// Multimedia
    #[clap(subcommand)]
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
        #[clap(parse(try_from_str = key::parse_scan_code))]
        key: Key,

        /// Optional modifier
        #[clap(
            short, long,
            parse(try_from_str = key::parse_scan_code_mod),
        )]
        modifier: Option<Key>,
    },

    /// JS-style KeyCode
    KeyCode {
        #[clap(parse(try_from_str = key::parse_key_code))]
        key: Key,

        /// Optional modifier
        #[clap(
            short, long,
            parse(try_from_str = key::parse_key_code_mod),
        )]
        modifier: Option<Key>,
    },

    /// JS-style Code
    Code {
        #[clap(parse(try_from_str = key::parse_code))]
        key: Key,

        /// Optional modifier
        #[clap(
            short, long,
            parse(try_from_str = key::parse_code_mod),
        )]
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
