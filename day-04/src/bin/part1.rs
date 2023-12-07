use day_04::part1::process;
use tracing::subscriber::set_global_default;
use tracing_subscriber::{layer::SubscriberExt, Registry};

fn main() -> std::io::Result<()> {
    // let subscriber = tracing_subscriber::fmt::layer().json();
    // //let subscriber = tracing_subscriber::fmt().init();
    // let registry = Registry::default().with(subscriber);
    // set_global_default(registry).expect("Failed to set subscriber");

    let input = include_str!("../../data/part1-real2.txt");
    let output = process(input);
    dbg!(output);
    Ok(())
}
