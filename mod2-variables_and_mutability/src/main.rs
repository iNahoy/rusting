mod constants;
mod imutabilidade;
mod scopes;
mod shadowing;
mod type_aliases;
mod variaveis;

fn main() {
    // Executa a aula de variáveis
    variaveis::executar();

    // Executa a aula de imutabilidade
    imutabilidade::executar();

    // Executa a aula de shadowing
    shadowing::executar();

    // Executa a aula de scopes
    scopes::executar();

    // Executa a aula de constantes
    constants::executar();

    //Executa a aula de type aliases
    type_aliases::executar();
}
