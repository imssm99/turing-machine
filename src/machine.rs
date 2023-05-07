type State = usize;
type Symbol = char;
type Tape = Vec<Symbol>;
type Program = Vec<Transition>;

pub enum Operation {
    Left,
    Right,
    Stay,
}

pub struct Transition {
    state_from: State,
    symbol_read: Symbol,
    state_to: State,
    symbol_write: Symbol,
    operation: Operation,
}

impl Transition {
    pub fn new(state_from: State, symbol_read: Symbol, state_to: State, symbol_write: Symbol, operation: Operation) -> Self {
        Self {
            state_from,
            symbol_read,
            state_to,
            symbol_write,
            operation,
        }
    }
}

pub struct Machine {
    program: Program,
    state: State,
    tape: Tape,
    head: usize,
    final_state: Vec<State>,
}

impl Machine {
    pub fn new(program: Program, inital_state: State, tape: Tape, head: usize, final_state: Vec<State>) -> Self {
        Self {
            program: program,
            state: inital_state,
            tape: tape,
            head: head,
            final_state: final_state,
        }
    }

    pub fn run(&mut self) -> bool {
        loop {
            let transition = self.program.iter().find(|t| t.state_from == self.state && t.symbol_read == self.tape[self.head]);
            match transition {
                Some(t) => {
                    self.tape[self.head] = t.symbol_write;
                    self.head = match t.operation {
                        Operation::Left => self.head - 1,
                        Operation::Right => self.head + 1,
                        Operation::Stay => self.head
                    };
                    self.state = t.state_to;
                },
                None => break,
            }
        } 

        return self.final_state.contains(&self.state);
    }
}