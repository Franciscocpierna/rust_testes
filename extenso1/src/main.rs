extern crate native_windows_gui as nwg; // Importa a biblioteca nwg
extern crate native_windows_derive as nwd; // Importa a biblioteca nwd
extern crate printpdf; // Importa a biblioteca printpdf

use nwg::NativeUi; // Importa o trait NativeUi
use std::str::FromStr; // Importa o trait FromStr
use printpdf::*; // Importa a biblioteca printpdf
use std::fs::File; // Importa o módulo File
use std::io::BufWriter; // Importa o módulo BufWriter
use std::process::Command; // Importa o módulo Command
use std::panic; // Importa o módulo panic
use std::io::Write; // Importa o módulo Write

// Função para gerar um PDF com o texto fornecido
fn gerar_pdf(texto: &str, caminho: &str) { 
    let (doc, page1, layer1) = PdfDocument::new("Documento", Mm(210.0), Mm(297.0), "Layer 1"); // Cria um novo documento PDF
    let current_layer = doc.get_page(page1).get_layer(layer1); // Obtém a camada atual do documento

    let font = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap(); // Adiciona uma fonte embutida ao documento
    current_layer.use_text(texto, 10.0, Mm(10.0), Mm(287.0), &font); // Usa o texto fornecido na camada atual
 //current_layer.use_text(texto, 48.0, Mm(10.0), Mm(287.0), &font);
    let file = File::create(caminho).unwrap(); // Cria um arquivo no caminho especificado
    let mut buf_writer = BufWriter::new(file); // Cria um buffer de escrita para o arquivo
    doc.save(&mut buf_writer).unwrap(); // Salva o documento PDF no buffer
}

// Função para converter um número em palavras
fn numero_por_extenso(n: u64) -> String {
    let unidades = ["", "um", "dois", "três", "quatro", "cinco", "seis", "sete", "oito", "nove"]; // Unidades
    let dezenas = ["", "dez", "vinte", "trinta", "quarenta", "cinquenta", "sessenta", "setenta", "oitenta", "noventa"]; // Dezenas
    let especiais = ["dez", "onze", "doze", "treze", "quatorze", "quinze", "dezesseis", "dezessete", "dezoito", "dezenove"]; // Números especiais
    let centenas = ["", "cem", "duzentos", "trezentos", "quatrocentos", "quinhentos", "seiscentos", "setecentos", "oitocentos", "novecentos"]; // Centenas

    if n == 0 {
        return "zero".to_string(); // Retorna "zero" se o número for 0
    }

    let mut partes = Vec::new(); // Vetor para armazenar as partes do número por extenso

    let mut n = n; // Variável mutável para o número
    if n >= 1_000_000_000_000 {
        let trilhoes = n / 1_000_000_000_000; // Calcula trilhões
        partes.push(format!("{} trilhão{}", numero_por_extenso(trilhoes), if trilhoes > 1 { "es" } else { "" })); // Adiciona trilhões ao vetor
        n %= 1_000_000_000_000; // Atualiza o valor de n
    }

    if n >= 1_000_000_000 {
        let bilhoes = n / 1_000_000_000; // Calcula bilhões
        partes.push(format!("{} bilhão{}", numero_por_extenso(bilhoes), if bilhoes > 1 { "es" } else { "" })); // Adiciona bilhões ao vetor
        n %= 1_000_000_000; // Atualiza o valor de n
    }

    if n >= 1_000_000 {
        let milhoes = n / 1_000_000; // Calcula milhões
        partes.push(format!("{} milhão{}", numero_por_extenso(milhoes), if milhoes > 1 { "es" } else { "" })); // Adiciona milhões ao vetor
        n %= 1_000_000; // Atualiza o valor de n
    }

    if n >= 1_000 {
        let milhares = n / 1_000; // Calcula milhares
        partes.push(format!("{} mil", numero_por_extenso(milhares))); // Adiciona milhares ao vetor
        n %= 1_000; // Atualiza o valor de n
    }

    if n >= 100 {
        let centena = n / 100; // Calcula centenas
        if centena == 1 && n % 100 != 0 {
            partes.push("cento".to_string()); // Adiciona "cento" ao vetor se for 100 e houver mais dígitos
        } else {
            partes.push(centenas[centena as usize].to_string()); // Adiciona a centena ao vetor
        }
        n %= 100; // Atualiza o valor de n
    }

    if n >= 20 {
        let dezena = n / 10; // Calcula dezenas
        partes.push(dezenas[dezena as usize].to_string()); // Adiciona a dezena ao vetor
        n %= 10; // Atualiza o valor de n
    }

    if n >= 10 {
        partes.push(especiais[(n - 10) as usize].to_string()); // Adiciona números especiais ao vetor
        n = 0; // Atualiza o valor de n
    }

    if n > 0 {
        partes.push(unidades[n as usize].to_string()); // Adiciona unidades ao vetor
    }
   // dbg!(&partes); // Debug
    partes.join(" ") // Junta as partes em uma única string
}

// Função para converter um número decimal em palavras
fn numero_por_extenso_completo(valor: f64) -> String {
    let inteiro = valor.trunc() as u64; // Parte inteira do valor
    let decimal = ((valor.fract() * 100.0).round()) as u64; // Parte decimal do valor

    let mut extenso = numero_por_extenso(inteiro); // Converte a parte inteira em palavras
    if decimal > 0 {
        extenso.push_str(" e "); // Adiciona "e" se houver parte decimal
        extenso.push_str(&numero_por_extenso(decimal)); // Converte a parte decimal em palavras
        extenso.push_str(" centavos"); // Adiciona "centavos"
    }

    extenso // Retorna o valor por extenso
}

#[derive(Default, nwd::NwgUi)]
pub struct App {
    // Janela principal do aplicativo (300, 250), position: (300, 300)
    #[nwg_control(size: (650, 500), position: (300, 300), title: "Conversor de Valores")]
    #[nwg_events(OnWindowClose: [nwg::stop_thread_dispatch()])]
    window: nwg::Window,

    // Campo de entrada de texto para o usuário digitar o valor
    #[nwg_control(size: (280, 25), position: (10, 10), placeholder_text: Some("Digite o valor"))]
    input: nwg::TextInput, // Campo de entrada de texto para o usuário digitar o valor

    // Botão para salvar em PDF
    #[nwg_control(size: (280, 25), position: (10, 50), text: "Salvar em PDF")]
    #[nwg_events(OnButtonClick: [App::salvar_pdf])]
    pdf_button: nwg::Button, // Botão para salvar em PDF

    // Botão para imprimir
    #[nwg_control(size: (280, 25), position: (10, 90), text: "Imprimir")]
    #[nwg_events(OnButtonClick: [App::imprimir_valor])]
    print_button: nwg::Button, // Botão para imprimir

    // Botão para exibir na tela
    #[nwg_control(size: (280, 25), position: (10, 130), text: "Exibir na Tela")]
    #[nwg_events(OnButtonClick: [App::exibir_na_tela])]
    display_button: nwg::Button, // Botão para exibir na tela

    // Campo de saída de texto para exibir o valor por extenso (280, 25), position: (10, 170)
    #[nwg_control(size: (620, 50), position: (10, 170), readonly: true)]
    output: nwg::TextInput, // Campo de saída de texto para exibir o valor por extenso
}

impl App {
    // Função que é chamada quando o botão "Salvar em PDF" é clicado
    fn salvar_pdf(&self) {
        let valor_str = self.input.text().replace(",", "."); // Obtém o texto do campo de entrada e substitui vírgulas por pontos
        match f64::from_str(&valor_str) {
            Ok(valor) => {
                let valor_extenso = numero_por_extenso_completo(valor); // Converte o valor em palavras
                self.output.set_text(&valor_extenso); // Define o texto do campo de saída
                let caminho_pdf = "valor_extenso.pdf"; // Caminho do arquivo PDF
                gerar_pdf(&valor_extenso, caminho_pdf); // Gera o PDF
                println!("PDF salvo como {}", caminho_pdf); // Imprime uma mensagem no console
                // Abre o arquivo PDF com o visualizador padrão do sistema
                Command::new("cmd")
                    .args(&["/C", "start", caminho_pdf])
                    .spawn()
                    .expect("Falha ao abrir o PDF");
            }
            Err(_) => {
                self.output.set_text("Valor inválido"); // Define o texto do campo de saída como "Valor inválido" se houver um erro
            }
        }
    }

    // Função que é chamada quando o botão "Imprimir" é clicado
    fn imprimir_valor(&self) {
        let valor_str = self.input.text().replace(",", "."); // Obtém o texto do campo de entrada e substitui vírgulas por pontos
        match f64::from_str(&valor_str) {
            Ok(valor) => {
                let valor_extenso = numero_por_extenso_completo(valor); // Converte o valor em palavras
                self.output.set_text(&valor_extenso); // Define o texto do campo de saída
                // Aqui você pode adicionar a lógica para imprimir o valor na impressora
                // No Windows, você pode usar a API de impressão do Windows para isso
                let mut file = File::create("print.txt").expect("Falha ao criar o arquivo de impressão"); // Cria um arquivo de impressão
                writeln!(file, "{}", valor_extenso).expect("Falha ao escrever no arquivo de impressão"); // Escreve o valor por extenso no arquivo de impressão
                Command::new("powershell")
                    .args(&["-Command", "Start-Process", "powershell", "-ArgumentList", "'-NoProfile -Command \"Get-Content print.txt | Out-Printer\"'", "-NoNewWindow"])
                    .spawn()
                    .expect("Falha ao imprimir o arquivo"); // Imprime o arquivo usando PowerShell
                println!("Imprimindo: {}", valor_extenso); // Imprime uma mensagem no console
            }
            Err(_) => {
                self.output.set_text("Valor inválido"); // Define o texto do campo de saída como "Valor inválido" se houver um erro
            }
        }
    }

    // Função que é chamada quando o botão "Exibir na Tela" é clicado
    fn exibir_na_tela(&self) {
        let valor_str = self.input.text().replace(",", "."); // Obtém o texto do campo de entrada e substitui vírgulas por pontos
        match f64::from_str(&valor_str) {
            Ok(valor) => {
                let valor_extenso = numero_por_extenso_completo(valor); // Converte o valor em palavras
                self.output.set_text(&valor_extenso); // Define o texto do campo de saída
                // Exibe o valor por extenso na tela
                nwg::simple_message("Valor por Extenso", &valor_extenso); // Exibe uma mensagem simples na tela
            }
            Err(_) => {
                self.output.set_text("Valor inválido"); // Define o texto do campo de saída como "Valor inválido" se houver um erro
            }
        }
    }
}

fn main() {
    // Configura um hook de pânico personalizado para exibir mensagens de erro na tela
    panic::set_hook(Box::new(|info| {
        let msg = if let Some(s) = info.payload().downcast_ref::<&str>() {
            s.to_string() // Obtém a mensagem de erro
        } else {
            "Erro desconhecido".to_string() // Define uma mensagem de erro padrão
        };

        if let Some(location) = info.location() {
            let location_msg = format!("Erro em {}:{}", location.file(), location.line()); // Obtém a localização do erro
            nwg::simple_message("Erro", &format!("{}\n{}", msg, location_msg)); // Exibe a mensagem de erro e a localização na tela
        } else {
            nwg::simple_message("Erro", &msg); // Exibe a mensagem de erro na tela
        }
    }));

    // Inicializa a biblioteca Native Windows GUI
    nwg::init().expect("Falha ao inicializar o Native Windows GUI");
    // Constrói a interface gráfica do aplicativo
    let _app = App::build_ui(Default::default()).expect("Falha ao construir a UI");

    // Inicia o loop de eventos do aplicativo
    nwg::dispatch_thread_events();
}