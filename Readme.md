Com certeza. Um bom arquivo `README.md` é essencial para qualquer projeto. Ele serve como o manual de instruções principal.

Aqui está um `README.md` completo e bem formatado para o seu projeto de xadrez em Rust. Ele explica o que é o projeto, como compilá-lo e, mais importante, como jogar. Você pode criar um arquivo chamado `README.md` na pasta raiz do seu projeto e colar o texto abaixo nele.

---

# Projeto de Xadrez em Rust (Versão Console)

Este é um projeto de xadrez completo, jogável via linha de comando, desenvolvido em Rust. Ele implementa as regras padrão do xadrez, incluindo detecção de xeque e xeque-mate, e foi criado como uma tradução de um projeto originalmente feito em Java.

## Funcionalidades

- **Interface Baseada em Texto:** Jogue uma partida de xadrez completa diretamente no seu terminal.
- **Modo para Dois Jogadores:** Projetado para dois jogadores no mesmo computador ("hot-seat").
- **Validação de Movimentos:** O sistema valida todos os movimentos de acordo com as regras do xadrez.
- **Destaque de Movimentos:** Ao selecionar uma peça, o tabuleiro exibe todos os seus movimentos possíveis, facilitando a jogada.
- **Detecção de Xeque e Xeque-Mate:** O jogo avisa quando um rei está em xeque e encerra a partida quando ocorre um xeque-mate, declarando o vencedor.
- **Captura de Peças:** Mantém e exibe uma lista de todas as peças capturadas por cada jogador.
- **Movimentos Especiais:** Implementa regras especiais como Roque (_Castling_) e _En Passant_.

## Pré-requisitos

Para compilar e executar este projeto, você precisará ter o **Rust** instalado em seu sistema. A instalação inclui o compilador (`rustc`) e o gerenciador de pacotes (`cargo`).

- A maneira recomendada de instalar o Rust é através do `rustup`. Você pode encontrar as instruções no site oficial: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

## Como Compilar e Executar

1.  **Clone o Repositório** (ou simplesmente descompacte os arquivos em uma pasta):

    ```bash
    git clone https://github.com/lucpc/rust_chess
    ```

2.  **Navegue até a Pasta do Projeto:**

    ```bash
    cd nome-da-pasta-do-projeto
    ```

3.  **Compile e Execute o Jogo:**
    Use o Cargo para compilar e rodar o projeto com um único comando. O Cargo irá baixar automaticamente as dependências (`colored`, `clearscreen`) na primeira vez.
    ```bash
    cargo run
    ```

O jogo será iniciado no seu terminal.

## Como Jogar

O jogo é controlado inteiramente por texto. Siga o fluxo abaixo para realizar suas jogadas.

### 1. Entendendo o Tabuleiro

O tabuleiro é exibido com as coordenadas padrão do xadrez:

- **Colunas:** `a` até `h`
- **Linhas:** `1` até `8`

As peças brancas são representadas por letras maiúsculas (R, N, B, Q, K, P) na cor branca/cinza, e as peças pretas na cor amarela.

### 2. O Fluxo de uma Jogada

A cada turno, o jogo irá te guiar pelo processo de mover uma peça:

**Passo 1: Selecionar a Peça de Origem**

O terminal irá mostrar de quem é a vez e solicitar a posição da peça que você deseja mover:

```
Turn : 1
Waiting player: White

Source: _
```

Digite a coordenada da sua peça (ex: `e2` para mover o peão do rei branco na primeira jogada) e pressione **Enter**.

**Passo 2: Selecionar a Posição de Destino**

Após inserir a origem, a tela será limpa e o tabuleiro será redesenhado. Todos os movimentos legais para a peça que você selecionou serão destacados com um **fundo azul**.

O terminal então solicitará a posição de destino:

```
8 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜ 
7 ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟ 
6 - - - - - - - - 
5 - - - - - - - - 
4 - - - - - - - - 
3 - - - - - - - - 
2 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙ 
1 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖ 
  a b c d e f g h

Target: _
```

_(Neste exemplo, imagine que os movimentos possíveis da peça `e2` estão com fundo azul)_

Digite a coordenada para onde você deseja mover a peça (ex: `e4`) e pressione **Enter**.

**Passo 3: Fim do Turno**

Se o movimento for válido, a peça será movida, o tabuleiro será atualizado e o turno passará para o próximo jogador. O ciclo então se repete.

### 3. Notação de Posição

Use a notação de xadrez padrão (notação algébrica) para inserir as posições:

- Formato: `[letra da coluna][número da linha]`
- Exemplos: `a1`, `h8`, `f5`.
- Não use espaços e digite em letras minúsculas.

### 4. Movimentos Especiais

- **Roque (Castling):** Para fazer o roque, simplesmente mova o seu Rei duas casas para o lado desejado (ex: de `e1` para `g1`). O jogo moverá a Torre automaticamente para a posição correta.
- **En Passant:** A captura _en passant_ é realizada movendo seu peão na diagonal para a casa vazia atrás do peão adversário que acabou de avançar duas casas.
- **Promoção de Peão:** Quando um peão alcança a última fileira do tabuleiro, ele é promovido **automaticamente a uma Rainha (Queen)**.
