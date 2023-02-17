use clap::Parser;

use rodio::{Decoder, OutputStream, source::Source};

use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::time::{Duration};
use std::thread;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Hour, type is 64bit unsigned integer, optionally.
    #[arg(short = 'H', long, default_value_t = 0)]
    hour: u64,

    /// Minutes, type is 64bit unsigned integer, optionally.
    #[arg(short = 'm', long, default_value_t = 0)]
    minutes: u64,

    /// Seconds, type is 64bit unsigned integer, optionally.
    #[arg(short = 's', long, default_value_t = 0)]
    seconds: u64,
    
    /// Message, type is string, output to stdout when at the specified time. Optionally.
    #[arg(short = 'M', long, default_value("時間です"))]
    message: String,
    
    /// sound file path, optionally.
    #[arg(short = 'f', long)]
    file_path: Option<String>,
}

fn timer(sec: u64, msg: &str, file_path_str: Option<String>) {
    println!("Waitineg...: {} seconds", &sec);
                                                                                                        
    thread::sleep(Duration::from_secs(sec));
    println!("{}", msg);
    
    let Some(p_str) = file_path_str else {
        return;
    };
    
    play_audio_from_file(&p_str);
}

fn calc_sec(h: u64, m: u64, s: u64) -> u64 {
   let mut result = s;
   
   if h > 0 {
    result = result + (h * 60 * 60);
   }
   
   if m > 0 {
    result = result + (m * 60);
   }
   
   result
}

fn play_audio_from_file(path_str: &str) {
   let Ok(_r) = OutputStream::try_default() else {
    return;
   };
   
   let (_output_stream, _output_stream_handel) = _r;
   
   let _path = Path::new(path_str);
   let Ok(_file) = File::open(_path) else {
    return;
   };
   
   let buf = BufReader::new(_file);

   let Ok(_source) = Decoder::new(buf) else {
       return;
   };

   let Some(total_duration) = _source.total_duration() else {
    return;
   };
   
   let _ = _output_stream_handel.play_raw(_source.convert_samples());
   
   std::thread::sleep(total_duration);
}

fn main() {
    
    let args = Args::parse();

    let h = args.hour;
    let m = args.minutes;
    let s = args.seconds;
    let sec = calc_sec(h, m, s);
    
    timer(sec, &args.message, args.file_path);
}
