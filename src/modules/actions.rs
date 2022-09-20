use super::character::GenericCharacter;

pub trait Attacks{
  // TODO: passar inimigo
  fn basic_attack(&mut self) -> u16;

  fn special_attack(&mut self) -> u16;
}

pub trait Defenses {
  // esquiva
  fn dodge(&mut self) -> bool ;

  // bloqueio
  fn block(&mut self) -> u16;
}

// TODO: no futuro, implementar skills genéricas, porém não sei como fazer ainda
// pub struct Skill {
//   name: String,
//   effect_name: String, // TODO: criar enum de efeitos de skills (Stun, Poison, Cure)
//   effect_factor: i8,
// }