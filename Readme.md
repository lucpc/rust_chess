# Projeto de Xadrez em Rust (Versão Online/Console)
Este é um projeto de xadrez completo, jogável via linha de comando, desenvolvido em Rust com suporte para partidas **Multiplayer Online** via TCP, mantendo a validação de regras, detecção de xeque/xeque-mate e movimentos especiais.

## Funcionalidades

  - **Multiplayer Online:** Arquitetura Cliente-Servidor que permite que dois jogadores joguem através da rede (local ou internet).
  - **Interface Baseada em Texto:** Jogue uma partida de xadrez completa diretamente no seu terminal com renderização colorida.
  - **Lobby de Espera:** O primeiro jogador aguarda em um "lobby" até que um oponente se conecte.
  - **Validação de Movimentos:** O servidor valida todos os movimentos de acordo com as regras do xadrez.
  - **Destaque de Movimentos:** Ao selecionar uma peça, o cliente exibe os movimentos possíveis (validado pelo servidor/regras locais).
  - **Detecção de Xeque e Xeque-Mate:** O jogo avisa quando um rei está em xeque e encerra a partida automaticamente.
  - **Movimentos Especiais:** Implementa regras como Roque (*Castling*), *En Passant* e Promoção de Peão (automática para Rainha).

## Pré-requisitos

Para compilar e executar este projeto, você precisará ter o **Rust** instalado em seu sistema.

  - Instalação recomendada via `rustup`: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

## Como Compilar e Executar

O jogo agora opera em dois modos: **Servidor** (que gerencia a partida) e **Cliente** (os jogadores).

### 1\. Preparação

Clone o repositório e entre na pasta:

```bash
git clone https://github.com/lucpc/rust_chess
cd rust_chess
```

### 2\. Iniciando o Servidor

O servidor é responsável por conectar os dois jogadores e validar o estado do jogo. Execute-o primeiro:

```bash
# Inicia o servidor na porta padrão (127.0.0.1:8080)
cargo run -- server

# OU especifique um IP e Porta personalizados
cargo run -- server 0.0.0.0:8080
```

*Nota: Se você quiser jogar com alguém fora da sua rede local, certifique-se de que a porta escolhida esteja aberta no seu roteador/firewall.*

### 3\. Conectando os Jogadores (Clientes)

Você precisará de dois terminais (janelas) adicionais para simular dois jogadores, ou dois computadores diferentes.

**Jogador 1 (Entra na fila e aguarda):**

```bash
# Conecta ao servidor local (padrão)
cargo run -- client

# OU conecta a um IP específico (ex: IP do amigo)
cargo run -- client 192.168.0.10:8080
```

*O Jogador 1 verá uma mensagem "Buscando adversário..." com uma animação.*

**Jogador 2 (Conecta e inicia a partida):**
Execute o mesmo comando em outro terminal/computador:

```bash
cargo run -- client
```

Assim que o segundo jogador conectar, o servidor iniciará a partida e atribuirá as cores (Branco e Preto) automaticamente.

## Como Jogar

O jogo é controlado via texto. Siga o fluxo indicado no terminal:

1.  **Sua Vez:** O jogo avisará `YOUR TURN (Color)!`.
2.  **Origem:** Digite a coordenada da peça que deseja mover (ex: `e2`) e pressione **Enter**.
3.  **Destino:** Digite a coordenada de destino (ex: `e4`) e pressione **Enter**.
4.  **Aguarde:** Enquanto o oponente joga, você verá a mensagem `Waiting for opponent...`.

### Notação e Regras

  - **Coordenadas:** Use o formato algébrico padrão (`a1` até `h8`).
  - **Roque:** Mova o Rei duas casas para o lado (ex: `e1` para `g1`).
  - **En Passant:** Mova o peão para a casa vazia atrás do peão adversário capturado.
  - **Vitória:** O jogo detecta automaticamente o Xeque-mate e declara o vencedor, encerrando a conexão.

## Estrutura do Projeto

  - **`server.rs`:** Gerencia conexões TCP e o estado da partida (`ChessMatch`).
  - **`client.rs`:** Interface do usuário, envia comandos e renderiza o tabuleiro recebido do servidor.
  - **`network.rs`:** Define o protocolo de comunicação (mensagens JSON) entre cliente e servidor.
  - **`chess/`:** Lógica central do xadrez (tabuleiro, peças, regras).

## Dependências

O projeto utiliza as seguintes *crates*:

  - `tokio`: Para rede assíncrona (TCP).
  - `serde` / `serde_json`: Para serialização das mensagens de rede.
  - `colored`: Para colorir o terminal.
  - `clearscreen`: Para limpar a tela entre os turnos.
