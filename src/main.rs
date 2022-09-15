pub mod modules;

use modules::{character::{
  Condition,
  Status,
  GenericCharacter
}, actions::Attacks};

fn main() {
  let char1 = GenericCharacter::new(200, 60, 60, "DarkKnight".to_string());
  let char2 = GenericCharacter::new(210, 40, 80, "Player2".to_string());
}

// colocar os testes para testar as features da "engine"