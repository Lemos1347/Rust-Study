# Rust-Study

## Objetivo

Esse repositório tem o objetivo de documentar meu processo de aprendizagem na linguagem `rust`. Todas as minhas anotações estão na pasta `documents/rust-study`. Elas estão nos arquivos `.md` e estão configurados para serem utilizados com o programa <a href="https://obsidian.md/">Obsidian</a>.  
Na pasta `src` contém arquivos de código `rust` para exemplificar algumas de minhas anotações ou para testes durante meu aprendizado.

### Organização 
Para simplificar os meus testes e melhorar a documentação, criei uma pasta bin em `/src/bin` para armazenar código que não possuem relações entre si. Para executar cada um deles, criei uma "alias study" em `/.cargo/config.toml`.  
Assim, para rodar um desses arquivos execute o seguinte comando no terminal na root do projeto (no comando substitua `--` pelo **nome** do arquivo em `/src/bin` que deseja rodar):
```shell
cargo study --
```
