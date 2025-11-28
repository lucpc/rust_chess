# Projeto de Xadrez em Rust

Este é um projeto de xadrez completo desenvolvido em Rust, criado como uma tradução de um projeto Java. Ele implementa as regras padrão do xadrez e oferece duas maneiras de jogar: localmente em um único computador ou online contra outro jogador em uma máquina diferente.

## Funcionalidades

*   **Dois Modos de Jogo:** Jogue localmente ("hot-seat") ou online via rede (cliente/servidor).
*   **Interface Baseada em Texto:** Jogue uma partida de xadrez completa diretamente no seu terminal.
*   **Validação de Movimentos:** O sistema valida todos os movimentos de acordo com as regras do xadrez.
*   **Destaque de Movimentos (Modo Local):** Ao selecionar uma peça no modo local, o tabuleiro exibe todos os seus movimentos possíveis.
*   **Detecção de Xeque e Xeque-Mate:** O jogo avisa quando um rei está em xeque e encerra a partida quando ocorre um xeque-mate, declarando o vencedor.
*   **Movimentos Especiais:** Implementa regras como Roque (*Castling*) e *En Passant*.
*   **Visual com Emojis:** Utiliza caracteres Unicode (♔, ♛, ♜, ...) para uma representação visual agradável das peças.

## Pré-requisitos

Para compilar e executar este projeto, você precisará ter o **Rust** instalado em seu sistema. A instalação inclui o compilador (`rustc`) e o gerenciador de pacotes (`cargo`).

*   A maneira recomendada de instalar o Rust é através do `rustup`. Você pode encontrar as instruções no site oficial: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

Para jogar online, os jogadores precisarão de um cliente TCP simples, como `telnet` ou `netcat` (`nc`), que geralmente vêm pré-instalados em sistemas Linux e macOS.

## Como Jogar: Duas Opções

Este projeto oferece duas formas de jogar.

---

### Opção 1: Jogo Local (Dois Jogadores no Mesmo Computador)

Neste modo, dois jogadores se revezam no mesmo terminal. O jogo oferece um fluxo em duas etapas com destaque de movimentos possíveis.

#### Como Executar:

1.  Navegue até a pasta raiz do projeto.
2.  Execute o seguinte comando:
    ```bash
    cargo run
    ```
    *(Isso é um atalho para `cargo run --bin rust_chess`)*

#### Como Jogar:

1.  **Selecionar a Origem:** Quando solicitado (`Source:`), digite a coordenada da peça que você deseja mover (ex: `e2`) e pressione Enter.
2.  **Visualizar Movimentos:** A tela será atualizada, e todos os movimentos legais para a peça selecionada serão destacados com um fundo azul.
3.  **Selecionar o Destino:** Quando solicitado (`Target:`), digite a coordenada para onde você deseja mover a peça (ex: `e4`) e pressione Enter.
4.  O turno passará para o próximo jogador.

---

### Opção 2: Jogo Online (Dois Jogadores em Computadores Diferentes)

Neste modo, uma pessoa executa o programa do servidor, e dois jogadores se conectam a ele a partir de seus próprios terminais para jogar um contra o outro. O matchmaking é simples: o primeiro jogador a se conectar espera pelo segundo.

#### Passo a Passo:

**A) Para a pessoa que vai hospedar o jogo (o "Host"):**

1.  **Inicie o Servidor:** Na pasta raiz do projeto, execute o seguinte comando:
    ```bash
    cargo run --bin server
    ```
    Você verá a mensagem: `Chess server listening on 127.0.0.1:8080`. O servidor agora está pronto para aceitar conexões.
    *   **Nota:** Por padrão, o servidor só aceita conexões da sua própria máquina (`127.0.0.1`). Para permitir que jogadores de outras máquinas se conectem, veja a seção "Jogando Pela Internet" abaixo.

**B) Para os dois Jogadores:**

1.  **Abra seu terminal.**

2.  **Conecte-se ao Servidor:** Use o comando `telnet` ou `netcat` com o endereço IP e a porta do servidor. Se estiver jogando na mesma máquina que o servidor, o comando é:
    ```bash
    telnet 127.0.0.1 8080
    ```
    *Se estiver jogando em outra máquina, substitua `127.0.0.1` pelo endereço IP do computador host.*

3.  **Aguarde o Oponente:** O primeiro jogador a se conectar receberá a mensagem `MSG:Waiting for an opponent...`. Quando o segundo jogador se conectar, o jogo começará para ambos.

#### Como Jogar (Modo Online):

O fluxo de jogo é em **uma única etapa**:

1.  O terminal mostrará de quem é a vez e solicitará o seu movimento (ex: `MSG:Your turn. (e.g., e2e4)`).
2.  Você deve digitar o movimento completo, combinando a **origem** e o **destino** em uma única string de 4 caracteres (ex: `e2e4`) e pressionar Enter.
3.  O tabuleiro será atualizado para ambos os jogadores, e o turno passará para o seu oponente.

---

### Jogando Pela Internet (Avançado)

Para permitir que jogadores fora da sua rede local (em computadores diferentes) se conectem, o host do servidor precisa fazer o seguinte:

1.  **Alterar o Código do Servidor:**
    No arquivo `src/bin/server.rs`, altere a linha `TcpListener::bind` para ouvir em `0.0.0.0`, que significa "todos os endereços IP desta máquina":
    ```rust
    // Em src/bin/server.rs
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    ```
    Isso permitirá conexões tanto da sua rede local quanto da internet.

2.  **Encaminhamento de Porta (Port Forwarding):**
    O host precisará configurar seu roteador de internet para encaminhar o tráfego da porta `8080` para o endereço IP local do computador que está rodando o servidor. Este processo varia de roteador para roteador.

3.  **Compartilhar o IP Público:**
    O host precisará encontrar e compartilhar seu endereço IP público (você pode encontrá-lo pesquisando "what is my ip" no Google) com o outro jogador. O jogador então se conectará usando esse IP público: `telnet <IP_PUBLICO_DO_HOST> 8080`.