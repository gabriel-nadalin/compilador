# Trabalhos práticos - Compiladores
## Autores
Augusto dos Santos Gomes Vaz - 800268\
Gabriel Kusumota Nadalin - 819498

Este repositório reúne os trabalhos práticos realizados para a disciplina de compiladores.

## T1
Analisador léxico para a linguagem LA. Lê um arquivo fonte, identificando tokens e reportando erros léxicos, interrompendo a execução em caso de erro.

## T2
Analisador sintático para a linguagem LA. Utiliza os tokens de saída do analisador léxico para a análise sintática de um arquivo fonte,
segundo a [gramática da linguagem](gramatica.txt), também interrompendo sua execução em caso de erro léxico ou sintático.

## Como executar
1. Compilando: é necessário ter instalado o gerenciador de pacotes da linguagem Rust, o [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html).\
Com o Cargo instalado, basta executar o comando `cargo build` no diretório raiz do projeto. Com isso, todos os arquivos
fonte da pasta `src/bin` serão compilados em seus repectivos binários na pasta `target/debug`.
2. Executando: Os binários compilados podem ser executados em linha de comando, recebendo como argumentos os caminhos para o
arquivo de entrada e de saída respectivamente.
