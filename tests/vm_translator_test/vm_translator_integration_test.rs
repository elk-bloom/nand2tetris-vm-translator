use std::fs;
use vm_translator::translators::translator_traits::Translate;
use vm_translator::translators::vm_translator::VMTranslator;

mod setup {
    use super::*;

    const DEFAULT_FILENAME: &str = "default";

    pub fn translator() -> VMTranslator {
        VMTranslator::new(DEFAULT_FILENAME.to_string())
    }
}

#[test]
fn pop_arg_0() {
    let input = "pop argument 0";

    let expected =
        fs::read_to_string("tests/vm_translator_test/expected_outputs/pop/pop_arg_0.txt").unwrap();
    let actual = setup::translator().convert(input).unwrap();

    assert_eq!(actual, expected)
}

#[test]
fn pop_arg_3() {
    let input = "pop argument 3";

    let expected =
        fs::read_to_string("tests/vm_translator_test/expected_outputs/pop/pop_arg_3.txt").unwrap();
    let actual = setup::translator().convert(input).unwrap();

    assert_eq!(actual, expected)
}

#[test]
fn pop_local_0() {
    let input = "pop local 0";

    let expected =
        fs::read_to_string("tests/vm_translator_test/expected_outputs/pop/pop_local_0.txt")
            .unwrap();
    let actual = setup::translator().convert(input).unwrap();

    assert_eq!(actual, expected)
}

#[test]
fn pop_local_2() {
    let input = "pop local 2";

    let expected =
        fs::read_to_string("tests/vm_translator_test/expected_outputs/pop/pop_local_2.txt")
            .unwrap();
    let actual = setup::translator().convert(input).unwrap();

    assert_eq!(actual, expected)
}

#[test]
fn pop_pointer_0() {
    let input = "pop pointer 0";

    let expected =
        fs::read_to_string("tests/vm_translator_test/expected_outputs/pop/pop_pointer_0.txt")
            .unwrap();
    let actual = setup::translator().convert(input).unwrap();

    assert_eq!(actual, expected)
}

#[test]
fn pop_pointer_1() {
    let input = "pop pointer 1";

    let expected =
        fs::read_to_string("tests/vm_translator_test/expected_outputs/pop/pop_pointer_1.txt")
            .unwrap();
    let actual = setup::translator().convert(input).unwrap();

    assert_eq!(actual, expected)
}

#[test]
fn pop_temp_0() {
    let input = "pop temp 0";

    let expected =
        fs::read_to_string("tests/vm_translator_test/expected_outputs/pop/pop_temp_0.txt").unwrap();
    let actual = setup::translator().convert(input).unwrap();

    assert_eq!(actual, expected)
}

#[test]
fn pop_temp_4() {
    let input = "pop temp 4";

    let expected =
        fs::read_to_string("tests/vm_translator_test/expected_outputs/pop/pop_temp_4.txt").unwrap();
    let actual = setup::translator().convert(input).unwrap();

    assert_eq!(actual, expected)
}

#[test]
fn pop_that_0() {
    let input = "pop that 0";

    let expected =
        fs::read_to_string("tests/vm_translator_test/expected_outputs/pop/pop_that_0.txt").unwrap();
    let actual = setup::translator().convert(input).unwrap();

    assert_eq!(actual, expected)
}

#[test]
fn pop_that_7() {
    let input = "pop that 7";

    let expected =
        fs::read_to_string("tests/vm_translator_test/expected_outputs/pop/pop_that_7.txt").unwrap();
    let actual = setup::translator().convert(input).unwrap();

    assert_eq!(actual, expected)
}

#[test]
fn pop_this_0() {
    let input = "pop this 0";

    let expected =
        fs::read_to_string("tests/vm_translator_test/expected_outputs/pop/pop_this_0.txt").unwrap();
    let actual = setup::translator().convert(input).unwrap();

    assert_eq!(actual, expected)
}

#[test]
fn pop_this_4() {
    let input = "pop this 4";

    let expected =
        fs::read_to_string("tests/vm_translator_test/expected_outputs/pop/pop_this_4.txt").unwrap();
    let actual = setup::translator().convert(input).unwrap();

    assert_eq!(actual, expected)
}

#[test]
fn push_arg_0() {
    let input = "push argument 0";

    let expected =
        fs::read_to_string("tests/vm_translator_test/expected_outputs/push/push_arg_0.txt")
            .unwrap();
    let actual = setup::translator().convert(input).unwrap();

    assert_eq!(actual, expected)
}

#[test]
fn push_arg_3() {
    let input = "push argument 3";

    let expected =
        fs::read_to_string("tests/vm_translator_test/expected_outputs/push/push_arg_3.txt")
            .unwrap();
    let actual = setup::translator().convert(input).unwrap();

    assert_eq!(actual, expected)
}

#[test]
fn push_local_0() {
    let input = "push local 0";

    let expected =
        fs::read_to_string("tests/vm_translator_test/expected_outputs/push/push_local_0.txt")
            .unwrap();
    let actual = setup::translator().convert(input).unwrap();

    assert_eq!(actual, expected)
}

#[test]
fn push_local_2() {
    let input = "push local 2";

    let expected =
        fs::read_to_string("tests/vm_translator_test/expected_outputs/push/push_local_2.txt")
            .unwrap();
    let actual = setup::translator().convert(input).unwrap();

    assert_eq!(actual, expected)
}

#[test]
fn push_pointer_0() {
    let input = "push pointer 0";

    let expected =
        fs::read_to_string("tests/vm_translator_test/expected_outputs/push/push_pointer_0.txt")
            .unwrap();
    let actual = setup::translator().convert(input).unwrap();

    assert_eq!(actual, expected)
}

#[test]
fn push_pointer_1() {
    let input = "push pointer 1";

    let expected =
        fs::read_to_string("tests/vm_translator_test/expected_outputs/push/push_pointer_1.txt")
            .unwrap();
    let actual = setup::translator().convert(input).unwrap();

    assert_eq!(actual, expected)
}

#[test]
fn push_temp_0() {
    let input = "push temp 0";

    let expected =
        fs::read_to_string("tests/vm_translator_test/expected_outputs/push/push_temp_0.txt")
            .unwrap();
    let actual = setup::translator().convert(input).unwrap();

    assert_eq!(actual, expected)
}

#[test]
fn push_temp_4() {
    let input = "push temp 4";

    let expected =
        fs::read_to_string("tests/vm_translator_test/expected_outputs/push/push_temp_4.txt")
            .unwrap();
    let actual = setup::translator().convert(input).unwrap();

    assert_eq!(actual, expected)
}

#[test]
fn push_that_0() {
    let input = "push that 0";

    let expected =
        fs::read_to_string("tests/vm_translator_test/expected_outputs/push/push_that_0.txt")
            .unwrap();
    let actual = setup::translator().convert(input).unwrap();

    assert_eq!(actual, expected)
}

#[test]
fn push_that_7() {
    let input = "push that 7";

    let expected =
        fs::read_to_string("tests/vm_translator_test/expected_outputs/push/push_that_7.txt")
            .unwrap();
    let actual = setup::translator().convert(input).unwrap();

    assert_eq!(actual, expected)
}

#[test]
fn push_this_0() {
    let input = "push this 0";

    let expected =
        fs::read_to_string("tests/vm_translator_test/expected_outputs/push/push_this_0.txt")
            .unwrap();
    let actual = setup::translator().convert(input).unwrap();

    assert_eq!(actual, expected)
}

#[test]
fn push_this_4() {
    let input = "push this 4";

    let expected =
        fs::read_to_string("tests/vm_translator_test/expected_outputs/push/push_this_4.txt")
            .unwrap();
    let actual = setup::translator().convert(input).unwrap();

    assert_eq!(actual, expected)
}

#[test]
fn push_constant_0() {
    let input = "push constant 0";

    let expected =
        fs::read_to_string("tests/vm_translator_test/expected_outputs/push/push_constant_0.txt")
            .unwrap();
    let actual = setup::translator().convert(input).unwrap();

    assert_eq!(actual, expected)
}


#[test]
fn push_constant_25() {
    let input = "push constant 25";

    let expected =
        fs::read_to_string("tests/vm_translator_test/expected_outputs/push/push_constant_25.txt")
            .unwrap();
    let actual = setup::translator().convert(input).unwrap();

    assert_eq!(actual, expected)
}

#[test]
fn push_constant_subsequent() {
    let input_1 = "push constant 0";
    let input_2 = "push constant 25";
    
    let mut translator = setup::translator();
    
    let output_1 = translator.convert(input_1).unwrap();
    let output_2 = translator.convert(input_2).unwrap();
    
    let actual = output_1 + output_2.as_ref();

    let expected =
        fs::read_to_string("tests/vm_translator_test/expected_outputs/combination/subsequent_push_constant.txt")
            .unwrap();

    assert_eq!(actual, expected)
}