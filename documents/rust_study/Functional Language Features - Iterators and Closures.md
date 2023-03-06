## Iteradores
iteradores são objetos que permitem percorrer uma coleção de elementos.
```rust
let numbers = vec![1, 2, 3, 4, 5];

// Criar um iterador a partir de uma coleção de números
let mut iterator = numbers.iter();

// Usando o método next para acessar cada item do iterador
while let Some(number) = iterator.next() {
    println!("Number: {}", number);
}
```

## Closures
Closures são funções anônimas que podem ser capturadas e armazenadas em variáveis, ser passadas como argumentos para outras funções ou retornadas como resultado de outras funções. Em Rust, closures são tipadas, o que significa que você precisa especificar os tipos de entrada e saída da closure.
```rust
let add = |x: i32, y: i32| x + y;
let result = add(5, 10);
println!("Result: {}", result);
```
