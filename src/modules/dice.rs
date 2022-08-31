use rand::Rng;

pub struct GenericDice { 
	// TODO: como colocar os valores de range num enum ou algo assim? pra já tê-los definidos
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
		rng.gen_range(0..=self.get_range())
	}
}