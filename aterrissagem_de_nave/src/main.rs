// Um jogo com GGEZ: Yasmin Souza Camargo
use std::env;
use ggez;                                       // Importa a biblioteca ggez
use ggez::event;                                // Importa o módulo de eventos da ggez
use ggez::graphics;                             // Importa o módulo de gráficos da ggez
use ggez::input::keyboard::{self, KeyCode};     // Importa o módulo de input do teclado
use ggez::{Context, GameResult};                // Importa tipos e funções básicas da ggez
use ggez::nalgebra as na;                       // Importa a biblioteca de álgebra linear

// Constantes para dimensões 
const LARGURA_NAVE: f32 = 40.0;     // dimensões da nave 
const ALTURA_NAVE: f32 = 40.0;
const LARGURA_ALVO: f32 = 100.0;    // dimensões do alvo
const ALTURA_ALVO: f32 = 20.0;

// Configurações
const CONSUMO_COMBUSTIVEL: f32 = 10.0;  // Consumo de combustível por segundo
const POTENCIA_MOTOR: f32 = 10000.0;    // Fator de aceleração do motor
const VELOCIDADE_NAVE: f32 = 200.0;     // Velocidade de movimento lateral da nave
const VELOCIDADE_PERMITIDA_PARA_POUSAR: f32 = 20.0;

// Estrutura para armazenar o estado do jogo
struct EstadoJogo {
    combustivel: f32,
    gravidade: f32,
    peso_nave: f32,
    colidiu: bool,
    pousou: bool,
    velocidade_nave: na::Vector2<f32>,
    posicao_nave: na::Point2<f32>,
    posicao_alvo: na::Point2<f32>,
}

impl EstadoJogo {
    // Método para criar um novo estado inicial do jogo
    fn novo(ctx: &mut Context, combustivel_parametro: f32, peso_nave_parametro: f32, gravidade_parametro: f32) -> GameResult<EstadoJogo> {
        // Calcula as posições iniciais da nave e do alvo com base no tamanho da tela
        let largura_tela = graphics::drawable_size(ctx).0;
        let altura_tela = graphics::drawable_size(ctx).1;
        let inicio_x_nave = largura_tela * 0.5;
        let inicio_x_alvo = largura_tela * 0.5;
        let inicio_y_alvo = altura_tela - ALTURA_ALVO * 0.5 - 20.0;

        Ok(EstadoJogo {
            combustivel: combustivel_parametro,
            gravidade: gravidade_parametro,
            peso_nave: peso_nave_parametro,
            colidiu: false,
            pousou: false,
            velocidade_nave: na::Vector2::new(0.0, 0.0),
            posicao_nave: na::Point2::new(inicio_x_nave, ALTURA_NAVE * 0.5),
            posicao_alvo: na::Point2::new(inicio_x_alvo, inicio_y_alvo),
        })
    }

    // Método para atualizar o estado do jogo a cada frame
    fn atualizar(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ggez::timer::delta(ctx).as_secs_f32();

        // A aceleração da nave é devido à sua gravidade e seu peso
        if !self.pousou && !self.colidiu{
            self.velocidade_nave.y += self.gravidade * self.peso_nave * dt;
        }

        // Impulsiona a nave para cima se ainda tem combustivel disponivel
        if keyboard::is_key_pressed(ctx, KeyCode::Up) && self.combustivel > 0.0  && !self.pousou && !self.colidiu {
            self.velocidade_nave.y -= (POTENCIA_MOTOR / self.peso_nave)  * dt;  
            self.combustivel -= CONSUMO_COMBUSTIVEL * dt;   // toda vez que acelera o combustivel é consumido
        }

        // Movimento lateral da nave
        if keyboard::is_key_pressed(ctx, KeyCode::Left) && !self.pousou && !self.colidiu {
            self.posicao_nave.x -= VELOCIDADE_NAVE * dt;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Right) && !self.pousou && !self.colidiu {
            self.posicao_nave.x += VELOCIDADE_NAVE * dt;
        }

        // Atualiza a posição da nave de acordo com sua velocidade
        if !self.pousou && !self.colidiu{
            self.posicao_nave += self.velocidade_nave * dt;
        }

        // Verificar colisão com o chão
        if self.posicao_nave.y > graphics::drawable_size(ctx).1 - ALTURA_NAVE * 0.5 {
            self.posicao_nave.y = graphics::drawable_size(ctx).1 - ALTURA_NAVE * 0.5;
            self.velocidade_nave.y = 0.0;
            self.colidiu = true;
        }

        // Verificar colisão com o teto da tela
        if self.posicao_nave.y < ALTURA_NAVE * 0.5 {
            self.posicao_nave.y = ALTURA_NAVE * 0.5;
            self.velocidade_nave.y = 0.0;
            self.colidiu = true;
        }

        // Verificar colisão com os lados da tela
        let largura_tela = graphics::drawable_size(ctx).0;
        if self.posicao_nave.x < LARGURA_NAVE * 0.5 {
            self.posicao_nave.x = LARGURA_NAVE * 0.5;
            self.colidiu = true;
        }
        if self.posicao_nave.x > largura_tela - LARGURA_NAVE * 0.5 {
            self.posicao_nave.x = largura_tela - LARGURA_NAVE * 0.5;
            self.colidiu = true;
        }

        // Verifica se a nave pousou sobre o alvo com sucesso
        let margem_pouso_horizontal = 40.0; // configurações para poder pousar em cima do alvo
        let margem_pouso_vertical = 40.0;    

        if (self.posicao_nave.x - self.posicao_alvo.x).abs() < margem_pouso_horizontal      // tem que estar no alvo (largura x)
            && (self.posicao_nave.y - self.posicao_alvo.y).abs() < margem_pouso_vertical    // tem que ter chegado no alvo (altura y)
            && self.velocidade_nave.y.abs() < VELOCIDADE_PERMITIDA_PARA_POUSAR              // tem que ter uma velocidade adequada para pousar
            && !self.pousou  && !self.colidiu                                               // não pode ter colidido ou já ter pousado
        {
            // Se pousou corretamente sobre o alvo
            self.posicao_nave = na::Point2::new(self.posicao_alvo.x, self.posicao_alvo.y  - 30.0); // Colocar a nave exatamente sobre o alvo
            self.velocidade_nave.y = 0.0;   // Parar a velocidade 
            self.pousou = true;             // Indica que fez o pouso com sucesso
        }

        Ok(())
    }

    // Método para desenhar os elementos do jogo na tela
    fn desenhar(&mut self, ctx: &mut Context) -> GameResult {
        // Limpa a tela com uma cor baseada no estado do jogo
        if self.pousou{
            graphics::clear(ctx, graphics::Color::new(0.0, 1.0, 0.0, 1.0)); // Fundo verde quando pousou corretamente
        } else if self.colidiu {
            graphics::clear(ctx, graphics::Color::new(1.0, 0.0, 0.0, 1.0)); // Fundo vermelho quando pousou no chão ou teve colisão nos lados da tela
        } else {
            graphics::clear(ctx, graphics::WHITE); // Fundo branco padrão
        }

        // Desenha o alvo na tela
        let retangulo_alvo = graphics::Rect::new(-LARGURA_ALVO * 0.5, -ALTURA_ALVO * 0.5, LARGURA_ALVO, ALTURA_ALVO);
        let malha_alvo = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), retangulo_alvo, graphics::Color::new(1.0, 0.0, 1.0, 1.0))?;
        let mut parametro_desenho_alvo = graphics::DrawParam::default();
        parametro_desenho_alvo.dest = self.posicao_alvo.into();
        graphics::draw(ctx, &malha_alvo, parametro_desenho_alvo)?;

        // Desenha a nave na tela
        let retangulo_nave = graphics::Rect::new(-LARGURA_NAVE * 0.5, -ALTURA_NAVE * 0.5, LARGURA_NAVE, ALTURA_NAVE);
        let malha_nave = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), retangulo_nave, graphics::Color::new(0.5, 0.0, 0.5, 1.0))?;
        let mut parametro_desenho_nave = graphics::DrawParam::default();
        parametro_desenho_nave.dest = self.posicao_nave.into();
        graphics::draw(ctx, &malha_nave, parametro_desenho_nave)?;

        // Desenha o valor do combustível na tela
        let texto_combustivel = graphics::Text::new(format!("Combustível: {:.2}", self.combustivel));
        let destino_texto_combustivel = na::Point2::new(10.0, 10.0); // Posição na tela
        let mut texto_combustivel_param = graphics::DrawParam::default();
        texto_combustivel_param.dest = destino_texto_combustivel.into();
        texto_combustivel_param.color = graphics::Color::new(0.0, 0.0, 0.0, 1.0); // Cor preta
        graphics::draw(ctx, &texto_combustivel, texto_combustivel_param)?;

        // Desenha o valor da velocidade na tela
        let speed_text = graphics::Text::new(format!("Velocidade: {:.2}", self.velocidade_nave.y));
        let speed_text_dest = na::Point2::new(10.0, 30.0); // Posição na tela
        let mut speed_text_param = graphics::DrawParam::default();
        speed_text_param.dest = speed_text_dest.into();
        speed_text_param.color = graphics::Color::new(0.0, 0.0, 0.0, 1.0); // Cor preta
        graphics::draw(ctx, &speed_text, speed_text_param)?;

        // Atualiza a tela
        graphics::present(ctx)?;
        Ok(())
    }
}

// Implementação do evento do jogo
impl event::EventHandler for EstadoJogo {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.atualizar(ctx)?;   // Atualiza o estado do jogo
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.desenhar(ctx)?;    // Desenha os elementos na tela
        Ok(())
    }
}

// Função principal que inicia o jogo
fn main() -> GameResult {
    let args: Vec<String> = env::args().collect();// Coleta os argumentos passados na linha de comando e armazena em um vetor de strings

    // Declaração das variáveis que irão armazenar os valores personalizados
    let combustivel;
    let peso_nave;
    let gravidade;

    if args.len() != 4 {    // Verifica se o número de argumentos é diferente de 4
        // Mensagem de instrução para executar o programa com entradas personalizadas
        println!("\nPara personalizar entradas: cargo run <combustivel> <peso_nave> <gravidade>\n");

        // Valores padrão usados se não houver argumentos suficientes
        combustivel = 8.0;
        peso_nave = 10.0;
        gravidade = 10.0;
    } else {
        // Se houver argumentos suficientes, converte cada argumento para um float
        combustivel = match args[1].parse::<f32>() {
            Ok(value) => value,
            Err(_) => {
                println!("O primeiro campo (combustivel) deve ser um float.");
                return Ok(());
            }
        };
        peso_nave = match args[2].parse::<f32>() {
            Ok(value) => value,
            Err(_) => {
                println!("O segundo campo (peso_nave) deve ser um float.");
                return Ok(());
            }
        };
        gravidade = match args[3].parse::<f32>() {
            Ok(value) => value,
            Err(_) => {
                println!("O terceiro campo (gravidade) deve ser um float.");
                return Ok(());
            }
        };
    }
    let cb = ggez::ContextBuilder::new("trabalho_TECVII", "Yasmin");    // Cria um novo construtor 
    let (mut ctx, mut event_loop) = cb.build()?;       // Constrói o contexto e o loop de eventos
    graphics::set_window_title(&ctx, "trabalho_TECVII");     // Define o título da janela do jogo
    let mut estado = EstadoJogo::novo(&mut ctx, combustivel, peso_nave, gravidade)?;  // Cria um novo estado de jogo
    event::run(&mut ctx, &mut event_loop, &mut estado)   // Inicia o loop de eventos do jogo
}