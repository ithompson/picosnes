use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::USB;
use embassy_rp::usb;

bind_interrupts!(pub struct Irqs {
    USBCTRL_IRQ => usb::InterruptHandler<USB>;
});
