use clap::Parser;

use vtt_2x::{run, Choreographer};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(required = true)]
    paths: Vec<String>,

    #[arg(long)]
    chat_time: Option<u32>,

    #[arg(long)]
    min_caption_time: Option<u32>,

    #[arg(long)]
    caption_divisor: Option<u8>,
}

fn main() {
    let args = Args::parse();
    run(
        args.paths,
        Choreographer::new(
            args.chat_time,
            args.min_caption_time,
            args.caption_divisor.map(|x| {
                if x == 0 {
                    panic!("Division by zero")
                } else {
                    x as u32
                }
            }),
        ),
    );
}
