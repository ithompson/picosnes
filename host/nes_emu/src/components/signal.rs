use std::cell::Cell;
use std::rc::Rc;

/// Driver for a level-oriented signal. Emulator components can use this to
/// drive boolean-valued signals between each other. Roughly corresponds
/// to a physical wire in the actual design.
#[derive(Debug, Default)]
pub struct LevelSignal {
    state: Rc<Cell<bool>>,
}

/// Object for observing the current value of a LevelSignal.
#[derive(Clone, Debug)]
pub struct LevelReceiver {
    state: Rc<Cell<bool>>,
}

impl LevelSignal {
    /// Create a new LevelSignal. Initializes to the false state.
    pub fn new() -> Self {
        Default::default()
    }

    /// Update the state of this signal.
    pub fn set(&mut self, value: bool) {
        self.state.set(value);
    }

    /// Create a new receiver for this signal. There is no limit to the number
    /// of receivers that can be active for a single signal.
    pub fn make_receiver(&mut self) -> LevelReceiver {
        LevelReceiver {
            state: Rc::clone(&self.state),
        }
    }
}

impl LevelReceiver {
    /// Get the current state of the associated signal.
    pub fn get(&self) -> bool {
        self.state.get()
    }
}

/// Driver for a pulse-oriented signal. Roughly corresponds to an edge-triggered
/// interrupt in the actual design.
#[derive(Debug, Default)]
pub struct PulseSignal {
    pulse_id: Rc<Cell<u64>>,
}

/// Receiver for a PulseSignal. Each receiver independently tracks whether a
/// pulse has occurred since the last time it was checked.
#[derive(Clone, Debug)]
pub struct PulseReceiver {
    pulse_id: Rc<Cell<u64>>,
    last_pulse_id: u64,
}

impl PulseSignal {
    /// Create a new PulseSignal.
    pub fn new() -> Self {
        Default::default()
    }

    /// Broadcast a pulse on this signal.
    pub fn trigger(&mut self) {
        self.pulse_id.update(|x| x + 1);
    }

    /// Create a new receiver for this signal.
    pub fn make_receiver(&mut self) -> PulseReceiver {
        PulseReceiver {
            pulse_id: Rc::clone(&self.pulse_id),
            last_pulse_id: self.pulse_id.get(),
        }
    }
}

impl PulseReceiver {
    /// Check if a pulse has been triggered since the last time this function
    /// was called. If multiple pulses have occurred, all of them are consumed.
    pub fn check_and_acknowledge(&mut self) -> bool {
        let pulse_id = self.pulse_id.get();
        let pulse_occurred = pulse_id != self.last_pulse_id;
        self.last_pulse_id = pulse_id;
        pulse_occurred
    }

    /// Check if a pulse has been triggered since the last call to
    /// check_and_acknowledge. Does not consume the pulse.
    pub fn peek(&self) -> bool {
        self.pulse_id.get() != self.last_pulse_id
    }
}
