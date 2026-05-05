use std::{cell::RefCell, rc::Rc};

use crate::components::signal::PulseSignal;

#[derive(Debug, Default)]
struct ResetControllerInner {
    current_tick: u64,
    next_reset_tick: Option<u64>,
}

#[derive(Debug)]
pub struct ResetController {
    inner: Rc<RefCell<ResetControllerInner>>,
    reset_signal: PulseSignal,
}

#[derive(Clone, Debug)]
pub struct ResetSource {
    resetter: Rc<RefCell<ResetControllerInner>>,
}

impl ResetController {
    pub fn new(reset_signal: PulseSignal) -> Self {
        Self {
            inner: Default::default(),
            reset_signal,
        }
    }

    pub fn trigger_reset(&mut self) {
        self.reset_signal.trigger()
    }

    pub fn tick(&mut self) {
        let mut inner = self.inner.borrow_mut();
        inner.current_tick += 1;
        if let Some(x) = inner.next_reset_tick
            && x <= inner.current_tick
        {
            inner.next_reset_tick = None;
            self.reset_signal.trigger();
        }
    }

    pub fn make_reset_source(&mut self) -> ResetSource {
        ResetSource {
            resetter: Rc::clone(&self.inner),
        }
    }
}

impl ResetSource {
    pub fn schedule_reset(&self, delay_ticks: u64) {
        let mut resetter = self.resetter.borrow_mut();
        let reset_tick = resetter.current_tick + delay_ticks;

        resetter.next_reset_tick = resetter
            .next_reset_tick
            .map_or(Some(reset_tick), |x| Some(std::cmp::min(x, reset_tick)))
    }

    pub fn trigger_reset(&self) {
        self.schedule_reset(0);
    }
}
