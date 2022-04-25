use super::structure::*;
use std::io::{Write, BufWriter};
use std::fs::OpenOptions;
use std::process::Command;
pub use m43lang_derive::AsCode;

pub const STORAGE_SIZE: usize = 1024;

pub trait Interpretable {
    fn interpret<I: FnMut(&str) -> String, O: FnMut(String)>(&self, input: I, output: O) -> u8;
}

pub trait Compilable: Interpretable {
    fn compile(&mut self) -> String;
}

pub trait Debuggable: Interpretable {
    fn debug<'a, G: Grid<Block>, D: Debugger<G, I, O>, I: FnMut(&str) -> String, O: FnMut(String)>(&'a self, input: I, output: O) -> &'a D;
}

pub struct GridState {
    pub dir: Direction,
    pub pos: Index,
    pub val: Value,
    pub storage: [Value; STORAGE_SIZE],
    pub coords: (usize, usize),
}

impl GridState {
    pub fn walk(&mut self, width: usize, height: usize) -> Result<(), u8> {
        match self.dir {
            Direction::Up => {
                if self.coords.1 == 0 {
                    return Err(1);
                }
                self.coords.1 -= 1;
            }
            Direction::Down => {
                if self.coords.1 == height - 1 {
                    return Err(1);
                }
                self.coords.1 += 1;
            }
            Direction::Left => {
                if self.coords.0 == 0 {
                    return Err(1);
                }
                self.coords.0 -= 1;
            }
            Direction::Right => {
                if self.coords.0 == width - 1 {
                    return Err(1);
                }
                self.coords.0 += 1;
            }
        }
        Ok(())
    }
}

pub trait Executable {
    fn execute<I, O>(&self, state: &mut GridState, input: &mut I, output: &mut O) -> bool
    where
        I: FnMut(&str) -> String,
        O: FnMut(String);
}

impl Executable for Block {
    fn execute<I, O>(&self, s: &mut GridState, input: &mut I, output: &mut O) -> bool
    where
        I: FnMut(&str) -> String,
        O: FnMut(String)
    {
        match self {
            Block::Start(_) => {},
            Block::Redirect(d) => s.dir = *d,
            Block::Store => s.storage[s.pos] = s.val,
            Block::Load => s.val = s.storage[s.pos],
            Block::Swap => std::mem::swap(&mut s.val, &mut s.storage[s.pos]),
            Block::MoveRight(n) => s.pos += *n,
            Block::MoveLeft(n) => s.pos -= *n,
            Block::Goto(n) => s.pos = *n,
            Block::Set(v) => s.val = *v,
            Block::Save(n) => s.storage[s.pos] = *n,
            Block::Increment(v) => s.val += *v,
            Block::Decrement(v) => s.val -= *v,
            Block::OpAdd => s.val += s.storage[s.pos],
            Block::OpSub => s.val -= s.storage[s.pos],
            Block::OpMul => s.val *= s.storage[s.pos],
            Block::OpDiv => s.val /= s.storage[s.pos],
            Block::CompLarger => s.val = if s.val > s.storage[s.pos] { 1 } else { 0 },
            Block::CompSmaller => s.val = if s.val < s.storage[s.pos] { 1 } else { 0 },
            Block::CompEqual => s.val = if s.val == s.storage[s.pos] { 1 } else { 0 },
            Block::Conditional(d1, d2) => s.dir = if s.val == 0 { *d2 } else { *d1 },
            Block::Display => output(format!("{}", s.val)),
            Block::Print => output(format!("{}", s.val as u8 as char)),
            Block::Break => output("\n".to_string()),
            Block::Input => s.val = input(&format!("{}", s.val)).parse().unwrap(),
            Block::End => return false,
        }
        return true;
    }
}

impl<G: Grid<Block>> Interpretable for G {
    fn interpret<I: FnMut(&str) -> String, O: FnMut(String) -> ()>(&self, mut input: I, mut output: O) -> u8 {
        let mut state = GridState {
            dir: Direction::Down,
            pos: 0,
            val: 0,
            storage: [0; STORAGE_SIZE],
            coords: self.find_start().expect("No start found")
        };
        let mut block = Block::End;
        
        if let Some(Block::Start(d)) = self.get_pos(state.coords) {
            state.dir = *d;
        } else {
            panic!("No start block found");
        }
    
        let mut none;
        while {
            let b = self.get(state.coords.0, state.coords.1);
            none = b.is_none();
            if none {
                true
            } else {
                block = b.unwrap();
                block != Block::End
            }
         } {
            if !none {
                block.execute(&mut state, &mut input, &mut output);
            }
    
            if let Err(k) = state.walk(self.get_width(), self.get_height()) {
                return k;
            }
        }
    
        return 0;
    }
}

impl Compilable for DynGrid<Block> {
    fn compile(&mut self) -> String {
        self.optimize();
    
        println!("{}", self.to_string());
    
        {
            let file = OpenOptions::new()
                .write(true)
                .create(false)
                .truncate(true)
                .open("inner/src/lib.rs")
                .expect("Unable to open file");
    
            let mut writer = BufWriter::new(file);
    
            writer.write(br#"mod utils;
    
    use wasm_bindgen::prelude::*;
    use m43lang_visual::*;
    
    #[cfg(feature = "wee_allo")]
    #[global_allocator]
    static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    
    #[wasm_bindgen]
    extern {
        fn prompt(s: &str) -> String;
    
        fn print_to_console(s: String);
    }
    
    const PROGRAM: ConstGrid<Block, "#).expect("Unable to write to file");
            write!(writer, "{}", self.get_cells().len()).expect("Unable to write to file");
            writer.write(br#"> = ConstGrid {
        cells: "#).expect("Unable to write to file");
    
            writer.write(self.as_code_depth(1).as_bytes()).expect("Unable to write to file");
    
            writer.write(b",\n\twidth: ").expect("Unable to write to file");
            writer.write(format!("{}", self.get_width()).as_bytes()).expect("Unable to write to file");
            writer.write(b",\n\theight: ").expect("Unable to write to file");
            writer.write(format!("{}", self.get_height()).as_bytes()).expect("Unable to write to file");
    
            writer.write(br#",
    };
    
    #[wasm_bindgen]
    pub fn debug_mode() {
        utils::set_panic_hook();
    }
    
    #[wasm_bindgen]
    pub fn run() {
        interpret(PROGRAM, prompt, print_to_console);
    }
    
    #[wasm_bindgen]
    pub fn execute_code(code: String) {
        let program = DynGrid::<Block>::from(code);
        interpret(program, prompt, print_to_console);
    }
    "#).expect("Unable to write to file");
        }
    
        let status = Command::new("wasm-pack")
            .arg("build")
            .current_dir("./inner")
            .status()
            .expect("Unexpected error while compiling wasm");
    
        assert!(status.success());

        return String::new();
    }
}

pub struct GridDebugger<G: Grid<Block>, I: FnMut(&str) -> String, O: FnMut(String)> {
    pub grid: G,
    input: I,
    output: O,
    pub state: GridState,
    pub break_points: Vec<(usize, usize)>,
}

pub trait Debugger<G, I, O>
where
    G: Grid<Block>,
    I: FnMut(&str) -> String,
    O: FnMut(String) -> (),
{
    fn new(grid: G, input: I, output: O, break_points: Vec<(usize, usize)>) -> Self;

    fn step(&mut self) -> Result<(), u8>;

    fn run(&mut self) -> Result<(), u8>;
}

impl<G, I, O> Debugger<G, I, O> for GridDebugger<G, I, O>
where
    G: Grid<Block>,
    I: FnMut(&str) -> String,
    O: FnMut(String) -> (),
{
    fn new(grid: G, input: I, output: O, break_points: Vec<(usize, usize)>) -> Self {
        let mut state = GridState {
            dir: Direction::Down,
            pos: 0,
            val: 0,
            storage: [0; STORAGE_SIZE],
            coords: grid.find_start().expect("No start found")
        };
        
        if let Some(Block::Start(d)) = grid.get_pos(state.coords) {
            state.dir = *d;
        } else {
            panic!("No start block found");
        }
    
        return GridDebugger {
            grid,
            input,
            output,
            state,
            break_points
        };
    }

    fn step(&mut self) -> Result<(), u8> {
        if let Some(block) = self.grid.get_pos(self.state.coords) {
            block.execute(&mut self.state, &mut self.input, &mut self.output);
        }

        self.state.walk(self.grid.get_width(), self.grid.get_height())
    }

    fn run(&mut self) -> Result<(), u8> {
        loop {
            if let Some(block) = self.grid.get_pos(self.state.coords) {
                block.execute(&mut self.state, &mut self.input, &mut self.output);
            }

            self.state.walk(self.grid.get_width(), self.grid.get_height())?;

            if self.break_points.contains(&self.state.coords) {
                return Ok(());
            }
        }
    }   
}