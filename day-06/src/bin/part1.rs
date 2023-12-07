use day_06::part1::process;
use tracing::{info, subscriber::set_global_default};
use tracing_subscriber::{layer::SubscriberExt, Registry};

fn main() -> std::io::Result<()> {
    // TODO check different assembly code for loop / while and for loop.
    // Each one is build on top of the other

    let subscriber = tracing_subscriber::fmt::layer().json();
    //let subscriber = tracing_subscriber::fmt().init();
    let registry = Registry::default().with(subscriber);
    set_global_default(registry).expect("Failed to set subscriber");

    let input = include_str!("../../data/part1.txt");
    let output = process(input);
    info!(output);

    // let mut input = String::new();
    // io::stdin().read_line(&mut input)?;
    // println!("{}", input);
    // let input_int = input
    //     .lines()
    //     .map(|line| line.parse::<u8>().unwrap())
    //     .collect::<Vec<u8>>();
    // let b = input_int.get(0).unwrap() + 1;
    // println!("{}", b);

    // let f1 = 0.1;
    // let f2 = 0.2;
    // let f3 = f1 + f2;
    // println!("{}", f3);

    //let i3 = 1_u64;

    // let a = Hello { a: 5_u8 };
    // let c = &a;

    // let b = hello(a);
    //let b = hello(a);
    //println!("a = {}, b = {}", a, b);

    Ok(())
}

// struct Hello {
//     a: u8,
// }

// fn hello(input: Hello) -> u8 {
//     input.a
// }
