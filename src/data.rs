#[derive(Clone)]
pub enum Instruction {
    Add(u8),
    Subtract(u8),
    Store(u8),
    Load(u8),
    BranchAlways(usize),
    BranchZero(usize),
    BranchPositive(usize),
    Input,
    Output,
    Halt,
}

pub struct Program {
    pub initial_memory: Vec<u16>,
    pub instructions: Vec<Instruction>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            initial_memory: vec![0; 100],
            instructions: Vec::new(),
        }
    }
}
