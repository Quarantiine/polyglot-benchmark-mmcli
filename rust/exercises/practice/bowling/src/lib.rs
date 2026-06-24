#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    rolls: Vec<u16>,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self { rolls: Vec::with_capacity(21) }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        // Check if game is already complete
        if self.score().is_some() {
            return Err(Error::GameComplete);
        }

        // Validate pins for a frame
        let mut roll_index = 0;
        let mut frames = 0;
        for _ in 0..10 {
            if roll_index >= self.rolls.len() {
                break;
            }
            if self.rolls[roll_index] == 10 {
                roll_index += 1;
                frames += 1;
            } else if roll_index + 1 < self.rolls.len() {
                if self.rolls[roll_index] + self.rolls[roll_index + 1] > 10 {
                    return Err(Error::NotEnoughPinsLeft);
                }
                roll_index += 2;
                frames += 1;
            } else {
                break;
            }
        }

        // If we are in a frame, check if the roll is valid
        if frames < 10 {
            if roll_index < self.rolls.len() && self.rolls[roll_index] != 10 && (self.rolls[roll_index] + pins > 10) {
                return Err(Error::NotEnoughPinsLeft);
            }
        } else {
            // 10th frame logic
            let rolls_in_10th = self.rolls.len() - roll_index;
            if rolls_in_10th == 1 {
                if self.rolls[roll_index] != 10 && (self.rolls[roll_index] + pins > 10) {
                    return Err(Error::NotEnoughPinsLeft);
                }
            } else if rolls_in_10th == 2 {
                if self.rolls[roll_index] != 10 && (self.rolls[roll_index] + self.rolls[roll_index + 1] < 10) {
                    return Err(Error::GameComplete);
                }
            }
        }

        self.rolls.push(pins);
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        let mut score = 0;
        let mut roll_index = 0;
        for _frame in 0..10 {
            if roll_index >= self.rolls.len() {
                return None;
            }

            if self.rolls[roll_index] == 10 {
                // Strike
                if roll_index + 2 >= self.rolls.len() {
                    return None;
                }
                score += 10 + self.rolls[roll_index + 1] + self.rolls[roll_index + 2];
                roll_index += 1;
            } else if roll_index + 1 < self.rolls.len() && self.rolls[roll_index] + self.rolls[roll_index + 1] == 10 {
                // Spare
                if roll_index + 2 >= self.rolls.len() {
                    return None;
                }
                score += 10 + self.rolls[roll_index + 2];
                roll_index += 2;
            } else if roll_index + 1 < self.rolls.len() {
                // Open frame
                score += self.rolls[roll_index] + self.rolls[roll_index + 1];
                roll_index += 2;
            } else {
                return None;
            }
        }
        Some(score)
    }
}
