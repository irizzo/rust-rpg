pub mod battle_actions {
	pub trait Attacks {
		// ataque simples
		fn basic_attack();

		// ataque especial
		fn special_attack();
	}

	pub trait Defense {
		// esquiva
		fn dodge();

		// bloqueio
		fn block();
	}
}
