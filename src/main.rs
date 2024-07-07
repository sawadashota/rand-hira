use clap::Parser;
use rand::Rng;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value = "10")]
    length: usize,
}

fn main() {
    let args = Args::parse();

    let rand_hiragana_string: String = (0..args.length)
        .map(|_| {
            let rand_num = rand::thread_rng().gen_range(0x3041..=0x3096);
            char::from_u32(rand_num).unwrap()
        })
        .collect();

    println!("{}", rand_hiragana_string);
}
