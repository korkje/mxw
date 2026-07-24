use crate::util::devices::Device;
use crate::args::{ScrollDirection, Button, Binding, MouseFn};
use super::bind;

pub fn set(device: &Device, direction: ScrollDirection) {
    for i in 1..=3 {
        match direction {
            ScrollDirection::Default => {
                // Up => Up
                bind::set(
                    device, i,
                    Button::ScrollUp,
                    Binding::Mouse(MouseFn::ScrollUp)
                );

                // Down => Down
                bind::set(
                    device, i,
                    Button::ScrollDown,
                    Binding::Mouse(MouseFn::ScrollDown)
                );
            },

            ScrollDirection::Invert => {
                // Up => Down
                bind::set(
                    device, i,
                    Button::ScrollUp,
                    Binding::Mouse(MouseFn::ScrollDown)
                );

                // Down => Up
                bind::set(
                    device, i,
                    Button::ScrollDown,
                    Binding::Mouse(MouseFn::ScrollUp)
                );
            },
        }
    }
}
