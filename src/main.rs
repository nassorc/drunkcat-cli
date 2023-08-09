use std::env;
use std::io;
use drunkcat;
fn main() {
  let args: Vec<String> = env::args().collect();
  let config = drunkcat::parse_config(&args);
  let mut writer = io::stdout();
  drunkcat::run(&mut writer, config);
}
