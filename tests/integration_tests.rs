use drunkcat::{self, Config};
use tempfile::NamedTempFile;
use std::fs::{self, File};
use std::io::{self, Write, Read};

#[test]
fn it_should_output_file_contents() {
  let mut file = NamedTempFile::new().unwrap();
  let file_path = file.path().to_str().unwrap().to_owned();
  let file_name = String::from("");
  let args: Vec<String> = vec![file_name, file_path];
  let file_content = "Test output file";
  writeln!(&file, "{}", file_content);

  let mut writer: Vec<u8> = vec![];
  let config = Config::build(&args);
  drunkcat::run(&mut writer, config);

  let writer = String::from_utf8(writer).unwrap();

  assert_eq!(writer.trim(), file_content);
}