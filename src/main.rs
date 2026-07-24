pub mod args;
pub mod util;
pub mod config;
pub mod report;

use clap::Parser;
use hidapi::HidApi;
use util::none::None;
use util::devices;
use args::{ Args, Kind, Report, Config };

fn main() {
    // Parse the command line arguments
    let args = Args::parse();

    // Interface with platform specific 'hidapi'
    let hid_api = HidApi::new().unwrap();

    // Try to find a matching Glorious device
    let not_found = format!(
        "No matching device found! Supported devices: {}",
        devices::supported_mice_list()
    );
    let (device, mouse_model, wired) = devices::find_device(&hid_api)
        .none(&not_found);

    // Act upon command line arguments
    match args.kind {
        // mxw report
        Kind::Report(report) => match report {
            // mxw report battery
            Report::Battery =>
                report::battery::get(&device, wired),

            // mxw report device
            Report::Device =>
                report::device::get(mouse_model, wired),

            // mxw report firmware
            Report::Firmware =>
                report::firmware::get(&device, wired),
        },

        // mxw config
        Kind::Config(config) => match config {
            // mxw config bind ...
            Config::Bind { profile, button, binding } =>
                config::bind::set(&device, profile, button, binding),

            // mxw config scroll <DIRECTION>
            Config::Scroll { direction } =>
                config::scroll::set(&device, direction),

            // mxw config profile <ID>
            Config::Profile { id } =>
                config::profile::set(&device, id),

            // mxw config sleep <MINUTES> [SECONDS]
            Config::Sleep { minutes, seconds} =>
                config::sleep::set(&device, minutes, seconds),

            // mxw config led-brightness <WIRED> [WIRELESS]
            Config::LEDBrightness { wired, wireless } =>
                config::led_brightness::set(&device, wired, wireless),

            // mxw config led-effect <EFFECT> ...
            Config::LEDEffect { profile, effect } =>
                config::led_effect::set(&device, profile, effect),

            // mxw config polling-rate <MS>
            Config::PollingRate { ms } =>
                config::polling_rate::set(&device, ms),

            // mxw config lift-off <MM>
            Config::LiftOff { mm } =>
                config::lift_off::set(&device, mm),

            // mxw config debounce <MS>
            Config::Debounce { profile, ms } =>
                config::debounce::set(&device, profile, ms),

            // mxw config dpi-stage <ID>
            Config::DPIStage { profile, id } =>
                config::dpi_stage::set(&device, profile, id),

            // mxw config dpi-stages <STAGES>...
            Config::DPIStages { profile, stages } =>
                config::dpi_stages::set(&device, profile, stages),

            // mxw config dpi-colors <COLORS>...
            Config::DPIColors { profile, colors } =>
                config::dpi_colors::set(&device, profile, colors),
        },
    }
}
