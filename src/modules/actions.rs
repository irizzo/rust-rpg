pub trait Attacks{
  fn basic_attack(&mut self) -> u16;

  fn special_attack(&mut self) -> u16;
}

// TODO: Efeitos que um ataque pode ter?
// pub enum AttackEffect {
//   Stun,
//   Poison
// }

// TODO: O que um ataque tem (dano, efeito, etc...)
// pub struct AttacksStruct {
//   damage: u16,
//   effect: String,
// }

// TODO: função para lidar com um ataque, quando receber um ataque
// pub fn handleAttackEffect(attack: AttacksStruct) {

// }

pub trait Defenses {
  // esquiva
  fn dodge(&mut self);

  // bloqueio
  fn block(&mut self);
}