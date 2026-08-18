use std::time::{Duration, Instant};

#[inline]
pub fn to_deadline(timeout: Duration) -> Option<Instant> {
    // A timeout which is too large to represent is treated as having no
    // deadline.
    Instant::now().checked_add(timeout)
}
