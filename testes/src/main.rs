use std::str::FromStr;

fn main() {
    let partes = vec!["um", "milhão", "duzentos", "mil", "trezentos", "e", "quarenta", "e", "cinco"];
    let resultado = partes.join(" ");
    println!("{}", resultado); // Saída: "um milhão duzentos mil trezentos e quarenta e cinco"
}