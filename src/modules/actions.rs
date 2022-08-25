pub trait Attacks{
  fn basic_attack(&mut self);

  fn special_attack(&mut self);
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

pub trait Defense {
  // esquiva
  fn dodge();

  // bloqueio
  fn block();
}