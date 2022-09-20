use rand::Rng;
use std::cmp::Ordering;

pub struct GenericDice {
	range: u8
}

impl GenericDice {
	pub fn new(dice_range: u8) -> GenericDice {
		GenericDice { 
			range: dice_range 
		}
	}

	pub fn get_range(&self) -> u8 {
		self.range
	}

	pub fn roll(&self) -> u8 {
		let mut rng = rand::thread_rng();
		rng.gen_range(1..=self.get_range())
	}
}

impl PartialEq for GenericDice {
	fn eq(&self, other: &Self) -> bool {
		self.get_range() == other.get_range()
	}
}

impl PartialOrd for GenericDice {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.range.partial_cmp(&other.range)
	}
}