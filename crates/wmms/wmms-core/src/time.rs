use core::time::Duration;
use crate::error::{Result, WMMSCoreError};

#[repr(transparent)]
#[derive(Clone,Copy,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct Tick(pub u64);

#[repr(transparent)]
#[derive(Clone,Copy,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct TickDelta(pub u64);

#[derive(Clone,Copy,Debug)]
pub struct TickRate {
    pub ticks_per_second: u32,
}

impl TickRate{
    pub fn duration_to_ticks(&self, d: Duration) -> TickDelta{
        let nanos = d.as_nanos();
        let tps = self.ticks_per_second as u128;
        let ticks = (nanos * tps + 1_000_000_000u128 / 2) / 1_000_000_000u128;
        TickDelta(ticks as u64)
    }

    pub fn ticks_to_duration(&self, dt: TickDelta) -> Duration{
        let tps = self.ticks_per_second as u128;
        let nanos = (dt.0 as u128) * 1_000_000_000u128 / tps;
        Duration::from_nanos(nanos as u64)
    }
}

impl Tick{
    pub fn checked_add(self, delta: TickDelta) -> Result<Tick>{
        self.0.checked_add(delta.0)
            .map(Tick)
            .ok_or(WMMSCoreError::TickOverflow)
    }
}