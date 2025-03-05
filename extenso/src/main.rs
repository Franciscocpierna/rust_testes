extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwg::NativeUi;
use std::str::FromStr;

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
    #[nwg_control(size: (300, 150), position: (300, 300), title: "Conversor de Valores")]
    #[nwg_events(OnWindowClose: [nwg::stop_thread_dispatch()])]
    window: nwg::Window,

    #[nwg_control(size: (280, 25), position: (10, 10), placeholder_text: Some("Digite o valor"))]
    input: nwg::TextInput,

    #[nwg_control(size: (280, 25), position: (10, 50), text: "Converter")]
    #[nwg_events(OnButtonClick: [App::converter_valor])]
    button: nwg::Button,

    #[nwg_control(size: (280, 25), position: (10, 90), readonly: true)]
    output: nwg::TextInput,
}

impl App {
    fn converter_valor(&self) {
        let valor_str = self.input.text().replace(",", ".");
        match f64::from_str(&valor_str) {
            Ok(valor) => {
                let valor_extenso = numero_por_extenso_completo(valor);
                self.output.set_text(&valor_extenso);
            }
            Err(_) => {
                self.output.set_text("Valor inválido");
            }
        }
    }
}

fn main() {
    nwg::init().expect("Falha ao inicializar o Native Windows GUI");
    let _app = App::build_ui(Default::default()).expect("Falha ao construir a UI");
    nwg::dispatch_thread_events();
}