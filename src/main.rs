use clap::Parser;

use std::time::{Duration};
use std::thread;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'H', long, default_value_t = 0)]
    hour: u64,

    #[arg(short = 'm', long, default_value_t = 0)]
    minutes: u64,

    #[arg(short = 's', long, default_value_t = 0)]
    seconds: u64,
    
    #[arg(short = 'M', long, default_value("時間です"))]
    message: String,
}

fn timer(sec: u64, msg: &str) {
       println!("Waitineg...: {} seconds", &sec);
                                                                                                        
       thread::sleep(Duration::from_secs(sec));
       println!("{}", msg);
}

fn calc_sec(h: u64, m: u64, s: u64) -> u64 {
   let mut result = s;
   
   if h > 0 {
    result = result + (h / 60 / 60);
   }
   
   if m > 0 {
    result = result + (m / 60);
   }
   
   From::from(result)
}

fn main() {
    
    let args = Args::parse();

    let h = args.hour;
    let m = args.minutes;
    let s = args.seconds;
    
    let sec = calc_sec(h, m, s);
    
    timer(sec, &args.message);
}
