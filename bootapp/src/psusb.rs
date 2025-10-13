use defmt::{info, unwrap};

use embassy_executor::Spawner;
use embassy_rp::Peri;
use embassy_rp::peripherals::USB;
use embassy_rp::usb::Driver;
use embassy_usb::class::cdc_acm;
use embassy_usb::descriptor::{SynchronizationType, UsageType};
use embassy_usb::msos::windows_version;
use embassy_usb::{UsbDevice, msos};
use static_cell::StaticCell;

use crate::irqs::Irqs;

type PSUSBDriver = Driver<'static, USB>;
type PSUSBDevice = UsbDevice<'static, PSUSBDriver>;

static DEVICE_INTERFACE_GUID: &str = "{524aaf73-c80e-48af-923c-aadaca5bc74d}";

#[embassy_executor::task]
async fn usb_task(mut usb: PSUSBDevice) -> ! {
    usb.run().await;
}

async fn handle_echo_conn(
    class: &mut cdc_acm::CdcAcmClass<'static, PSUSBDriver>,
) -> Result<(), embassy_usb::driver::EndpointError> {
    let mut buf = [0u8; 64];
    loop {
        let n = class.read_packet(&mut buf).await?;
        if n > 0 {
            class.write_packet(&buf[..n]).await?;
        }
    }
}

#[embassy_executor::task]
async fn cdc_echo_task(mut class: cdc_acm::CdcAcmClass<'static, PSUSBDriver>) -> ! {
    loop {
        class.wait_connection().await;
        info!("USB connected");
        let _ = handle_echo_conn(&mut class).await;
        info!("USB disconnected");
    }
}

pub fn launch_usb_stack(dev: Peri<'static, USB>, spawner: Spawner) {
    let usb_driver = PSUSBDriver::new(dev, Irqs);

    let usb_config = {
        let mut config = embassy_usb::Config::new(0xc0de, 0xcafe);
        config.manufacturer = Some("Ian Thompson");
        config.product = Some("PicoSNES");
        config.max_power = 500;
        config.max_packet_size_0 = 64;
        config.composite_with_iads = true;
        config
    };

    let mut builder = {
        static CONFIG_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
        static BOS_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
        static CONTROL_BUF: StaticCell<[u8; 64]> = StaticCell::new();
        static MSOS_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();

        let builder = embassy_usb::Builder::new(
            usb_driver,
            usb_config,
            CONFIG_DESCRIPTOR.init([0; 256]),
            BOS_DESCRIPTOR.init([0; 256]),
            MSOS_DESCRIPTOR.init([0; 256]),
            CONTROL_BUF.init([0; 64]),
        );
        builder
    };

    builder.msos_descriptor(windows_version::WIN8_1, 2);

    let cdc_class = {
        static STATE: StaticCell<cdc_acm::State> = StaticCell::new();
        let state = STATE.init(cdc_acm::State::new());
        cdc_acm::CdcAcmClass::new(&mut builder, state, 64)
    };

    {
        let mut func = builder.function(0xFF, 0, 0);
        func.msos_feature(msos::CompatibleIdFeatureDescriptor::new("WINUSB", ""));
        func.msos_feature(msos::RegistryPropertyFeatureDescriptor::new(
            "DeviceInterfaceGUID",
            msos::PropertyData::Sz(DEVICE_INTERFACE_GUID),
        ));

        let mut cmd_if = func.interface();

        let mut cmd_alt = cmd_if.alt_setting(0xFF, 0, 0, None);
        cmd_alt.endpoint_bulk_out(None, 64);
        cmd_alt.endpoint_bulk_in(None, 64);

        let mut audio_if = func.interface();
        audio_if.alt_setting(0xFF, 0, 0, None);
        let mut audio_alt = audio_if.alt_setting(0xFF, 0, 0, None);
        audio_alt.endpoint_isochronous_out(
            None,
            64,
            1,
            SynchronizationType::Asynchronous,
            UsageType::DataEndpoint,
            &[],
        );
    }

    let usb = builder.build();
    unwrap!(spawner.spawn(usb_task(usb)));
    unwrap!(spawner.spawn(cdc_echo_task(cdc_class)));
}
