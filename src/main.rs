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

#[cfg(test)] 
pub mod testes {
  use crate::modules::{character::{
    Condition,
    Status,
    GenericCharacter
  }, actions::{Attacks, Defenses}};

  #[test]
  fn teste_atacar_basico() {
    let mut cavaleiro = GenericCharacter::new(200, 60, 60, "DarkKnight".to_string());

    // no ataque básico, o dano deve ser igual à força do personagem
    let dano = cavaleiro.basic_attack();

    assert_eq!(dano, cavaleiro.get_strength());
  }

  #[test]
  fn teste_atacar_especial() {
    let mut cavaleiro = GenericCharacter::new(200, 60, 60, "DarkKnight".to_string());

    // no ataque especial rola um d6 e multiplica pela vida máxima
    let min = cavaleiro.get_strength() + cavaleiro.get_max_health() / 10; // o mínimo é se a rolagem do d6 = 1
    let max = cavaleiro.get_strength() + cavaleiro.get_max_health() * 6 / 10; // o máximo será quando a rolagem do d6 = 6
    
    let dano = cavaleiro.special_attack();

    assert!(min <= dano && dano <= max);
  }

  #[test]
  fn teste_lidar_com_ataque() {
    // vida, força, defesa, nome
    let mut cavaleiro = GenericCharacter::new(200, 60, 60, "DarkKnight".to_string());
    let mut inimigo = GenericCharacter::new(210, 40, 80, "Player2".to_string());

    let dano = cavaleiro.basic_attack();

    inimigo.handle_damage(dano);

    let vida_inimigo = inimigo.get_health();

    assert_eq!(vida_inimigo, (inimigo.get_max_health()-&dano))
  }

  #[test]
  fn teste_bloquear() {
    let mut cavaleiro = GenericCharacter::new(200, 60, 60, "DarkKnight".to_string());
    let mut inimigo = GenericCharacter::new(210, 40, 80, "Player2".to_string());

    let dano_bruto = cavaleiro.basic_attack();

    let dano_bloqueado = inimigo.block();

    let dano_recebido = if dano_bloqueado > dano_bruto {
      0
    } else {
      dano_bruto - dano_bloqueado
    };

    inimigo.handle_damage(dano_recebido);

    // verificar que o inimigo só recebeu o que não bloqueou
    assert_eq!(inimigo.get_health(), inimigo.get_max_health()-&dano_recebido);
  }

  #[test]
  fn teste_esquivar() {
    let mut cavaleiro = GenericCharacter::new(200, 60, 60, "DarkKnight".to_string());
    let mut inimigo = GenericCharacter::new(210, 40, 80, "Player2".to_string());

    let dano_bruto = cavaleiro.special_attack();

    let inimigo_esquivou = inimigo.dodge();

    let dano_recebido = if inimigo_esquivou {
      0
    } else {
      dano_bruto
    };

    inimigo.handle_damage(dano_recebido);

    assert_eq!(inimigo.get_health(), inimigo.get_max_health()-&dano_recebido);
  }
}

