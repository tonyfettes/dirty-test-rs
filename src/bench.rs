/// A trait describes how to obtain measure current time and obtain time
/// duration.
pub trait Time: Sync + Send {

    /// Starts the timer
    fn start(&mut self);

    /// Ends the timer, and return the time duration between calling to
    /// [`start`](trait.Time.html#tymethod.start) and
    /// [`end`](trait.Time.html#tymethod.end).
    fn end(&mut self);
}

struct VacuumTimer;

impl Time for VacuumTimer {
    fn start(&mut self) { }
    fn end(&mut self) { }
}

#[cfg(feature = "spin_once")]
static TIMER: spin::Once<&dyn Time> = spin::Once::new();

pub fn set_timer(timer: &'static dyn Time) {
    if TIMER.is_completed() {
        panic!("timer has already been initialized");
    } else {
        TIMER.call_once(|| timer);
    }
}
