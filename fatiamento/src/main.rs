fn main() {
 println!(" Usando chars() e iterando");
// Define a palavra "Antonia" e itera sobre cada caractere usando chars()
let palavra = "Antonia";
for c in palavra.chars() {
    println!("{}", c); // Imprime cada caractere da string
}

println!("2. Usando chars() e coletando em um vetor");
// Define a palavra "Antonia" e coleta os caracteres em um vetor
let palavra = "Antonia";
let letras: Vec<char> = palavra.chars().collect(); // Converte os caracteres para um vetor
println!("{:?}", letras); // Imprime o vetor de caracteres
for c in letras {
    println!("{}", c); // Imprime cada caractere do vetor
}
println!("3. Acessando por índices com char_indices()");
// Define a palavra "Antonia" e itera sobre os índices e caracteres
let palavra = "Antonia";
for (i, c) in palavra.char_indices() {
    println!("Índice {}: {}", i, c); // Imprime o índice e o caractere correspondente
}

println!("4. Convertendo para um vetor de bytes e acessando cada byte");
// Define a palavra "Antonia" e acessa os bytes da string
let palavra = "Antonia";
for b in palavra.as_bytes() {
    println!("{}", *b as char); // Converte cada byte para char e imprime
}

println!("5. Usando nth() em um iterador");
// Define a palavra "Antonia" e cria um iterador de caracteres
let palavra = "Antonia";
let mut iter = palavra.chars();
while let Some(c) = iter.next() { // Usa o método next() para pegar o próximo caractere
    println!("{}", c); // Imprime o caractere atual
}

println!("5. Usando nth() em um iterador");
// Define a palavra "Antonia" e cria um iterador de caracteres
let palavra = "Antonia";
let mut iter = palavra.chars(); // Cria um iterador sobre os caracteres da string

// Itera manualmente usando nth() explicitamente
for i in 0..palavra.len() { // Itera de 0 até o tamanho da string
    if let Some(c) = iter.nth(0) { // Pega o próximo caractere explicitamente (sempre o próximo, pois nth(0) avança o iterador)
        println!("Índice {}: {}", i, c); // Imprime o índice atual e o caractere correspondente
    }
}
}
