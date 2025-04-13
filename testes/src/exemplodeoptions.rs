fn main() {
    // Exemplo 1: Verificar se um valor existe
    let valor: Option<i32> = Some(10);
    if let Some(v) = valor {
        println!("O valor é: {}", v);
    } else {
        println!("Nenhum valor encontrado.");
    }

    // Exemplo 2: Retorno de função com Option
    fn dividir(a: i32, b: i32) -> Option<i32> {
        if b != 0 {
            Some(a / b)
        } else {
            None
        }
    }
    match dividir(10, 2) {
        Some(resultado) => println!("Resultado da divisão: {}", resultado),
        None => println!("Divisão por zero!"),
    }

    // Exemplo 3: Usando unwrap_or para fornecer um valor padrão
    let valor_opcional: Option<&str> = None;
    let resultado = valor_opcional.unwrap_or("Valor padrão");
    println!("Resultado: {}", resultado);

    // Exemplo 4: Mapeando um valor dentro de Option
    let numero: Option<i32> = Some(5);
    let dobro = numero.map(|n| n * 2);
    println!("Dobro do número: {:?}", dobro);

    // Exemplo 5: Encadeando operações com and_then
    fn incrementar_se_par(n: i32) -> Option<i32> {
        if n % 2 == 0 {
            Some(n + 1)
        } else {
            None
        }
    }
    let resultado = Some(4).and_then(incrementar_se_par);
    println!("Resultado do incremento: {:?}", resultado);
}