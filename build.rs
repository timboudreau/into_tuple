use std::fs::{read_to_string, File};
use std::io::*;
use std::path::PathBuf;

// Used to substitute into docs
const COUNT_SMALL: u8 = 13;
const COUNT_MEDIUM: u8 = 25;
const COUNT_LARGE: u8 = 65;
const COUNT_HUGE: u8 = 251;

#[cfg(not(feature = "medium"))]
#[cfg(not(feature = "large"))]
#[cfg(not(feature = "huge"))]
const COUNT: u8 = COUNT_SMALL;

#[cfg(feature = "medium")]
const COUNT: u8 = COUNT_MEDIUM;

#[cfg(feature = "large")]
const COUNT: u8 = COUNT_LARGE;

#[cfg(feature = "huge")]
const COUNT: u8 = COUNT_HUGE;

const DIR: &str = env!("CARGO_MANIFEST_DIR");

pub fn main() {
    generate_lib();
    generate_tests();
}

fn destination() -> PathBuf {
    PathBuf::from(std::env::var_os("OUT_DIR").unwrap())
}

fn generate_lib() {
    let mut src = PathBuf::from(DIR);
    src.push("lib.rs.template");
    let mut template = read_to_string(&src)
        .unwrap()
        .replace("$COUNT_SMALL", (COUNT_SMALL - 1).to_string().as_str())
        .replace("$COUNT_MEDIUM", (COUNT_MEDIUM - 1).to_string().as_str())
        .replace("$COUNT_LARGE", (COUNT_LARGE - 1).to_string().as_str())
        .replace("$COUNT_HUGE", (COUNT_HUGE - 1).to_string().as_str())
        .replace("$COUNT", (COUNT - 1).to_string().as_str());

    template.push_str("\n// Generated implementations:\n\n");
    for i in 0..COUNT {
        let case = generate_one(i);
        template.push_str(case.as_str());
    }
    for i in 0..COUNT {
        let case = generate_one_borrow(i);
        template.push_str(case.as_str());
    }

    // FIXME - there must be SOME way to generate your lib.rs and have
    // cargo --publish accept it -- ?
//    let mut lib_path = destination();
    let mut lib_path = PathBuf::from(DIR);
    lib_path.push("src");
    
    lib_path.push("lib.rs");
    let mut dest = File::create(&lib_path).unwrap();
    write!(dest, "{}", template).unwrap();
}

fn generate_tests() {
    let mut result = String::from("#![cfg(test)]\n\nuse super::IntoTuple;\n");
//    let mut result = String::from("#![cfg(test)]\n\n");
    for i in 0..COUNT {
        let case = generate_one_test(i);
        result.push_str(case.as_str());
        let case = generate_one_borrow_test(i);
        result.push_str(case.as_str());
    }
    let mut path = PathBuf::from(DIR);
    path.push("src");
    path.push("generated_tests.rs");
    let mut dest = File::create(&path).unwrap();
    write!(dest, "{}", result.as_str()).unwrap();
}

fn generate_one_test(count: u8) -> String {
    let mut result = format!(
        "\n#[test]\nfn test_tuple_of_{}() {{\n    let arr : [u8; {}] = [\n",
        count, count
    );
    if count == 0 {
        result.push_str("    ];\n");
        result.push_str("    let tup = arr.into_tuple();\n");
        result.push_str("    assert_eq!((), tup)\n");
        result.push_str("}\n");
    } else {
        for i in 0..count {
            result.push_str(format!("        {}_u8,\n", i + 1).as_str());
        }
        result.push_str("    ];\n\n");

        result.push_str("    let tup = arr.into_tuple();\n");
        for i in 0..count {
            result.push_str(format!("    assert_eq!({}, tup.{}, \"Item {} in an array->tuple sized to {} has the wrong value\")", i + 1, i, i, count).as_str());
            if i != count - 1 {
                result.push_str(";\n");
            } else {
                result.push('\n');
            }
        }
        result.push_str("}\n");
    }
    result
}

fn generate_one_borrow_test(count: u8) -> String {
    let mut result = format!("\nconst STATIC_ARR_{} : [u8; {}] = [", count, count);
    for i in 0..count {
        result.push_str(format!("{}_u8, ", i + 1).as_str());
        if i > 0 && i % 5 == 0 && i < count - 1 {
            result.push_str("\n    ");
        }
    }
    result.push_str("];\n");
    result.push_str(
        format!(
            "static STATIC_ARR_REF_{} : &'static [u8; {}] = &STATIC_ARR_{};\n",
            count, count, count
        )
        .as_str(),
    );

    result.push_str(format!("\n#[test]\nfn test_borrow_tuple_of_{}() {{\n    let tup = STATIC_ARR_REF_{}.into_tuple();\n", count, count).as_str());
    for i in 0..count {
        result.push_str(format!("    assert_eq!({}, *tup.{});\n", i + 1, i).as_str());
    }
    result.push_str("}\n\n");
    result
}

fn generate_one(count: u8) -> String {
    let some_es = comma_delimited(count, "E");

    let mut result = format!("impl<E> Sealed for [E; {}]{{ }}\n", count);
    result.push_str(format!("impl<'l, E> Sealed for &'l [E; {}]{{ }}\n", count).as_str());
    result.push_str(
        format!(
            "impl<E> IntoTuple<{}, E, ({})> for [E; {}] {{\n    ",
            count, &some_es, count
        )
        .as_str(),
    );

    let arg_name = if count == 0 {
        // avoid the compiler complaining about unused variables here
        "self"
    } else {
        "self"
    };

    result.push_str(
        format!(
            "fn into_tuple({}: [E; {}]) -> ({}) {{\n        ",
            arg_name, count, &some_es
        )
        .as_str(),
    );
    if count == 0 {
        result.push_str("()\n    }\n}\n\n");
    } else {
        result.push_str("let mut it = self.into_iter();\n        ");
        result.push_str("unsafe {\n            (\n");
        for i in 0..count {
            result.push_str(
                format!("                 it.next().unwrap_unchecked(), // {}\n", i).as_str(),
            );
        }
        result.push_str("            )\n        }\n    }\n}\n\n");
    }
    result
}

fn generate_one_borrow(count: u8) -> String {
    if count == 0 {
        return String::from("");
    }
    let some_es = comma_delimited(count, "&'l E");
    let mut result = format!(
        "impl<'l, E> IntoTuple<{}, E, ({})> for &'l [E; {}] {{\n    ",
        count, &some_es, count
    );

    let arg_name = if count == 0 {
        // avoid the compiler complaining about unused variables here
        "self"
    } else {
        "self"
    };

    result.push_str(
        format!(
            "fn into_tuple({}: &'l [E; {}]) -> ({}) {{\n        ",
            arg_name, count, &some_es
        )
        .as_str(),
    );
    if count == 0 {
        result.push_str("()\n    }\n}\n\n");
    } else {
        result.push_str("let mut it = self.iter();\n        ");
        result.push_str("unsafe {\n            (\n");
        for i in 0..count {
            result.push_str(
                format!("                 it.next().unwrap_unchecked(), // {}\n", i).as_str(),
            );
        }
        result.push_str("            )\n        }\n    }\n}\n\n");
    }
    result
}

fn comma_delimited(count: u8, what: impl Into<String>) -> String {
    let mut result = String::new();
    let s = what.into();
    for _ in 0..count {
        if !result.is_empty() {
            result.push_str(", ");
        }
        result.push_str(s.as_str());
    }
    if count == 1 {
        // We need to return (E,) or the compiler will interpret (E) as E and
        // complain about unnecessary parentheses, and then we are no longer returning
        // the right type
        result.push(',');
    }
    result
}
