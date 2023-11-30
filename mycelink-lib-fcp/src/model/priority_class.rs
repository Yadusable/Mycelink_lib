use std::cmp::Ordering;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PriorityClass {
    Maximum,
    VeryHigh,
    High,
    Medium,
    Low,
    VeryLow,
    Pause,
}

impl From<&PriorityClass> for u8 {
    fn from(value: &PriorityClass) -> Self {
        match value {
            PriorityClass::Maximum => 0,
            PriorityClass::VeryHigh => 1,
            PriorityClass::High => 2,
            PriorityClass::Medium => 3,
            PriorityClass::Low => 4,
            PriorityClass::VeryLow => 5,
            PriorityClass::Pause => 6,
        }
    }
}

impl From<&PriorityClass> for Box<str> {
    fn from(value: &PriorityClass) -> Self {
        u8::from(value).to_string().into_boxed_str()
    }
}

impl PartialOrd for PriorityClass {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PriorityClass {
    fn cmp(&self, other: &Self) -> Ordering {
        u8::from(self).cmp(&other.into()).reverse()
    }
}
