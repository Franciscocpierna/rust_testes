
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

fn main() {
    let valor_str = "1234,56"; // Exemplo de valor com vírgula
    let valor_str = valor_str.replace(",", ".");
    let valor = f64::from_str(&valor_str).expect("Valor inválido");
    let valor_extenso = numero_por_extenso_completo(valor);
    println!("Valor por extenso: {}", valor_extenso);
}