[[Mod | O que é um mod e como funciona o compilador rust para ele?]]

Para importar uma função de outro arquivo, podemos fazer da seguinte forma:
Na root da pasta `src` criamos um arquivo `.rs` e nele criamos as funções publicas que desejamos exportar. No arquivo em que se deseja importar, usamos a palavra chave `mod` e em seguida o nome do arquivo que desejamos importar as funções. A partir isso, usamos as funções.
Exemplo:
```rust 
// no arquivo src/second.rs
pub fn my_pub_func(a: u32, b: u32) -> u32 {
	return a + b;
}

// no arquivo src/main.rs
mod second;

fn main() {

	let number: u32 = second::my_pub_func(10, 10);

	println!("{}", number);
}
```
