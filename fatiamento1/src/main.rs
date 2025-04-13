fn main() {
    println!("Acessando de três em três letras");
    // Define a palavra "Antonia" e cria um iterador de caracteres
    let palavra = "Antonia";
    let mut iter = palavra.chars(); // Cria um iterador sobre os caracteres da string
    
    // Itera manualmente pulando de três em três
    for i in (0..palavra.len()).step_by(3) { // Itera com um passo de 3
        if let Some(c) = iter.nth(2) { // Pega o terceiro caractere a partir do atual
            println!("Índice {}: {}", i, c); // Imprime o índice inicial e o caractere correspondente
        }
    }
    println!("Fatiando a palavra em grupos de 3 caracteres");
    // Define a palavra "Antonia"
    let palavra = "Antonia";

    // Itera sobre a string em fatias de 3 caracteres
    let mut start = 0;
    let step = 3;
    while start < palavra.len() {
        let end = (start + step).min(palavra.len()); // Garante que o índice final não ultrapasse o tamanho da string
        let slice = &palavra[start..end]; // Faz o fatiamento da string
        println!("Fatia: {}", slice); // Imprime a fatia atual
        start += step; // Avança para a próxima fatia
    }
    println!("&palavra[0..3]: {}", &palavra[0..3]); // Fatiamento da string");
}

