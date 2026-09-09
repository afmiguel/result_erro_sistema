use std::fs;

fn ler_bilhete(caminho: &str) -> Result<String, String> {
    match fs::read_to_string(caminho) {
        Ok(texto) => Ok(texto),
        Err(erro) => Err(String::from("Não deu certo!")),
        //Err(erro) => Err(erro.to_string()),   // <- Aqui você traduz a msg do sistema
    }
}

fn main() {
    match ler_bilhete("bilhete.txt") {
        // Complete o código para exibir o erro ou o texto lido
    }
}
