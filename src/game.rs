use crate::interpretation::*;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Cell {
    Spawn(Direction),
    Kill,
    Redirect(Direction),
    Hold,
    Add,
    Sub,
    Delay,
    Print,
}

impl IsStart for Cell {
    fn is_start(&self) -> bool {
        match self {
            Cell::Spawn => true,
            _ => false,
        }
    }
}

impl AsCode for Cell {
    fn as_code(&self) -> String {
        match self {
            Cell::Spawn(dir) => format!("Cell::Spawn({})", dir.as_code()),
            Cell::Kill => "Cell::Kill".to_string(),
            Cell::Redirect(dir) => format!("Cell::Redirect({})", dir.as_code()),
            Cell::Hold => "Cell::Hold".to_string(),
            Cell::Add => "Cell::Add".to_string(),
            Cell::Sub => "Cell::Sub".to_string(),
            Cell::Delay => "Cell::Delay".to_string(),
            Cell::Print => "Cell::Print".to_string()
        }
    }
}

pub struct Interpreter();

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter()
    }

    pub fn interpret<G, F, O>(code: G, input: F, output: O) -> u8
    where 
        F: Fn(&str) -> String, 
        O: Fn(String),
        G: Grid<Cell>,
    {
        
    }
}