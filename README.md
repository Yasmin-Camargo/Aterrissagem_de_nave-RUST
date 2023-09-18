# :flight_arrival: Aterrissagem_de_nave-RUST
Implementação de um jogo em RUST utilizando GGEZ, desenvolvido como trabalho avaliativo da disciplina de Tópicos Especiais em Computação VIII 

![image](https://github.com/Yasmin-Camargo/Aterrissagem_de_nave-RUST/assets/88253809/1297b770-c07c-49d1-a568-f1ed08a4bcad)


## :joystick: Como jogar
### Controles da Nave:
- Setas para cima: Acelerar o motor da nave para cima.
- Setas para a esquerda e direita: Mover a nave horizontalmente.

### Objetivo:
- Aterrar com segurança na área designada, representada pelo alvo no centro da tela.
- Evitar colisões com o chão e as laterais da tela.
- Manter a velocidade da nave abaixo do limite especificado para ganhar o jogo.

### Combustível:
- O combustível é limitado. Cada vez que o motor é acelerado, consome combustível.
- Nível de combustível é exibido na tela. Sem combustível, não é possível controlar a nave.

### Pousar com Sucesso:
- Para vencer o jogo, deve-se pousar na área designada com uma velocidade inferior ao limite crítico.

### Perda de Jogo:
- O jogo é perdido se colidir com o chão, as laterais da tela ou pousar com uma velocidade acima do limite crítico.

### Customização (Opcional):
- É possível personalizar o jogo fornecendo argumentos na linha de comando, como combustível, peso da nave e gravidade. Consulte "Executando o Jogo" para detalhes.

## :play_or_pause_button: Executando o Jogo
Para executar o jogo com valores personalizados (opcional):

**cargo run < combustivel > < peso_nave > < gravidade >**

- < combustivel >: Quantidade de combustível disponível para a nave (float)
- < peso_nave >: Peso da nave espacial (float)
- < gravidade >: Força de gravidade que afeta a nave (float)

Se nenhum argumento for fornecido, o jogo usará valores padrão.

## :dart: Objetivo do Jogo
O objetivo do jogo é pousar com segurança na área designada, mantendo a nave sob controle e gerenciando o combustível disponível.

#### Vitória
![image](https://github.com/Yasmin-Camargo/Aterrissagem_de_nave-RUST/assets/88253809/c9358deb-9f24-420a-a22c-bf8a355f6c4e)

#### Derrota
![image](https://github.com/Yasmin-Camargo/Aterrissagem_de_nave-RUST/assets/88253809/1b73e9a5-1047-455f-89b7-00d62b99001e)






