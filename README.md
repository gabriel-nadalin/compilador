# Trabalhos práticos - Compiladores
## Autores
Augusto dos Santos Gomes Vaz - 800268\
Gabrielly Castilho Guimarães - 805757\
Gabriel Kusumota Nadalin - 819498

Este repositório reúne os trabalhos práticos realizados para a disciplina de compiladores.

## T1
Analisador léxico para a linguagem LA. Lê um arquivo fonte, identificando tokens e reportando erros léxicos, interrompendo a execução em caso de erro.

## T2
Analisador sintático para a linguagem LA. Utiliza os tokens de saída do analisador léxico para a análise sintática de um arquivo fonte,
segundo a [gramática da linguagem](gramatica.txt), também interrompendo sua execução em caso de erro léxico ou sintático.

## T3
Primeira parte do analisador semântico para a linguagem LA. Utiliza a árvore sintática construida pelo analisador sintático para a análise semântica de um arquivo fonte, identificando erros semânticos sem interromper sua execução.

## T4
Segunda parte do analizador semântico para a linguagem LA. Usa da base do T3 e implementa novas detecções de erro semântico, como detecção de escopo para ponteiros, registros e funções, incompatibilidade para ponteiros, reais, inteiros, literais, logicos e registros.

## T5
Gerador de código em linguagem C. Realiza todas as análises e, no caso de ausência de erros léxicos, sintáticos ou semânticos, traduz o código fonte escrito na linguagem LA para código equivalente em C.

## Como executar
### Usando o docker
1. Garanta que você tenha o docker instalado e seu usuário faça parte do grupo docker (para não precisar rodar com `sudo`)
```sh
docker ps
```
2. Builde a imagem usando o script `build.sh`
```sh
./build.sh
```
3. Rode o container usando o script `run.sh`
```sh
./run.sh
```
4. Compile o programa
```sh
cd /home/compilador && cargo build
```
5. Para corrigir os trabalhos, use os scripts `corrigir_tX`, onde `X` é o trabalho a qual se quer corrigir. Por exemplo:
```sh
cd /home/corretor && ./corrigir_t4
```

### Usando Rust nativo
1. Compilando: é necessário ter instalado o gerenciador de pacotes da linguagem Rust, o [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html).\
Com o Cargo instalado, basta executar o comando `cargo build` no diretório raiz do projeto. Com isso, todos os arquivos
fonte do diretório `src/bin` serão compilados em seus repectivos binários no diretório `target/debug`.
2. Executando: Os binários compilados podem ser executados em linha de comando, recebendo como argumentos os caminhos para o
arquivo de entrada e de saída respectivamente.
