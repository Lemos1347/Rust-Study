Em Rust, um módulo é uma maneira de organizar o código em grupos lógicos e controlar o acesso a ele. Um módulo é representado por um arquivo com o sufixo `.rs` ou por uma pasta com um arquivo `mod.rs` dentro.

Os módulos são usados para:

-   Agrupar códigos relacionados em um único lugar;
-   Controlar a visibilidade do código, tornando determinadas partes públicas ou privadas;
-   Evitar conflitos de nomes de funções, tipos, variáveis e outros identificadores;
-   Fornecer uma estrutura de namespaces para o código, o que ajuda na organização e manutenção do mesmo.

Você pode usar a palavra-chave `mod` para criar um módulo, e a sintaxe `use` para importar funções e tipos de um módulo em outro lugar no código. Além disso, você pode anexar outros módulos dentro de um módulo principal, criando uma árvore de módulos que ajuda a organizar o código em um nível hierárquico.

## Como a palavra-chave mod funciona no compilador
Aqui, fornecemos uma referência rápida sobre como os módulos, caminhos, a palavra-chave "use" e a palavra-chave "pub" funcionam no compilador, e como a maioria dos desenvolvedores organiza seu código. Vamos passar por exemplos de cada uma dessas regras ao longo deste capítulo, mas este é um ótimo lugar para consultar como lembrete sobre como os módulos funcionam.
- Comece a partir da raiz do projeto: Quando você compila um projeto, o compilador primeiro procura no arquivo raiz do projeto (normalmente src/lib.rs para uma biblioteca ou src/main.rs para um projeto binário) por código para compilar. 
- Declarando módulos: No arquivo raiz da caixa, você pode declarar novos módulos; digamos, você declara um módulo "jardim" com `mod garden`. O compilador procurará pelo código do módulo nestes lugares:
	- Inline, dentro de chaves que substituem o ponto e vírgula após mod garden
	- No arquivo src/garden.rs
	- No arquivo src/garden/mod.rs
- Essa lógica pode ser seguida para que sub modules possam ser criados. Lembrando que por padrão modules são privados.