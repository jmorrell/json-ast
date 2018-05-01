#[macro_use]
extern crate snaptest;
extern crate json_ast;

use json_ast::parse;

#[test]
fn pass_0() {
  parse(r##"
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
  "##);
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

        "##).unwrap();

  assert!(true);
}

#[test]
fn pass_2() {
  parse(r#"[[[[[[[[[[[[[[[[[[["Not too deep"]]]]]]]]]]]]]]]]]]]"#).unwrap();

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
  ).unwrap();

  assert!(true);
}

#[cfg(test)]
mod tests {

  use json_ast::{parse, Value};

  snaptest! {
    fn parse_1() -> Option<Value> {
      parse(include_str!("./fixtures/valid/array-in-arrays.json"))
    }
  }

  snaptest! {
    fn parse_2() -> Option<Value> {
        parse(include_str!("./fixtures/valid/array-in-arrays.json"))
    }
  }

  snaptest! {
    fn parse_3() -> Option<Value> {
        parse(include_str!("./fixtures/valid/array-only.json"))
    }
  }

  snaptest! {
    fn parse_4() -> Option<Value> {
        parse(include_str!("./fixtures/valid/big.json"))
    }
  }

  snaptest! {
    fn parse_5() -> Option<Value> {
        parse(include_str!("./fixtures/valid/deep.json"))
    }
  }

  snaptest! {
    fn parse_6() -> Option<Value> {
        parse(include_str!("./fixtures/valid/exponential-numbers.json"))
    }
  }

  snaptest! {
    fn parse_7() -> Option<Value> {
        parse(include_str!("./fixtures/valid/literals.json"))
    }
  }

  snaptest! {
    fn parse_8() -> Option<Value> {
        parse(include_str!("./fixtures/valid/number-only.json"))
    }
  }

  snaptest! {
    fn parse_9() -> Option<Value> {
        parse(include_str!("./fixtures/valid/object-only.json"))
    }
  }

  snaptest! {
    fn parse_10() -> Option<Value> {
        parse(include_str!("./fixtures/valid/string-escaping.json"))
    }
  }

  snaptest! {
    fn parse_11() -> Option<Value> {
        parse(include_str!("./fixtures/valid/string-only.json"))
    }
  }

  snaptest! {
    fn parse_12() -> Option<Value> {
        parse(include_str!("./fixtures/valid/symbols.json"))
    }
  }

  snaptest! {
    fn parse_13() -> Option<Value> {
        parse(include_str!("./fixtures/valid/unicode.json"))
    }
  }

  snaptest! {
    fn parse_14() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_array_arraysWithSpaces.json"))
    }
  }

  snaptest! {
    fn parse_15() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_array_empty-string.json"))
    }
  }

  snaptest! {
    fn parse_15b() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_array_empty.json"))
    }
  }

  snaptest! {
    fn parse_16() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_array_ending_with_newline.json"))
    }
  }

  snaptest! {
    fn parse_17() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_array_false.json"))
    }
  }

  snaptest! {
    fn parse_18() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_array_heterogeneous.json"))
    }
  }

  snaptest! {
    fn parse_19() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_array_null.json"))
    }
  }

  snaptest! {
    fn parse_20() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_array_with_1_and_newline.json"))
    }
  }

  snaptest! {
    fn parse_21() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_array_with_leading_space.json"))
    }
  }

  snaptest! {
    fn parse_22() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_array_with_several_null.json"))
    }
  }

  snaptest! {
    fn parse_23() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_array_with_trailing_space.json"))
    }
  }

  snaptest! {
    fn parse_24() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_number.json"))
    }
  }

  snaptest! {
    fn parse_25() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_number_0e+1.json"))
    }
  }

  snaptest! {
    fn parse_26() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_number_0e1.json"))
    }
  }

  snaptest! {
    fn parse_27() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_number_after_space.json"))
    }
  }

  snaptest! {
    fn parse_28() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_number_double_close_to_zero.json"))
    }
  }

  snaptest! {
    fn parse_29() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_number_int_with_exp.json"))
    }
  }

  snaptest! {
    fn parse_30() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_number_minus_zero.json"))
    }
  }

  snaptest! {
    fn parse_31() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_number_negative_int.json"))
    }
  }

  snaptest! {
    fn parse_32() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_number_negative_one.json"))
    }
  }

  snaptest! {
    fn parse_33() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_number_negative_zero.json"))
    }
  }

  snaptest! {
    fn parse_34() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_number_real_capital_e.json"))
    }
  }

  snaptest! {
    fn parse_35() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_number_real_capital_e_neg_exp.json"))
    }
  }

  snaptest! {
    fn parse_36() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_number_real_capital_e_pos_exp.json"))
    }
  }

  snaptest! {
    fn parse_37() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_number_real_exponent.json"))
    }
  }

  snaptest! {
    fn parse_38() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_number_real_fraction_exponent.json"))
    }
  }

  snaptest! {
    fn parse_39() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_number_real_neg_exp.json"))
    }
  }

  snaptest! {
    fn parse_40() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_number_real_pos_exponent.json"))
    }
  }

  snaptest! {
    fn parse_41() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_number_simple_int.json"))
    }
  }

  snaptest! {
    fn parse_42() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_number_simple_real.json"))
    }
  }

  snaptest! {
    fn parse_43() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_object.json"))
    }
  }

  snaptest! {
    fn parse_44() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_object_basic.json"))
    }
  }

  snaptest! {
    fn parse_45() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_object_duplicated_key.json"))
    }
  }

  snaptest! {
    fn parse_46() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_object_duplicated_key_and_value.json"))
    }
  }

  snaptest! {
    fn parse_47() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_object_empty.json"))
    }
  }

  snaptest! {
    fn parse_48() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_object_empty_key.json"))
    }
  }

  snaptest! {
    fn parse_49() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_object_escaped_null_in_key.json"))
    }
  }

  snaptest! {
    fn parse_50() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_object_extreme_numbers.json"))
    }
  }

  snaptest! {
    fn parse_51() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_object_long_strings.json"))
    }
  }

  snaptest! {
    fn parse_52() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_object_simple.json"))
    }
  }

  snaptest! {
    fn parse_53() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_object_string_unicode.json"))
    }
  }

  snaptest! {
    fn parse_54() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_object_with_newlines.json"))
    }
  }

  snaptest! {
    fn parse_55() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_string_1_2_3_bytes_UTF-8_sequences.json"))
    }
  }

  snaptest! {
    fn parse_56() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_string_accepted_surrogate_pair.json"))
    }
  }

  snaptest! {
    fn parse_57() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_string_accepted_surrogate_pairs.json"))
    }
  }

  snaptest! {
    fn parse_58() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_string_allowed_escapes.json"))
    }
  }

  snaptest! {
    fn parse_59() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_string_backslash_and_u_escaped_zero.json"))
    }
  }

  snaptest! {
    fn parse_60() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_string_backslash_doublequotes.json"))
    }
  }

  snaptest! {
    fn parse_61() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_string_comments.json"))
    }
  }

  snaptest! {
    fn parse_62() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_string_double_escape_a.json"))
    }
  }

  snaptest! {
    fn parse_63() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_string_double_escape_n.json"))
    }
  }

  snaptest! {
    fn parse_64() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_string_escaped_control_character.json"))
    }
  }

  snaptest! {
    fn parse_65() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_string_escaped_noncharacter.json"))
    }
  }

  snaptest! {
    fn parse_66() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_string_in_array.json"))
    }
  }

  snaptest! {
    fn parse_67() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_string_in_array_with_leading_space.json"))
    }
  }

  snaptest! {
    fn parse_68() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_string_last_surrogates_1_and_2.json"))
    }
  }

  snaptest! {
    fn parse_69() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_string_nbsp_uescaped.json"))
    }
  }

  snaptest! {
    fn parse_70() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_string_nonCharacterInUTF-8_U+10FFFF.json"))
    }
  }

  snaptest! {
    fn parse_71() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_string_nonCharacterInUTF-8_U+1FFFF.json"))
    }
  }

  snaptest! {
    fn parse_72() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_string_nonCharacterInUTF-8_U+FFFF.json"))
    }
  }

  snaptest! {
    fn parse_73() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_string_null_escape.json"))
    }
  }

  snaptest! {
    fn parse_74() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_string_one-byte-utf-8.json"))
    }
  }

  snaptest! {
    fn parse_75() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_string_pi.json"))
    }
  }

  snaptest! {
    fn parse_76() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_string_simple_ascii.json"))
    }
  }

  snaptest! {
    fn parse_77() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_string_space.json"))
    }
  }

  snaptest! {
    fn parse_78() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_string_surrogates_U+1D11E_MUSICAL_SYMBOL_G_CLEF.json"))
    }
  }

  snaptest! {
    fn parse_79() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_string_three-byte-utf-8.json"))
    }
  }

  snaptest! {
    fn parse_80() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_string_two-byte-utf-8.json"))
    }
  }

  snaptest! {
    fn parse_81() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_string_u+2028_line_sep.json"))
    }
  }

  snaptest! {
    fn parse_82() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_string_u+2029_par_sep.json"))
    }
  }

  snaptest! {
    fn parse_83() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_string_uEscape.json"))
    }
  }

  snaptest! {
    fn parse_84() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_string_uescaped_newline.json"))
    }
  }

  snaptest! {
    fn parse_85() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_string_unescaped_char_delete.json"))
    }
  }

  snaptest! {
    fn parse_86() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_string_unicode.json"))
    }
  }

  snaptest! {
    fn parse_87() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_string_unicodeEscapedBackslash.json"))
    }
  }

  snaptest! {
    fn parse_88() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_string_unicode_2.json"))
    }
  }

  snaptest! {
    fn parse_89() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_string_unicode_U+10FFFE_nonchar.json"))
    }
  }

  snaptest! {
    fn parse_90() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_string_unicode_U+1FFFE_nonchar.json"))
    }
  }

  snaptest! {
    fn parse_91() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_string_unicode_U+200B_ZERO_WIDTH_SPACE.json"))
    }
  }

  snaptest! {
    fn parse_92() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_string_unicode_U+2064_invisible_plus.json"))
    }
  }

  snaptest! {
    fn parse93() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_string_unicode_U+FDD0_nonchar.json"))
    }
  }

  snaptest! {
    fn parse_94() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_string_unicode_U+FFFE_nonchar.json"))
    }
  }

  snaptest! {
    fn parse_95() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_string_unicode_escaped_double_quote.json"))
    }
  }

  snaptest! {
    fn parse_96() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_string_utf8.json"))
    }
  }

  snaptest! {
    fn parse_97() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_string_with_del_character.json"))
    }
  }

  snaptest! {
    fn parse_98() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_structure_lonely_false.json"))
    }
  }

  snaptest! {
    fn parse_99() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_structure_lonely_int.json"))
    }
  }

  snaptest! {
    fn parse_100() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_structure_lonely_negative_real.json"))
    }
  }

  snaptest! {
    fn parse_101() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_structure_lonely_null.json"))
    }
  }

  snaptest! {
    fn parse_102() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_structure_lonely_string.json"))
    }
  }

  snaptest! {
    fn parse_103() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_structure_lonely_true.json"))
    }
  }

  snaptest! {
    fn parse_104() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_structure_string_empty.json"))
    }
  }

  snaptest! {
    fn parse_105() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_structure_trailing_newline.json"))
    }
  }

  snaptest! {
    fn parse_106() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_structure_true_in_array.json"))
    }
  }

  snaptest! {
    fn parse_107() -> Option<Value> {
        parse(include_str!("./fixtures/valid/y_structure_whitespace_array.json"))
    }
  }

}

