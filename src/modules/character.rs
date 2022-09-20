use std::cmp::Ordering;
use super::actions::{Attacks, Defenses};
use super::dice::GenericDice;

#[derive(Debug, Copy, Clone)]
pub enum Status {
  Alive, 
  Dead,
  Disabled
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Condition {
  Well, // (100% a 40%)
  Hurt, // (39% a 10%)
  Unconscious // (9% a 0%)
}

#[derive(Debug, Clone)]
pub struct GenericCharacter {
  health: u16,
  max_health: u16,
  strength: u16,
  defense: u16,
  status: Status,
  condition: Condition,
  name: String
}

impl GenericCharacter {
  pub fn new(health: u16, strength: u16, defense: u16, name: String) -> GenericCharacter {
    GenericCharacter {
      health,
      max_health: health,
      strength,
      defense,
      status: Status::Alive,
      condition: Condition::Well,
      name,
    }
  }

  pub fn get_status(&self) -> Status {
    self.status
  }

  pub fn get_health(&self) -> u16 {
    self.health
  }

  pub fn get_max_health(&self) -> u16 {
    self.max_health
  }

  pub fn get_strength(&self) -> u16 {
    self.strength
  }

  pub fn get_defense(&self) -> u16 {
    self.defense
  }

  pub fn get_name(&self) -> String {
    self.name.clone()
  }

  pub fn get_condition(&self) -> Condition {
    self.condition
  }

  pub fn set_health(&mut self, new_health: u16) {
    self.health = new_health;
  }

  pub fn set_strength(&mut self, new_strength: u16) {
    self.strength = new_strength;
  }

  pub fn set_defense(&mut self, new_defense: u16) {
    self.defense = new_defense;
  }

  pub fn set_name(&mut self, new_name: String) {
    self.name = new_name;
  }

  pub fn set_status(&mut self, new_status: Status) {
    self.status = new_status;
  }

  pub fn update_status(&mut self) {
    if self.get_condition() == Condition::Unconscious {
      
      if self.get_health() == 0 {
        self.set_status(Status::Dead); // se está inconsciente e com vida = 0, está morto

      } else {
        self.set_status(Status::Disabled); // se está inconsciente e com vida != 0, está "desabilitado"
      }

    } else {
      self.set_status(Status::Alive) // de resto, está vivo
    }
  }

  pub fn set_condition(&mut self, new_condition: Condition) {
    self.condition = new_condition;
  }

  pub fn update_condition(&mut self, massive: bool) {
    if massive {
      self.set_condition(Condition::Unconscious);

    } else {
      let current_health_percent: u16 = (self.get_health() * 100) / self.get_max_health();

      match current_health_percent {
        // se a vida está entre 0 e 9 (inclusivo), o personagem está inconsciente
        0..=9 => self.set_condition(Condition::Unconscious),
        // se a vida está entre 10 e 39 (inclusivo), o personagem está machucado
        10..=39 => self.set_condition(Condition::Hurt),
        // de resto, o personagem está bem
        _ => self.set_condition(Condition::Well),
      }
    }
  }

  pub fn handleDamage(&mut self, rcvd_damage: u16) {
    // se o damage for maior que a vida restante, então a vida restante é 0.
    if rcvd_damage >= self.get_health() {
      self.set_health(0);
      
    } else {
      self.set_health(self.get_health() - rcvd_damage)
    }

    // se o damage for maior ou igual a 40% da vida máxima, o personagem fica Unconscious
    let is_damage_massive = rcvd_damage * 10 > (self.get_max_health() * 4);

    self.update_condition(is_damage_massive);

    self.update_status();
  }
  
}

impl PartialEq for GenericCharacter {
  fn eq(&self, other: &Self) -> bool {
    self.get_name() == other.get_name()
  }
}

impl PartialOrd for GenericCharacter{
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.max_health.partial_cmp(&other.max_health)
  }
  
  fn ge(&self, other: &Self) -> bool {
    self.max_health >= other.max_health
  }
}

impl Attacks for GenericCharacter {
  fn basic_attack(&mut self) -> u16 {
    let damage = self.get_strength();
    damage
  }

  fn special_attack(&mut self) -> u16 {
    let d6: GenericDice = GenericDice::new(6);
    let damage = self.get_strength() + (self.get_max_health() * d6.roll() as u16) / 10;
    print!("special attack damage");
    println!("{}", damage);
    damage
  }
}

impl Defenses for GenericCharacter {
  fn block(&mut self) -> u16 {
    let d20: GenericDice = GenericDice::new(20);
    let roll = d20.roll();

    let blocked_damage;

    if roll >= 10 {
      blocked_damage = self.get_defense() * roll as u16;
    } else {
      blocked_damage = 0;
    }

    blocked_damage
  }

  fn dodge(&mut self) -> bool {
    let d20: GenericDice = GenericDice::new(20);
    let roll = d20.roll();
    
    if roll >= d20.get_range()/2 {
      true
    } else {
      false
    }
  }
}
