# :flight_arrival: Aterrissagem_de_nave-RUST
Implementação de um jogo em RUST utilizando GGEZ, desenvolvido como trabalho avaliativo da disciplina de Tópicos Especiais em Computação VIII 

![image](https://github.com/Yasmin-Camargo/Aterrissagem_de_nave-RUST/assets/88253809/b4b0736a-0d5a-4fea-a74b-47f5c440480d)

## :joystick: Como jogar
### Controles da Nave:
- Setas para cima: Acelerar o motor da nave para cima.
- Setas para a esquerda e direita: Mover a nave horizontalmente.

### Objetivo:
- Aterrar com segurança na área designada, representada pelo alvo no centro da tela.
- Evitar colisões com o chão e as laterais da tela.
- Manter a velocidade da nave abaixo do limite especificado para ganhar o jogo.

### Combustível:
- Seu combustível é limitado. Cada vez que você acelera o motor, consome combustível.
- Monitore o nível de combustível exibido na tela. Sem combustível, você não poderá controlar a nave.

### Pousar com Sucesso:
- Para vencer o jogo, você deve pousar na área designada com uma velocidade inferior ao limite crítico.

### Perda de Jogo:
- Você perde o jogo se colidir com o chão, as laterais da tela ou pousar com uma velocidade acima do limite crítico.

### Customização (Opcional):
- Você pode personalizar o jogo fornecendo argumentos na linha de comando, como combustível, peso da nave e gravidade. Consulte "Executando o Jogo" para detalhes.

## :play_or_pause_button: Executando o Jogo
Para executar o jogo com valores personalizados (opcional):

cargo run <combustivel> <peso_nave> <gravidade>

- < combustivel >: Quantidade de combustível disponível para a nave (float)
- < peso_nave >: Peso da nave espacial (float)
- < gravidade >: Força de gravidade que afeta a nave (float)

Se nenhum argumento for fornecido, o jogo usará valores padrão.

## :dart: Objetivo do Jogo
O objetivo do jogo é pousar com segurança na área designada, mantendo a nave sob controle e gerenciando o combustível disponível.

#### Vitória
![image](https://github.com/Yasmin-Camargo/Aterrissagem_de_nave-RUST/assets/88253809/91e3589d-1bb3-4985-94e8-06b9ff4676cf)

#### Derrota
![image](https://github.com/Yasmin-Camargo/Aterrissagem_de_nave-RUST/assets/88253809/4fa6bec9-8836-4ff0-b1da-2d9cfb35ae1f)





