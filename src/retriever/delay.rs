use chrono::Duration;
use tokio::time::{sleep_until, Instant};

impl Default for Delay {
    fn default() -> Self { Self(Instant::now()) }
}
#[derive(Clone, Debug)]
pub struct Delay(pub Instant);
impl Delay {
    pub async fn delay_if(&mut self, delay: Duration) {
        let until = self.0 + delay.to_std().unwrap();
        if until > Instant::now() {
            sleep_until(until).await;
        }
        self.0 = Instant::now();
    }
}
