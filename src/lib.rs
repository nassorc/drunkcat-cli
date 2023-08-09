use std::{env, fs, io::Write};

#[derive(Debug)]
pub enum Output {
  Stdout(std::io::Stdout),
  File(std::fs::File)
}
#[derive(Debug)]
pub struct Config {
  pub files: Vec<String>,
}
impl Config {
  pub fn build(args: &[String]) -> Config {
    Config {
      files: args[1..].to_vec(),
    }
  }
}

pub fn run(writer: &mut impl Write, config: Config) {
  let mut buffer = String::new();

  for file_path in config.files {
    match read_file(&file_path) {
      Ok(content) => buffer.push_str(&content),
      _ => ()
    }
  }
  writer.write_all(buffer.as_bytes());
}

pub fn parse_config(args: &[String]) -> Config {
  Config::build(&args)
}

pub fn read_file(file_path: &str) -> Result<String, std::io::Error> {
  fs::read_to_string(file_path)
}

pub fn write_contents<W>(mut writer: W, content: &str) -> std::io::Result<()>
where
  W: std::io::Write,
{
  writeln!(&mut writer, "{}", content)
}

#[cfg(test)]
mod tests {
  use super::*;
  use tempfile::NamedTempFile;
  use std::io::Write;
  use std::fs;
  #[test]
  fn args_contain_one_file_path() {
    let args = vec![String::from("path"), String::from("hello.txt")];
    let result = parse_config(&args);
    assert_eq!(result.files[0], "hello.txt");
  }
  #[test]
  fn args_contain_two_file_paths() {
    let args = vec![String::from("path"), String::from("hello.txt"), String::from("bye.txt")];
    let result = parse_config(&args);
    assert_eq!(result.files, ["hello.txt", "bye.txt"]);
  }
  #[test]
  fn read_one_file() {
    let mut file = NamedTempFile::new().unwrap();
    let file_path = file.path();

    writeln!(&file, "Hello world.");

    let contents = read_file(file_path.to_str().unwrap());
    assert_eq!(contents.unwrap(), "Hello world.\n");
  }
  #[test]
  fn reading_nonexistent_file_returns_error() {
    let contents = read_file("doesnotexist.txt");
    assert!(contents.is_err());
  }
  #[test]
  fn output_content_to_stdout() {
    let mut output = Vec::new();
    let content = "Lorem 100" ;
    write_contents(&mut output, &content);

    let output = String::from_utf8(output).unwrap();

    assert_eq!(output, "Lorem 100");
  }
  #[test]
  fn output_content_to_file() {
    let content = "File contents.";
    let mut file = NamedTempFile::new().unwrap();
    let file_path = file.path().to_owned();

    write_contents(&mut file, content);
    let output = fs::read_to_string(file_path).unwrap();
    assert_eq!(output, content);
  }
}