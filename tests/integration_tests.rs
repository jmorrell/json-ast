extern crate json_ast;

use std::fs::{self, DirEntry};
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use json_ast::parse;
use std::env;
use std::ffi::OsString;

#[test]
fn trailing_comma_1() {
  parse(r##"{
    "test": {
      "foo": "bar",
    }
  }"##);
  assert!(true);
}

#[test]
fn trailing_comma_2() {
  parse(r##"{
    "test": {
      "foo": "one",
      "bar": "two",
      "baz": 123,
    }
  }"##);
  assert!(true);
}

#[test]
fn trailing_comma_3() {
  parse(r##"{
    "test": {
      "foo": "one",
      "bar": "two",
      "baz": {
        "a": "whole",
        "new": "object",
      },
    }
  }"##);
  assert!(true);
}

// #[test]
// fn fail_0() {
//   parse(r##""##);
//   assert!(true);
// }

// #[test]
// fn fail_0b() {
//   parse(
//     r##"  
      
//       "##,
//   );
//   assert!(true);
// }

// #[test]
// fn fail_1() {
//   let result = parse(r##"["a", "b""##);
//   println!("{:#?}", result);
//   assert!(true);
// }

// #[test]
// fn fail_2() {
//   parse(r##"[][]"##);
//   assert!(true);
// }

// #[test]
// fn fail_3() {
//   parse(r##"[1]x"##);
//   assert!(true);
// }

#[test]
fn pass_0() {
  parse(
    r##"
{
  "a<": 2,
  "b)": {
    "c(": [
      "3!", "4:", "5;", "6'"
    ],
    "d&": {
      "e!": "~_~"
    },
    ":e": "𠮷",
    " f ": "*±*∆"
  }
}
  "##,
  );
  assert!(true);
}

#[test]
fn pass_1() {
  parse(r##"

        [
            "JSON Test Pattern pass1",
            {"object with 1 member":["array with 1 element"]},
            {},
            [],
            true,
            false,
            null,
            {
                "integer": 1234567890,
                "e": 0.123456789e-12,
                "E": 1.234567890E+34,
                "":  23456789012E66,
                "zero": 0,
                "one": 1,
                "space": " ",
                "quote": "\"",
                "backslash": "\\",
                "controls": "\b\f\n\r\t",
                "slash": "/ & \/",
                "alpha": "abcdefghijklmnopqrstuvwyz",
                "ALPHA": "ABCDEFGHIJKLMNOPQRSTUVWYZ",
                "digit": "0123456789",
                "0123456789": "digit",
                "special": "`1~!@#$%^&*()_+-={':[,]}|;.</>?",
                "hex": "\u0123\u4567\u89AB\uCDEF\uabcd\uef4A",
                "true": true,
                "false": false,
                "null": null,
                "array":[  ],
                "object":{  },
                "address": "50 St. James Street",
                "url": "http://www.JSON.org/",
                "comment": "// /* <!-- --",
                "# -- --> */": " ",
                " s p a c e d " :[1,2 , 3

        ,

        4 , 5        ,          6           ,7        ],"compact":[1,2,3,4,5,6,7],
                "jsontext": "{\"object with 1 member\":[\"array with 1 element\"]}",
                "quotes": "&#34; \u0022 %22 0x22 034 &#x22;",
                "\/\\\"\uCAFE\uBABE\uAB98\uFCDE\ubcda\uef4A\b\f\n\r\t`1~!@#$%^&*()_+-=[]{}|;:',./<>?"
        : "A key can be any string"
            },
            0.5 ,98.6
        ,
        99.44
        ,

        1066,
        1e1,
        0.1e1,
        1e-1,
        1e00,2e+00,2e-00
        ,"rosebud"]

        "##);

  assert!(true);
}

#[test]
fn pass_2() {
  parse(r#"[[[[[[[[[[[[[[[[[[["Not too deep"]]]]]]]]]]]]]]]]]]]"#);

  assert!(true);
}

#[test]
fn pass_3() {
  parse(
    r#"

        {
            "JSON Test Pattern pass3": {
                "The outermost value": "must be an object or array.",
                "In this test": "It is an object."
            }
        }

        "#,
  );

  assert!(true);
}

#[test]
fn snapshots() {
  match env::var_os("BUILD_SNAPSHOTS") {
    Some(val) => {
      if val == OsString::from("true") {
        build_snapshots();
        assert!(true);
      } else {
        test_snapshots();
      }
    }
    _ => {
      test_snapshots();
    }
  }
}

fn build_snapshots() {
  for entry in fs::read_dir(Path::new("./tests/fixtures/valid/")).unwrap() {
    let entry = entry.unwrap();
    create_snapshot(entry);
  }
}

fn test_snapshots() {
  for entry in fs::read_dir(Path::new("./tests/fixtures/valid/")).unwrap() {
    let entry = entry.unwrap();
    test_snapshot(entry);
  }
  assert!(true);
}

fn test_snapshot(entry: DirEntry) {
  let name = entry.file_name();
  let saved = format!(
    "./tests/snapshots/valid/{}.snapshot",
    name.into_string().unwrap()
  );
  let snapshot = format_snapshot(entry);

  let mut buffer = String::new();
  let mut f = File::open(saved).unwrap();
  f.read_to_string(&mut buffer).unwrap();

  assert_eq!(buffer, snapshot);
}

fn format_snapshot(entry: DirEntry) -> String {
  let path = entry.path();
  let mut buffer = String::new();
  let mut f = File::open(path).unwrap();
  f.read_to_string(&mut buffer).unwrap();

  let val = parse(&buffer);

  format!("JSON:\n{}\n\nValue:\n{:#?}\n", buffer, val)
}

fn create_snapshot(entry: DirEntry) {
  let name = entry.file_name();
  let snapshot = format_snapshot(entry);
  let out = format!(
    "./tests/snapshots/valid/{}.snapshot",
    name.into_string().unwrap()
  );
  let mut f = File::create(out).unwrap();
  f.write_all(snapshot.as_bytes()).unwrap();
}
