use super::{BusDevice, EmuError, EmuResult, ReadResult, reset_controller::ResetSource};

const STATE_FLAG_OFFSET: u32 = 0x0;
const TEST_SIG_OFFSET: u32 = 0x1;
const MSG_OFFSET: u32 = 0x4;

const SIGNATURE_SIZE: usize = 3;
const TEST_SIGNATURE: [u8; SIGNATURE_SIZE] = [0xDE, 0xB0, 0x61];

// CPU Clock speed: (236.25 MHz / 11) / 12
const CPU_TICKS_PER_MS: u64 = (((236_250_000.0 / 11.0) / 12.0) / 1000.0) as u64;
const RESET_DELAY_TICKS: u64 = 100 * CPU_TICKS_PER_MS;

#[derive(Debug)]
pub struct TestROMMonitor<T: BusDevice> {
    device: T,
    test_mem_base: u32,
    reset_trigger: ResetSource,

    current_test_signature: [u8; 3],
}

impl<T: BusDevice> TestROMMonitor<T> {
    pub fn new(device: T, test_mem_base: u32, reset_trigger: ResetSource) -> Self {
        TestROMMonitor {
            device,
            test_mem_base,
            reset_trigger,

            current_test_signature: [0; SIGNATURE_SIZE],
        }
    }

    pub fn test_mode_active(&self) -> bool {
        self.current_test_signature == TEST_SIGNATURE
    }
}

impl<T: BusDevice> BusDevice for TestROMMonitor<T> {
    fn bus_read(&mut self, addr: u32) -> EmuResult<super::ReadResult> {
        // No action needed on reads, just pass through
        self.device.bus_read(addr)
    }

    fn bus_write(&mut self, addr: u32, data: u8) -> EmuResult<()> {
        // Pass the write through to the underlying device
        self.device.bus_write(addr, data)?;

        // Detect writes to the test signature range and update the current signature
        let signature_base = self.test_mem_base + TEST_SIG_OFFSET;
        if let Some(slot) = self
            .current_test_signature
            .get_mut(addr.wrapping_sub(signature_base) as usize)
        {
            *slot = data;
        }

        // Detect writes to the test status flag
        if (addr == self.test_mem_base + STATE_FLAG_OFFSET) && self.test_mode_active() {
            match data {
                0x80 => {
                    // Test in progress
                }
                0x81 => {
                    // Soft reset requested
                    self.reset_trigger.schedule_reset(RESET_DELAY_TICKS);
                }
                0x00 => {
                    // Test completed successfully
                    return Err(EmuError::StopEmulation);
                }
                fail_code => {
                    // Test failed with error code
                    return Err(EmuError::TestROMFailure(fail_code));
                }
            }
        }

        Ok(())
    }

    fn end_of_simulation(&mut self) {
        // If in test mode, print the message buffer from the test ROM
        if self.test_mode_active() {
            let msg_base = self.test_mem_base + MSG_OFFSET;
            let msg: String = (msg_base..u32::MAX)
                .map_while(|addr| {
                    self.device.bus_read(addr).ok().and_then(|res| match res {
                        ReadResult::Data(byte) => {
                            if byte == 0 {
                                None // Null terminator, end the message
                            } else {
                                Some(byte)
                            }
                        }
                        // Open bus or read error, implies we ran past the end of memory
                        _ => None,
                    })
                })
                // Convert latin-1 to unicode
                .map(|byte| byte as char)
                .collect();
            // Trim any trailing whitespace
            println!("Test ROM message:\n{}", msg.trim_end());
        }
    }
}
