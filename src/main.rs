use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3{
        println!("Faltando parâmetros!");
        return;
    }
    
    let action = (&args[1]).as_str();
    println!("asd {}",action);
    if action != "feature" && action != "bugfix" {
        println!("Ação inválida");
        return;
    }

    let name = (&args[2]).as_str();
    if name == "" {
        println!("Nome da branch invalido");
        return;
    }

    let branch_name = format!("{}/{}", action, name);

    let checkout_dev = Command::new("cmd")
        .args(["/C", "git", "checkout", "dev"])
        .output()
        .expect("Falha ao executar o comando.");
    
    if !checkout_dev.status.success() {
        println!("Falha ao realizar checkout para dev");
        return;
    }
    
    let git_pull = Command::new("cmd")
    .args(["/C", "git", "pull"])
    .output()
    .expect("Falha ao executar o comando.");

    if !git_pull.status.success() {
        println!("Falha ao realizar git pull");
        return;
    }
    
    Command::new("cmd")
    .args(["/C", "git", "checkout", "-b", &branch_name])
    .output()
    .expect("Falha ao executar o comando.");

    if !git_pull.status.success() {
        println!("Falha ao criar uma nova branch");
        return;
    }
    
    println!("Branch criada com sucesso!")
}
