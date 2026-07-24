pub mod args;
pub mod util;
pub mod config;
pub mod report;

use clap::Parser;
use hidapi::HidApi;
use util::none::None;
use args::{ Args, Kind, Report, Config };

fn main() {
    // Parse the command line arguments
    let args = Args::parse();

    // Interface with platform specific 'hidapi'
    let hid_api = HidApi::new().unwrap();

    // Try to find a matching device
    let device_info = hid_api
        .device_list()
        .filter(|d| {
            // Glorious' vendor id
            d.vendor_id() == 0x258A &&

            // Model O product id
            [0x2011, 0x2022].contains(&d.product_id()) &&

            // Feature report interface
            d.interface_number() == 0x02
        })
        // Get wired (0x2011) if available
        .min_by(|a, b| a.product_id().cmp(&b.product_id()))
        .none("No matching device found!");

    // Product id indicates whether wired
    let wired = device_info.product_id() == 0x2011;

    // Connect to the device
    let device = device_info.open_device(&hid_api).unwrap();

    // Act upon command line arguments
    match args.kind {
        // mow report
        Kind::Report(report) => match report {
            // mow report battery
            Report::Battery =>
                report::battery::get(&device, wired),

            // mow report firmware
            Report::Firmware =>
                report::firmware::get(&device, wired),
        },

        // mow config
        Kind::Config(config) => match config {
            // mow config bind ...
            Config::Bind { profile, button, binding } =>
                config::bind::set(&device, profile, button, binding),

            // mow config scroll <DIRECTION>
            Config::Scroll { direction } =>
                config::scroll::set(&device, direction),

            // mow config profile <ID>
            Config::Profile { id } =>
                config::profile::set(&device, id),

            // mow config sleep <MINUTES> [SECONDS]
            Config::Sleep { minutes, seconds} =>
                config::sleep::set(&device, minutes, seconds),

            // mow config led-brightness <WIRED> [WIRELESS]
            Config::LEDBrightness { wired, wireless } =>
                config::led_brightness::set(&device, wired, wireless),

            // mow config led-effect <EFFECT> ...
            Config::LEDEffect { profile, effect } =>
                config::led_effect::set(&device, profile, effect),

            // mow config polling-rate <MS>
            Config::PollingRate { ms } =>
                config::polling_rate::set(&device, ms),

            // mow config lift-off <MM>
            Config::LiftOff { mm } =>
                config::polling_rate::set(&device, mm),

            // mow config debounce <MS>
            Config::Debounce { profile, ms } =>
                config::debounce::set(&device, profile, ms),

            // mow config dpi-stage <ID>
            Config::DPIStage { profile, id } =>
                config::dpi_stage::set(&device, profile, id),

            // mow config dpi-stages <STAGES>...
            Config::DPIStages { profile, stages } =>
                config::dpi_stages::set(&device, profile, stages),

            // mow config dpi-colors <COLORS>...
            Config::DPIColors { profile, colors } =>
                config::dpi_colors::set(&device, profile, colors),
        },
    }
}
