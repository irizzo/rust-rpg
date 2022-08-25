pub mod b_acts {
	pub trait Attacks1 {
		// ataque simples
		fn basic_attack();

		// ataque especial
		fn special_attack();
	}

	pub trait Defense1 {
		// esquiva
		fn dodge();

		// bloqueio
		fn block();
	}
}