use super::lib::*;

use std::collections::HashSet;

pub fn run() -> (Option<String>, Option<String>) {
    let filename = "inputs/day8.txt";
    let inputs = read_inputs(&filename);
    let program = parse_program(&inputs);

    let part_one = Some(
        run_program_and_stop_on_first_repeated(&program)
            .accumulator
            .to_string(),
    );
    let part_two = Some(fix_and_run_program(&program).accumulator.to_string());

    (part_one, part_two)
}

fn parse_program(program: &str) -> Vec<Instruction> {
    program
        .lines()
        .map(|line| line.split_whitespace())
        .map(|mut line| Instruction {
            operation: Operation::from_str(line.next().expect("Expected an operation"))
                .expect("Expected an operation"),
            argument: line
                .next()
                .expect("Expected argument")
                .parse::<i64>()
                .expect("Expected an integer"),
        })
        .collect()
}

fn run_program_and_stop_on_first_repeated(program: &[Instruction]) -> State {
    let mut state = State::EMPTY;
    let mut instruction_history = HashSet::new();

    loop {
        let instruction = program.get(state.program_counter);

        if instruction.is_none() {
            break state;
        }

        let instruction = program.get(state.program_counter).unwrap();
        state = run_instruction(instruction, &state);

        if instruction_history.contains(&state.program_counter) {
            break state;
        }
        instruction_history.insert(state.program_counter);
    }
}

fn run_instruction(instruction: &Instruction, state: &State) -> State {
    match instruction.operation {
        Operation::NOP => State {
            program_counter: state.program_counter + 1,
            ..*state
        },
        Operation::JMP => State {
            program_counter: (state.program_counter as i64 + instruction.argument) as usize,
            ..*state
        },
        Operation::ACC => State {
            program_counter: state.program_counter + 1,
            accumulator: state.accumulator + instruction.argument,
        },
    }
}

fn fix_and_run_program(program: &[Instruction]) -> State {
    let mut starting_idx = 0;

    loop {
        let mut program = program.to_vec();
        let position = program
            .iter()
            .skip(starting_idx)
            .position(|instruction| {
                matches!(instruction.operation, Operation::NOP | Operation::JMP)
            })
            .unwrap();

        let mut instruction = program.get_mut(starting_idx + position).unwrap();
        starting_idx += position + 1;
        instruction.operation = instruction.operation.opposite_instruction();
        let state = run_program_and_stop_on_first_repeated(&program);
        if state.program_counter == program.len() {
            break state;
        }
    }
}

#[derive(Debug, PartialEq)]
struct State {
    accumulator: i64,
    program_counter: usize,
}

impl State {
    const EMPTY: State = State {
        program_counter: 0,
        accumulator: 0,
    };
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Operation {
    NOP,
    ACC,
    JMP,
}

impl Operation {
    fn from_str(operation: &str) -> Option<Operation> {
        match operation {
            "nop" => Some(Operation::NOP),
            "acc" => Some(Operation::ACC),
            "jmp" => Some(Operation::JMP),
            _ => None,
        }
    }

    fn opposite_instruction(&self) -> Self {
        match self {
            Operation::NOP => Operation::JMP,
            Operation::JMP => Operation::NOP,
            _ => *self,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Instruction {
    operation: Operation,
    argument: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CASE: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn test_parse_program() {
        let program = "nop +0
acc +1
jmp +4";

        let expected_program = vec![
            Instruction {
                operation: Operation::NOP,
                argument: 0,
            },
            Instruction {
                operation: Operation::ACC,
                argument: 1,
            },
            Instruction {
                operation: Operation::JMP,
                argument: 4,
            },
        ];
        assert_eq!(expected_program, parse_program(program));
    }

    #[test]
    fn test_operation_from_str() {
        assert_eq!(Some(Operation::NOP), Operation::from_str("nop"));
        assert_eq!(Some(Operation::ACC), Operation::from_str("acc"));
        assert_eq!(Some(Operation::JMP), Operation::from_str("jmp"));
        assert_eq!(None, Operation::from_str("abc"));
    }

    #[test]
    fn test_run_program_and_stop_on_first_repeated() {
        let program = parse_program(&TEST_CASE);
        assert_eq!(
            5,
            run_program_and_stop_on_first_repeated(&program).accumulator
        );
    }

    #[test]
    fn test_fix_and_run_program() {
        let program = parse_program(&TEST_CASE);
        assert_eq!(8, fix_and_run_program(&program).accumulator);
    }

    #[test]
    fn test_run_instruction() {
        let initial_state = State {
            accumulator: 10,
            program_counter: 10,
        };
        assert_eq!(
            State {
                program_counter: initial_state.program_counter + 1,
                ..initial_state
            },
            run_instruction(
                &Instruction {
                    operation: Operation::NOP,
                    argument: 0
                },
                &initial_state
            )
        );
        assert_eq!(
            State {
                program_counter: initial_state.program_counter + 10,
                ..initial_state
            },
            run_instruction(
                &Instruction {
                    operation: Operation::JMP,
                    argument: 10
                },
                &initial_state,
            )
        );
        assert_eq!(
            State {
                program_counter: initial_state.program_counter + 1,
                accumulator: initial_state.accumulator + 10,
            },
            run_instruction(
                &Instruction {
                    operation: Operation::ACC,
                    argument: 10
                },
                &initial_state
            )
        );
    }
}
