extern crate native_windows_gui as nwg;// Importa a biblioteca nwg
extern crate native_windows_derive as nwd;// Importa a biblioteca nwd

use nwg::NativeUi; // Importa o trait NativeUi
use std::str::FromStr; // Importa o trait FromStr

fn numero_por_extenso(n: u64) -> String {
    let unidades = ["", "um", "dois", "três", "quatro", "cinco", "seis", "sete", "oito", "nove"];
    let dezenas = ["", "dez", "vinte", "trinta", "quarenta", "cinquenta", "sessenta", "setenta", "oitenta", "noventa"];
    let centenas = ["", "cem", "duzentos", "trezentos", "quatrocentos", "quinhentos", "seiscentos", "setecentos", "oitocentos", "novecentos"];
    let especiais = ["dez", "onze", "doze", "treze", "quatorze", "quinze", "dezesseis", "dezessete", "dezoito", "dezenove"];

    if n == 0 {
        return "zero".to_string();
    }

    let mut partes = Vec::new();

    let mut n = n;
    if n >= 1_000_000_000_000 {
        let trilhoes = n / 1_000_000_000_000;
        partes.push(format!("{} trilhão{}", numero_por_extenso(trilhoes), if trilhoes > 1 { "es" } else { "" }));
        n %= 1_000_000_000_000;
    }

    if n >= 1_000_000_000 {
        let bilhoes = n / 1_000_000_000;
        partes.push(format!("{} bilhão{}", numero_por_extenso(bilhoes), if bilhoes > 1 { "es" } else { "" }));
        n %= 1_000_000_000;
    }

    if n >= 1_000_000 {
        let milhoes = n / 1_000_000;
        partes.push(format!("{} milhão{}", numero_por_extenso(milhoes), if milhoes > 1 { "es" } else { "" }));
        n %= 1_000_000;
    }

    if n >= 1_000 {
        let milhares = n / 1_000;
        partes.push(format!("{} mil", numero_por_extenso(milhares)));
        n %= 1_000;
    }

    if n >= 100 {
        let centena = n / 100;
        if centena == 1 && n % 100 != 0 {
            partes.push("cento".to_string());
        } else {
            partes.push(centenas[centena as usize].to_string());
        }
        n %= 100;
    }

    if n >= 20 {
        let dezena = n / 10;
        partes.push(dezenas[dezena as usize].to_string());
        n %= 10;
    }

    if n >= 10 {
        partes.push(especiais[(n - 10) as usize].to_string());
        n = 0;
    }

    if n > 0 {
        partes.push(unidades[n as usize].to_string());
    }
   // dbg!(&partes); // Debug
    partes.join(" ")
}

fn numero_por_extenso_completo(valor: f64) -> String {
    let parte_inteira = valor.trunc() as u64;
    let parte_decimal = ((valor.fract() * 100.0).round()) as u64;

    let mut resultado = numero_por_extenso(parte_inteira);
    if parte_inteira == 1 {
        resultado.push_str(" real");
    } else {
        resultado.push_str(" reais");
    }

    if parte_decimal > 0 {
        resultado.push_str(" e ");
        resultado.push_str(&numero_por_extenso(parte_decimal));
        resultado.push_str(" centavo");
        if parte_decimal > 1 {
            resultado.push_str("s");
        }
    }

    resultado
}

#[derive(Default, nwd::NwgUi)]
pub struct App {
   // Janela principal do aplicativo
   #[nwg_control(size: (300, 150), position: (300, 300), title: "Conversor de Valores")]
   #[nwg_events(OnWindowClose: [nwg::stop_thread_dispatch()])]
   window: nwg::Window,

   // Campo de entrada de texto para o usuário digitar o valor
   #[nwg_control(size: (280, 25), position: (10, 10), placeholder_text: Some("Digite o valor"))]
   input: nwg::TextInput,// Campo de entrada de texto para o usuário digitar o valor

   // Botão para acionar a conversão do valor
   #[nwg_control(size: (280, 25), position: (10, 50), text: "Converter")]
   #[nwg_events(OnButtonClick: [App::converter_valor])]
   button: nwg::Button,// Botão para acionar a conversão do valor

   // Campo de saída de texto para exibir o valor por extenso
   #[nwg_control(size: (280, 25), position: (10, 90), readonly: true)]
   output: nwg::TextInput,// Campo de saída de texto para exibir o valor por extenso
}

impl App {
    // Função que é chamada quando o botão "Converter" é clicado
    fn converter_valor(&self) {
        // Obtém o texto do campo de entrada e substitui a vírgula por ponto
        let valor_str = self.input.text().replace(",", ".");
        // Tenta converter o texto para um valor de ponto flutuante
        match f64::from_str(&valor_str) {
            Ok(valor) => {
                // Se a conversão for bem-sucedida, converte o valor para sua representação por extenso
                let valor_extenso = numero_por_extenso_completo(valor);
                // Define o texto do campo de saída com o valor por extenso
                self.output.set_text(&valor_extenso);
            }
            Err(_) => {
                // Se a conversão falhar, exibe uma mensagem de erro no campo de saída
                self.output.set_text("Valor inválido");
            }
        }
    }
}

fn main() {
    // Inicializa a biblioteca Native Windows GUI
    nwg::init().expect("Falha ao inicializar o Native Windows GUI");
    // Constrói a interface gráfica do aplicativo
    let _app = App::build_ui(Default::default()).expect("Falha ao construir a UI");
    // Inicia o loop de eventos do aplicativo
    nwg::dispatch_thread_events();
}