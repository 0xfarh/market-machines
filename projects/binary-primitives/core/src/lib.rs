use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Outcome {
    Yes,
    No,
}

#[derive(Debug, Clone, Serialize)]
pub struct Market {
    pub question: String,
    pub price_yes: f64, // fixed price for YES (0 < price < 1)
    pub resolved: bool,
    pub outcome: Option<Outcome>,
}

#[derive(Debug)]
pub struct Position {
    pub outcome: Outcome,
    pub shares: u64,
    pub cost: f64,
}

impl Market {
    /// Create a new binary market with a fixed YES price
    pub fn new(question: &str, price_yes: f64) -> Self {
        assert!(price_yes > 0.0 && price_yes < 1.0, "Price must be between 0 and 1");
        Self {
            question: question.to_string(),
            price_yes,
            resolved: false,
            outcome: None,
        }
    }

    /// Buy a position in the market
    pub fn buy(&self, outcome: Outcome, shares: u64) -> Position {
        let price = match outcome {
            Outcome::Yes => self.price_yes,
            Outcome::No => 1.0 - self.price_yes,
        };

        Position {
            outcome,
            shares,
            cost: price * shares as f64,
        }
    }

    /// Resolve the market with the winning outcome
    pub fn resolve(&mut self, outcome: Outcome) {
        self.resolved = true;
        self.outcome = Some(outcome);
    }
}

impl Position {
    /// Calculate payout after market is resolved
    pub fn payout(&self, market: &Market) -> f64 {
        if !market.resolved {
            return 0.0;
        }

        if Some(self.outcome) == market.outcome {
            self.shares as f64 // $1 per share if correct
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yes_payout() {
        let mut market = Market::new("Will it rain tomorrow?", 0.6);
        let pos = market.buy(Outcome::Yes, 10);

        market.resolve(Outcome::Yes);

        assert_eq!(pos.payout(&market), 10.0);
    }

    #[test]
    fn test_no_loses() {
        let mut market = Market::new("Will it rain tomorrow?", 0.6);
        let pos = market.buy(Outcome::No, 10);

        market.resolve(Outcome::Yes);

        assert_eq!(pos.payout(&market), 0.0);
    }

    #[test]
    fn test_no_wins() {
        let mut market = Market::new("Will it rain tomorrow?", 0.6);
        let pos = market.buy(Outcome::No, 5);

        market.resolve(Outcome::No);

        assert_eq!(pos.payout(&market), 5.0);
    }
}
