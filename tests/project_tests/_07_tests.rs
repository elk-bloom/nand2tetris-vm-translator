use crate::test_helpers::test_project::test_project;

// MemoryAccess

// // Basic
#[test]
fn test_basic() {
  test_project("tests/projects/07/MemoryAccess/BasicTest/BasicTest.vm");
}

// // Pointer
#[test]
fn test_pointer() {
  test_project("tests/projects/07/MemoryAccess/PointerTest/PointerTest.vm");
}

// // Static
#[test]
fn test_static() {
  test_project("tests/projects/07/MemoryAccess/StaticTest/StaticTest.vm");
}

// StackArithmetic

// // SimpleAdd
#[test]
fn test_simple_add() {
  test_project("tests/projects/07/StackArithmetic/SimpleAdd/SimpleAdd.vm");
}

// // Stack
#[test]
fn test_stack_test() {
  test_project("tests/projects/07/StackArithmetic/StackTest/StackTest.vm");
}