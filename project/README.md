# novadojo

## :computer: Instalação do Rust

  - Linux e MacOS: https://www.rust-lang.org/tools/install
  - Windows: https://forge.rust-lang.org/infra/other-installation-methods.html
  - Com docker: 
    ```
    alias rustc='docker run -it --rm  -w /project -v $(pwd):/project rust:1-alpine rustc'
    alias cargo='docker run -it --rm  -w /project -v $(pwd):/project rust:1-alpine cargo'
    alias rustp='docker run -it --rm  -w /project -v $(pwd):/project rust:1-alpine rustp'
    ```


## :ninja: Formato 

  - Dojo Randori


## :clipboard: Processo

  - Vamos criar um repositório para o Dojo
  - Antes de iniciar os turnos de desenvolvimento vamos explicar o desafio/kata
  - Em cada turno, a pessoa que for codar (piloto) executa os seguintes passos:
    - compartilha a tela
    - faz o pull do repositório,
    - desenvolve o código e ao final do turno faz um commit/push
  - Cada sala (se houver mais de uma) terá um sensei que organizará os turnos e a oderm das pessoas que vão participar
  - O tempo de caada turno será estipulado no início do Dojo e vai depender da quantidade de participantes

## :link: Link úteis:

  - Site oficial do Rust: https://www.rust-lang.org/pt-BR
  - The Rust Programming Language handbook: https://doc.rust-lang.org/book/
  - Sobre Coding Dojo: https://pt.wikipedia.org/wiki/Coding_Dojo
