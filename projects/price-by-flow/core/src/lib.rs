use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Outcome {
    Yes,
    No,
}

#[derive(Debug, Clone, Serialize)]
pub struct Market {
    pub question: String,
    pub price_yes: f64,
    pub total_yes: u64,
    pub total_no: u64,
}

impl Market {
    pub fn new(question: &str, initial_price: f64) -> Self {
        assert!(initial_price > 0.0 && initial_price < 1.0);
        Self {
            question: question.to_string(),
            price_yes: initial_price,
            total_yes: 0,
            total_no: 0,
        }
    }

    pub fn buy(&mut self, outcome: Outcome, shares: u64) -> f64 {
        match outcome {
            Outcome::Yes => self.total_yes += shares,
            Outcome::No => self.total_no += shares,
        }
        let total = (self.total_yes + self.total_no) as f64;
        self.price_yes = self.total_yes as f64 / total;
        self.price_yes * shares as f64
    }
}
