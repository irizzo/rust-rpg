#[cfg(test)] // só compila quando for rodar teste
pub mod tests {
	fn soma(x: i32, y: i32) -> i32 {
  	x + y
	}

	#[test]
	fn teste_soma() {
		assert_eq!(soma(2,2), 4);
	}

	#[test]
	fn teste_soma_2() {
		let mut somatório: i32 = 0;
		for i in 1..=100 {
			somatório = soma(somatório,i)
		}
		assert_eq!(somatório, 5050);
	}
	
}