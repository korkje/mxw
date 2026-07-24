# Model O Wireless (mow)
Cross platform CLI for configuring Glorious Model O Wireless.

## Goal
The goal of this project is to reverse engineer the communication between Glorious Core and the Model O Wireless mouse, so the mouse can be used (more or less feature complete) on all platforms.

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

## Installation
It is a Rust (Cargo) project, so just run `cargo build --release` in the root folder, create a symlink somewhere in your `PATH`, and you should be good to go. The CLI has built in `--help`, so use that to understand usage.

I'll look into making it more easily available at some point, maybe through `brew`, `apt-get` etc., though I might add support for more devices and have to change the project's name before then.

### Arch (thanks, [crstian19](https://github.com/crstian19))
On Arch linux, you can install this from [AUR](https://aur.archlinux.org) with `paru -S mow-git`.

## Misc
With some minor alteration, my findings probably also apply to Model D as well as reduced size versions of both devices, but I don't have access to those myself, so I will not be able to extend support at least for now.

I might at some point compile and upload a detailed writeup describing the protocol in it's entirety, but until that time this repo should (hopefully) be readable enough for others wanting to look into how it works. There are some bits of code that look like utter nonsense (such as the `set_and_check` function), that's because I've tried to keep everything pretty much functionally identical to the Glorious Core source. Anyway, let me know and I will try my best to explain, and I'll provide the garbled mess that is the Glorious Core source for anyone who wants it.

## Safety
Should you be so unlucky as to somehow brick your device (as I have done myself repeatedly while working on this project), there is the option of factory resetting by pressing down both left and right mouse buttons and the scroll wheel, and hold for five seconds while the device flashes green.

However, I do not (yet) have insight into the inner workings of this device, and can therefore NOT guarantee all faults will be recoverable. Use this at your own risk!
