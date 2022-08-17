// use std::fs::read_to_string;
// use std::iter::Iterator;
// use std::path::{Path, PathBuf};
// use std::str::FromStr;
// use std::ops::{Index, IndexMut};
//
// use toml::value::Value;
//
// use clap::Parser;
//
// struct Item {
//     name: String,
//     regex: String,
// }
//
// impl Item {
//     fn new(name: String, regex: String) -> Self {
//         Self { name, regex }
//     }
//
//     fn name(&self) -> String {
//         self.name.clone()
//     }
//
//     fn regex(&self) -> String {
//         self.regex.clone()
//     }
// }
//
// struct Configs {
//     inner: Vec<Item>,
// }
//
// impl Configs {
//     fn new<P: AsRef<Path> + Clone>(path: P) -> Self {
//         let ref config_string = read_to_string(path.clone())
//             .expect(&format!("filename {:?} is not exists", path.as_ref()));
//         let toml = Value::from_str(config_string)
//             .expect(&format!("filename {:?} is not toml file", path.as_ref()));
//
//         let mut inner = Vec::new();
//
//         match toml {
//             Value::Table(map) => {
//                 map.into_iter().fold(&mut inner, |prev, x| {
//                     let (ref name, ref value) = x;
//                     let value = value
//                         .as_table()
//                         .expect("this is not what I expect toml format");
//                     let regex = value.get("regex").expect("regex is must.");
//                     let item = Item::new(name.clone(), regex.to_string());
//                     prev.push(item);
//                     prev
//                 });
//             }
//             _ => {
//                 unreachable!()
//             }
//         }
//
//         Configs { inner }
//     }
//
//     fn to_enum_code(&self) -> String {
//         let mut code = "pub enum Token { ".to_string();
//
//         self.inner.iter().fold(&mut code, |prev, x| {
//             let add = format!("{}, ", x.name());
//             prev.push_str(&add);
//             prev
//         });
//         code.push_str("}");
//         code
//     }
// }
//
// #[test]
// fn test_parse_toml() {
//     let code = Configs::new("./test/test_toml_parse.toml").to_enum_code();
//     let ans = "pub enum Token { Manko, Tinko, }".to_string();
//     assert_eq!(ans, code);
// }
//
// #[derive(Parser, Debug)]
// #[clap(author, version, about, long_about=None)]
// struct Args {
//     /// path to the config toml file.
//     #[clap(short, long, value_name="INPUT_TOML")]
//     input: PathBuf,
//
//     /// output path
//     #[clap(short, long, value_name="OUTPUT_RS")]
//     output: PathBuf
// }
//
// fn main() {
//     let arg = Args::parse();
//     let toml_path = arg.input;
//     let output_path = arg.output;
//     let enum_code = Configs::new(toml_path).to_enum_code();
//     std::fs::write(output_path, enum_code).expect("cann't write to output");
// }
