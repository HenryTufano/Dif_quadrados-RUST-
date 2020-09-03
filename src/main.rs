use std::io;
use std::num;
fn main() {
    let mut x=0;//faz a varivel ser alterada
    let mut y=0;
    let mut z=0;
    let mut w=0;
    let mut q=0;
    //let z=0;
    println!("Digite o valor desejado");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let n:i32 = guess
                    .trim()
                    .parse()
                    .expect("Opção incorreta");
    
        for i in 0..n+2{
            if i <=n{

               
                q=i*i;
                w=w+q;
                y =y+i;
                
                
            }else {
                println!("{}",y);
                z= y*y;
                println!("O quadrado da soma dos primeiros dez números naturais é: {}",z);
                println!("A soma dos quadrados dos primeiros dez números naturais é: {}",w);
                println!("A diferença entre o quadrado da soma e a soma dos quadrados dos primeiros números naturais é {}",z-w);
            }
        }

    
}