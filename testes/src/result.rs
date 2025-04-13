// Exemplo 1: Função simples com Result
fn dividir(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Divisão por zero"))
    } else {
        Ok(a / b)
    }
}

// Exemplo 2: Usando Result com match
fn exemplo_match() {
    match dividir(10, 2) {
        Ok(resultado) => println!("Resultado: {}", resultado),
        Err(erro) => println!("Erro: {}", erro),
    }
}

// Exemplo 3: Propagando erros com `?`
fn ler_arquivo(caminho: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(caminho)
}

// Exemplo 4: Result com tipos customizados
#[derive(Debug)]
enum MeuErro {
    Erro1,
    Erro2,
}

fn exemplo_customizado(flag: bool) -> Result<&'static str, MeuErro> {
    if flag {
        Ok("Tudo certo!")
    } else {
        Err(MeuErro::Erro1)
    }
}

// Exemplo 5: Convertendo Result para Option
fn result_para_option() {
    let resultado = dividir(10, 0).ok();
    println!("{:?}", resultado); // None
}

// Exemplo 6: Trabalhando com Result em iteradores
fn somar_valores(valores: Vec<&str>) -> Result<i32, std::num::ParseIntError> {
    valores.iter().map(|v| v.parse::<i32>()).sum()
}

// Exemplo 7: Combinando múltiplos Results
fn combinar_results() -> Result<i32, String> {
    let a = dividir(10, 2)?;
    let b = dividir(20, 4)?;
    Ok(a + b)
}

// Exemplo 8: Result com closures
fn usar_closure() {
    let resultado = (|| -> Result<i32, String> {
        let x = dividir(10, 2)?;
        let y = dividir(20, 0)?;
        Ok(x + y)
    })();

    println!("{:?}", resultado);
}

// Exemplo 9: Tratando erros com unwrap_or
fn tratar_com_default() {
    let resultado = dividir(10, 0).unwrap_or(-1);
    println!("Resultado com fallback: {}", resultado);
}

// Exemplo 10: Result com async/await
async fn operacao_assincrona() -> Result<String, reqwest::Error> {
    let resposta = reqwest::get("https://api.github.com").await?;
    let texto = resposta.text().await?;
    Ok(texto)
}

/*
Aqui está a explicação detalhada das linhas dos exemplos fornecidos:

### Exemplo 9: Tratando erros com `unwrap_or`
```rust
let resultado = dividir(10, 0).unwrap_or(-1);
```
1. **`dividir(10, 0)`**: Chama a função `dividir` com os valores `10` e `0`. Como o divisor é `0`, a função retornará um `Err(String::from("Divisão por zero"))`.

2. **`.unwrap_or(-1)`**: Trata o `Result` retornado. Se for `Ok`, retorna o valor contido no `Ok`. Se for `Err`, retorna o valor fornecido como fallback, neste caso `-1`.
   - Aqui, como o divisor é `0`, o resultado será `-1`.

```rust
println!("Resultado com fallback: {}", resultado);
```
3. **`println!`**: Imprime o valor de `resultado` no console. Como o fallback foi usado, a saída será: `Resultado com fallback: -1`.

---

### Exemplo 10: Result com `async/await`
```rust
async fn operacao_assincrona() -> Result<String, reqwest::Error> {
```
1. **`async fn`**: Define uma função assíncrona chamada `operacao_assincrona`. Funções assíncronas retornam um `Future` que precisa ser aguardado com `.await`.
2. **`Result<String, reqwest::Error>`**: A função retorna um `Result` que, em caso de sucesso, contém um `String`. Em caso de erro, contém um erro do tipo `reqwest::Error`.

```rust
let resposta = reqwest::get("https://api.github.com").await?;
```
3. **`reqwest::get`**: Faz uma requisição HTTP GET para o URL fornecido (`https://api.github.com`).
4. **`.await`**: Aguarda a conclusão da requisição assíncrona.
5. **`?`**: Propaga o erro, caso a requisição falhe. Se for bem-sucedida, o resultado é armazenado em `resposta`.

```rust
let texto = resposta.text().await?;
```
6. **`resposta.text()`**: Extrai o corpo da resposta HTTP como texto.
7. **`.await`**: Aguarda a conclusão da operação assíncrona.
8. **`?`**: Propaga o erro, caso a extração do texto falhe. Se for bem-sucedida, o texto é armazenado em `texto`.

```rust
Ok(texto)
```
9. **`Ok(texto)`**: Retorna o texto extraído como um `Ok`, indicando sucesso.

---

Esses exemplos mostram como tratar erros de forma segura e como usar operações assíncronas com `Result` em Rust.

*/