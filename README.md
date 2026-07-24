# Model O/O-/D/D- Wireless (mxw)
Cross platform CLI for configuring Glorious Model O/O-/D/D- Wireless.

## Installation
It is a Rust (Cargo) project, install Rust from [rustup.rs](https://rustup.rs) and run `cargo install mxw`.

### Linux
**Build dependencies:** `mxw` builds `hidapi` from source, which uses the Linux `hidraw` backend and links against `libudev`. You need its development headers installed, otherwise the build fails with `Unable to find libudev`:

- Fedora / RHEL: `sudo dnf install systemd-devel`
- Debian / Ubuntu: `sudo apt install libudev-dev`
- Arch: already provided by `systemd` (part of a base install)

**Permissions (udev):** by default, accessing the mouse requires root, so you'd have to run every command with `sudo`. To use `mxw` as a normal user, install the included [`69-mxw.rules`](69-mxw.rules), which grants the logged-in user access to all supported devices:

```sh
sudo cp 69-mxw.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules
```

Then replug the mouse (or reboot) for the rule to take effect.

## Usage
Run `mxw --help` for usage information.

### Shell completions
`mxw` can generate completion scripts for `bash`, `zsh`, `fish`, `elvish`, and `powershell`. Pipe the output wherever your shell looks for completions, e.g.:

```sh
# bash
mxw completions bash | sudo tee /etc/bash_completion.d/mxw > /dev/null

# zsh (somewhere on your $fpath)
mxw completions zsh > ~/.zfunc/_mxw

# fish
mxw completions fish > ~/.config/fish/completions/mxw.fish
```

## Goal
The goal of this project is to reverse engineer the communication between Glorious Core and the Model O/O-/D/D- Wireless mice, so the mice can be used (more or less feature complete) on all platforms.

Another goal is just (re-)learning Rust, so if any Rust afficionados come across this project, feel free to tell me which parts of the code suck the most.

## About
I've been using WireShark and USBPcap for packet sniffing. Also the sloppily packaged Glorious Core software (for Windows only, hence this project) has been great help, with it's easily unarchive'able `.asar` file containing the JavaScript source.

The CLI tool is written in Rust. I've tried to keep it somewhat readable, but it's a learning project for me, so it won't be the cleanest Rust you've ever seen. This is the functionality in its current state:

### Reports
- [x] Battery percentage
- [x] Firmware version

### Configuration
- [x] LED
    - [x] Brightness
    - [x] Effects
        - [x] Glorious
        - [x] Seamless breathing
        - [x] Breathing
        - [x] Single color
        - [x] Breathing single color
        - [x] Tail
        - [x] Rave
        - [x] Wave
        - [x] Off
- [x] Active profile
- [x] Sleep delay
- [x] Lift-off distance
- [x] Polling rate
- [x] Debounce
- [x] DPI
    - [x] Active
    - [x] Stages
    - [x] Colors
- [x] Scroll inversion
- [x] Key binding
    - [x] Single key
        - [x] Scan code
        - [x] Key code
        - [x] Code
    - [x] Keyboard function
    - [x] Mouse function
    - [x] DPI modifier
    - [x] Multimedia
    - [x] None
    - [ ] Macro
    - [ ] Shortcuts

### Notes on functionality
Only macros and shortcuts remain. Shortcuts are a lot of work, as they will require a daemon listening to and acting upon "commands" from the device. I might look in to that at some point, but no promises. I might do macros, but first I'll have to check that it will even work on Linux and/or macOS.

As of now, I've found that macOS does not register "Single key" or "Multimedia" actions from the device, the same might be true for Linux. Some things are out of my hands, unfortunately... unless I reverse engineer the on-device firmware and update protocol, that is.

Another issue is that the `hidapi` abstraction that I'm currently using is a bit lacking in functionality, especially on Linux/macOS. That's not the fault of the author though, but rather the respective backends that are used. For instance, `libusb` (Linux) does not seem to "know about" usage pages and/or usage, and on macOS (maybe Linux as well) reading from a HID device via interrupt doesn't yield anything. I found this out when trying to mash together a daemon for this project, and had to abandon that part for now. To get anywhere I would need a better way to do cross platform HID comms, so if anyone has suggestions towards that, I'm all ears.

## Misc
There are some bits of code that look like utter nonsense (such as the `set_and_check` function), that's because I've tried to keep everything pretty much functionally identical to the Glorious Core source. Anyway, let me know and I will try my best to explain, and I'll provide the garbled mess that is the Glorious Core source for anyone who wants it.

## Safety
Should you be so unlucky as to somehow brick your device (as I have done myself repeatedly while working on this project), there is the option of factory resetting by pressing down both left and right mouse buttons and the scroll wheel, and hold for five seconds while the device flashes green.

However, I do not (yet) have insight into the inner workings of this device, and can therefore NOT guarantee all faults will be recoverable. Use this at your own risk!
