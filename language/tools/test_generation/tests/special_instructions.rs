extern crate test_generation;
use test_generation::abstract_state::{AbstractState, AbstractValue};
use vm::file_format::{Bytecode, SignatureToken};

mod common;

#[test]
fn bytecode_pop() {
    let mut state1 = AbstractState::new();
    state1.stack_push(AbstractValue::new_primitive(SignatureToken::U64));
    let state2 = common::run_instruction(Bytecode::Pop, state1);
    assert_eq!(state2.stack_len(), 0, "stack type postcondition not met");
}
