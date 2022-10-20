#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ClockCycles(usize);

impl From<usize> for ClockCycles {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<ClockCycles> for usize {
    fn from(clock_cycles: ClockCycles) -> Self {
        clock_cycles.0
    }
}