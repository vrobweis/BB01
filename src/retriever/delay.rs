use chrono::Duration;
use std::{cell::RefCell, rc::Rc};
use tokio::time::{interval_at, sleep_until, Instant, Interval};
impl Default for Delay {
    fn default() -> Self {
        let dur = Duration::milliseconds(200).to_std().unwrap();
        Self(
            Instant::now(),
            Rc::new(RefCell::new(interval_at(Instant::now() + dur, dur))),
        )
    }
}
#[derive(Clone, Debug)]
pub struct Delay(pub Instant, pub Rc<RefCell<Interval>>);
impl Delay {
    pub async fn delay_if(&mut self, delay: Duration) -> &mut Self {
        let until = self.0 + delay.to_std().unwrap();
        if until > Instant::now() {
            sleep_until(until).await;
            self.1.as_ref().borrow_mut().tick().await;
        }
        self.0 = Instant::now();
        self
    }
}
