pub mod args;
pub mod util;
pub mod config;
pub mod report;

use clap::{ Parser, CommandFactory };
use hidapi::HidApi;
use util::fail::Fail;
use util::devices;
use args::{ Args, Kind, Report, Config };

fn main() {
    let args = Args::parse();

    if let Kind::Completions { shell } = &args.kind {
        let mut cmd = Args::command();
        let name = cmd.get_name().to_string();
        clap_complete::generate(*shell, &mut cmd, name, &mut std::io::stdout());
        return;
    }

    if let Kind::Keys = args.kind {
        util::key::list();
        return;
    }
    let (device, mouse_model, wired) = if args.dry {
        (devices::Device::Dry, devices::DRY_MODEL, false)
    } else {
        let hid_api = HidApi::new()
            .map_err(|e| format!("failed to initialize HID API: {}", e))
            .or_fail();

        let (device, mouse_model, wired) = devices::find_device(&hid_api).or_fail();

        (devices::Device::Real(device), mouse_model, wired)
    };

    match args.kind {
        Kind::Report(report) => match report {
            Report::Battery => report::battery::get(&device, wired),
            Report::Device => report::device::get(mouse_model, wired),
            Report::Firmware => report::firmware::get(&device, wired),
        },
        Kind::Config(config) => match config {
            Config::Bind { profile, button, binding } => config::bind::set(&device, profile, button, binding),
            Config::Scroll { direction } => config::scroll::set(&device, direction),
            Config::Profile { id } => config::profile::set(&device, id),
            Config::Sleep { minutes, seconds} => config::sleep::set(&device, minutes, seconds),
            Config::LEDBrightness { wired, wireless } => config::led_brightness::set(&device, wired, wireless),
            Config::LEDEffect { profile, effect } => config::led_effect::set(&device, profile, effect),
            Config::PollingRate { ms } => config::polling_rate::set(&device, ms),
            Config::LiftOff { mm } => config::lift_off::set(&device, mm),
            Config::Debounce { profile, ms } => config::debounce::set(&device, profile, ms),
            Config::DPIStage { profile, id } => config::dpi_stage::set(&device, profile, id),
            Config::DPIStages { profile, stages } => config::dpi_stages::set(&device, profile, stages),
            Config::DPIColors { profile, colors } => config::dpi_colors::set(&device, profile, colors),
        },
        Kind::Keys | Kind::Completions { .. } => unreachable!(),
    }
}
