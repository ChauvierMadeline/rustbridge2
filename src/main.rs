use clap :: Parser;

#[derive(Debug, Parser)]
/// A little CLI that outpouts  message
struct Arguments{
    //the thing to say
    message : String,
}

fn main() {
    let args: Arguments = Arguments::parse();
    //dbg!(args.message); débug
    println!("{}", args.message);
    for i in 0..args.message.len()/2{
        for _ in 0..args.message.len()/2+i{
            print!(" ");
        }
        println!("\\");
    }
    for _ in 0.. args.message.len(){
        print!(" ")
    }
    println!(">O_O<");
}
