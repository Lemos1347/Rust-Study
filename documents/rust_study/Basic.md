# Declaração de variáveis

const -> Variáveis imutáveis que tem seus valores antes do runtime do código e podem ser criadas no escopo global.
let -> Variáveis imutáveis que possuem `mut` como opcional para que seu valor possa ser alterado ao longo do código e não podem ser criadas no escopo global do código. Lets podem ser criadas com o mesmo nome, porém, apenas o valor da última declaração é "considerado".

## Tipos
Números inteiros:
``` rust
let x: i32 = 42; // pode ser positivo ou negativo
let y: u8 = 0xff; // apenas positvo
```

Ponto flutuante:
```rust
let x: f64 = 3.14; // 64 bits aproximadamente 15 casas decimais
let y: f32 = 2.71828f32; // 32 bits aproximadamente 7 casas decimais
```

Caractere:
```rust 
let x: char = 'x'; // tipo de dados que representa um único caractere Unicode. Ele é imutável e ocupa apenas 4 bytes na pilha
```

Booleanos:
```rust
let x: bool = true;
let y: bool = false;
```

Tuplas:
```rust
let x: (i8, str) = (1, "hello");
let y: (i32, f64, char) = (10, 3.14, 'a');
```

Arrays:
```rust 
let x: [i8; 5] = [1, 2, 3, 4, 5];
let y: [char; 5] = ['h', 'e', 'l', 'l', 'o'];
```

Strings:
```rust
let x: str = "hello world"; // usado para representar strings de caracteres que são conhecidos no momento da compilação e não precisam ser modificados
let y: String = String::from("hello rust"); // usado para representar strings de caracteres que precisam ser modificadas durante a execução do programa
```

Strucs:
```rust
struct Point {
    x: i32,
    y: i32,
}

let origin = Point { x: 0, y: 0 };
```

Enums:
```rust
enum Color {
    Red,
    Green,
    Blue,
}

let color: Color = Color::Green;

// Em um enum não consigo fazer o print de seu valor, portando utilizamos da palavra chave "match" para que possamos fazer isso
enum Colors {
	Red(u8),
	Blue(u8),
	Yellow(u8),
}

let color2: Color = Colors::Red(8);

match color2 {
	Colors::Red(number) => println!("{}", number),
	Colors::Blue(number) => println!("{}", number),
	Colors::Yellow(number) => println!("{}", number),
}
```

# Palavras-chave

## For 
O loop `for` em Rust é usado para repetir uma determinada ação várias vezes. Ele pode ser usado para iterar sobre uma coleção de elementos, como uma lista ou um array, ou para contar de um número inicial a outro número final.
```rust
// For para um range em específico
for i in 0..10 {
    println!("{}", i);
}

// For para elementos dentro de um array
let fruits = vec!["apple", "banana", "cherry"];

for fruit in fruits {
    println!("I like {}!", fruit);
}
```

## While
O loop `while` em Rust é usado para repetir uma determinada ação enquanto uma condição é verdadeira. O loop continua executando enquanto a condição é verdadeira, e para assim que a condição é falsa.
```rust
let mut answer = String::new();

while answer != "yes" {
    println!("Do you want to continue?");
    std::io::stdin().read_line(&mut answer).unwrap();
    answer = answer.trim().to_lowercase();
}
```

## Loop
A palavra-chave `loop` em Rust é usada para criar um loop infinito. O loop infinito continuará a executar uma determinada ação indefinidamente, a menos que seja interrompido por uma instrução `break` ou `return`.
```rust
let mut count = 0;

loop {
    println!("{}", count);
    count += 1;
    if count == 5 {
        break;
    }
}
```

## Match 
A palavra-chave `match` é usada para fazer comparações "exaustivas" de valores em Rust. É semelhante a uma estrutura de seleção, como `switch` em outras linguagens de programação.

O `match` é usado para comparar um valor com vários possíveis valores (casos) e executar a ação adequada para o primeiro caso que corresponde. Cada caso é identificado por uma expressão, geralmente uma constante, e é seguido por uma sequência de comandos.
```rust
match VARIABLETOWATCH {
    CASE1 => EXPRESSION1,
    CASE2 => EXPRESSION2,
    ...
    CASE_N => EXPRESSION_N,
}
```

## As
É usado para renomear uma importação ou alias de tipo.
```rust
use std::fmt::Result as FmtResult;

fn main() {
    let result: FmtResult;
}
```

## Break
É usado para interromper o fluxo de controle atual (loop, match, etc.).
```rust
for i in 0..10 {
    if i == 5 {
        break;
    }
    println!("{}", i);
}
```

## Continue
É usado para pular para a próxima iteração de um loop.
```rust
for i in 0..10 {
    if i % 2 == 0 {
        continue;
    }
    println!("{}", i);
}
```

## Fn
É usado para declarar funções.
```rust
// para retornar um valor, a palavra "return" é opcional
fn sum(a: i32, b: i32) -> i32 {
    a + b;
}

// Ao adicionar a palavra-chave `pub` antes de `fn`, você indica que a função pode ser acessada de outros módulos também.
// No argumento de uma função, podemos tipá-la com "Option<>" caso seja opcional que exista um valor para aquela variável.
pub sub(a: i32, b: Option<i32>) -> i32 {
	a - b;
}

fn main() {
    let result = sum(3, 4);
}
```
[[Importar funções de outros arquivos]]
[[Functional Language Features - Iterators and Closures| Iterators e Closures]]

## Struct
m Rust, as classes são implementadas como "structs" e "methods". Uma "struct" é usada para representar uma entidade com vários campos ou atributos, enquanto os "methods" são funções que são associadas a uma struct. 
```rust
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn distance_from_origin(&self) -> f64 {
        let x = self.x as f64;
        let y = self.y as f64;
        (x.powi(2) + y.powi(2)).sqrt()
    }
}

fn main() {
    let p = Point::new(3, 4);
    println!("The distance from the origin is {}", p.distance_from_origin());

	// para instanciar uma struct sem uma implementação é dessa forma
	let instance = Point{ x: 3, y: 4};
}
```

## Crate
A palavra-chave `extern` é usada para declarar bibliotecas externas que serão usadas em um programa Rust. O uso da palavra-chave `extern` indica ao compilador que a biblioteca está disponível em algum outro lugar e não precisa ser fornecida pelo próprio código Rust.
```rust
extern crate libc;

use libc::c_int;

fn main() {
    let result: c_int;
}
```
[[Como criar uma crate]]
