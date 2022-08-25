use super::actions::{self, Attacks};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Status {
  Alive, 
  Dead,
  Disabled
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Condition {
  Well, // (100% a 40%)
  Hurt, // (39% a 10%)
  Unconscious // (9% a 0%)
}

#[derive(Debug)]
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
    pub fn new(h: u16, max_h: u16, s: u16, d: u16, st: Status, c: Condition, n: String) -> GenericCharacter {
    GenericCharacter {
      health: h,
      max_health: max_h,
      strength: s,
      defense: d,
      status: st,
      condition: c,
      name: n
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

  pub fn update_condition(&mut self) {
    let current_health_percent: u16 = (self.get_health() * 100) / self.get_max_health();

    match current_health_percent {
      0..=9 => self.condition = Condition::Unconscious,
      10..=39 => self.condition = Condition::Hurt,
      40..=100 => self.condition = Condition::Well,
      _ => self.condition = Condition::Well,
    }
  }
}
