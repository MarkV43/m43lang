use std::fmt::Debug;
use std::io::{Write, BufWriter};
use std::fs::OpenOptions;
use std::process::Command;
use std::str::FromStr;
pub use m43lang_derive::AsCode;
use m43lang_derive::Decodable;

pub trait AsCode {
    fn as_code(&self) -> String;

    fn as_code_depth(&self, _depth: u8) -> String {
        self.as_code()
    }
}

impl AsCode for usize {
    fn as_code(&self) -> String {
        format!("{}", self)
    }
}

impl AsCode for u64 {
    fn as_code(&self) -> String {
        format!("{}", self)
    }
}

impl<T: AsCode> AsCode for Option<T> {
    fn as_code(&self) -> String {
        match self {
            Some(x) => format!("Some({})", x.as_code()),
            None => "None".to_string(),
        }
    }
}

pub trait Decodable {
    fn decode<'a, I: Iterator<Item = &'a str>>(iter: &mut I) -> Self;

    fn treat_inp(inp: &str) -> &str {
        if inp.chars().last().unwrap_or_default() == ')' {
            &inp[..inp.len() - 1]
        } else {
            inp
        }
    }
}

impl<F, E> Decodable for F
where 
    F: FromStr<Err = E>,
    E: Debug,
{
    fn decode<'a, I: Iterator<Item = &'a str>>(iter: &mut I) -> Self {
        Self::treat_inp(iter.next().unwrap()).parse().unwrap()
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, AsCode, Decodable)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/* impl AsCode for Direction {
    fn as_code(&self) -> String {
        match self {
            Direction::Up => "Direction::Up",
            Direction::Down => "Direction::Down",
            Direction::Left => "Direction::Left",
            Direction::Right => "Direction::Right",
        }.to_string()
    }
} */

type Index = usize;
type Value = u64;

pub trait IsStart {
    fn is_start(&self) -> bool;
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, AsCode, Decodable)]
pub enum Block {
    Start(Direction),
    Redirect(Direction),
    Store,
    Load,
    MoveRight(Index),
    MoveLeft(Index),
    Goto(Index),
    Set(Value),
    Save(Value),
    OpAdd,
    OpSub,
    OpMul,
    OpDiv,
    CompLarger,
    CompSmaller,
    CompEqual,
    Conditional(Direction, Direction),
    Display,
    Print,
    Break,
    Input,
    End,
}

/* impl Block {
    pub fn decode<'a, I: Iterator<Item = &'a str>>(iter: &mut I) -> Option<Self> {
        match iter.next().unwrap() {
            "Start" => Some(Block::Start(match iter.next().unwrap().chars().next().unwrap() {
                'U' => Direction::Up,
                'D' => Direction::Down,
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!("Invalid direction"),
            })),
            "Redirect" => Some(Block::Redirect(match iter.next().unwrap().chars().next().unwrap() {
                'U' => Direction::Up,
                'D' => Direction::Down,
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!("Invalid direction"),
            })),
            "Store" => Some(Block::Store),
            "Load" => Some(Block::Load),
            "MoveRight" => Some(Block::MoveRight(treat_inp(iter.next().unwrap()).parse().unwrap())),
            "MoveLeft" => Some(Block::MoveLeft(treat_inp(iter.next().unwrap()).parse().unwrap())),
            "Goto" => Some(Block::Goto(treat_inp(iter.next().unwrap()).parse().unwrap())),
            "Set" => Some(Block::Set(treat_inp(iter.next().unwrap()).parse().unwrap())),
            "Save" => Some(Block::Save(treat_inp(iter.next().unwrap()).parse().unwrap())),
            "OpAdd" => Some(Block::OpAdd),
            "OpSub" => Some(Block::OpSub),
            "OpMul" => Some(Block::OpMul),
            "OpDiv" => Some(Block::OpDiv),
            "CompLarger" => Some(Block::CompLarger),
            "CompSmaller" => Some(Block::CompSmaller),
            "CompEqual" => Some(Block::CompEqual),
            "Conditional" => Some(Block::Conditional(match iter.next().unwrap().chars().next().unwrap() {
                'U' => Direction::Up,
                'D' => Direction::Down,
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!("Invalid direction"),
            }, match iter.next().unwrap().chars().next().unwrap() {
                'U' => Direction::Up,
                'D' => Direction::Down,
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!("Invalid direction"),
            })),
            "Display" => Some(Block::Display),
            "Print" => Some(Block::Print),
            "Break" => Some(Block::Break),
            "Input" => Some(Block::Input),
            "End" => Some(Block::End),
            _ => panic!("Invalid block"),
        }
    }
} */

/* impl AsCode for Block {
    fn as_code(&self) -> String {
        match self {
            Block::Start(dir) => format!("Block::Start({})", dir.as_code()),
            Block::Redirect(dir) => format!("Block::Redirect({})", dir.as_code()),
            Block::Store => "Block::Store".to_string(),
            Block::Load => "Block::Load".to_string(),
            Block::MoveRight(i) => format!("Block::MoveRight({})", i),
            Block::MoveLeft(i) => format!("Block::MoveLeft({})", i),
            Block::Goto(i) => format!("Block::Goto({})", i),
            Block::Set(value) => format!("Block::Set({})", value),
            Block::Save(value) => format!("Block::Save({})", value),
            Block::OpAdd => "Block::OpAdd".to_string(),
            Block::OpSub => "Block::OpSub".to_string(),
            Block::OpMul => "Block::OpMul".to_string(),
            Block::OpDiv => "Block::OpDiv".to_string(),
            Block::CompLarger => "Block::CompLarger".to_string(),
            Block::CompSmaller => "Block::CompSmaller".to_string(),
            Block::CompEqual => "Block::CompEqual".to_string(),
            Block::Conditional(dir1, dir2) => format!("Block::Conditional({}, {})", dir1.as_code(), dir2.as_code()),
            Block::Display => "Block::Display".to_string(),
            Block::Print => "Block::Print".to_string(),
            Block::Break => "Block::Break".to_string(),
            Block::Input => "Block::Input".to_string(),
            Block::End => "Block::End".to_string(),
        }
    }
} */

impl IsStart for Block {
    fn is_start(&self) -> bool {
        match self {
            Block::Start(_) => true,
            _ => false,
        }
    }
}

pub trait Grid<T>: From<Vec<Option<T>>> + AsCode {
    fn new(width: usize, height: usize) -> Self;
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;
    fn get_cells(&self) -> &[Option<T>];
    fn get_cells_mut(&mut self) -> &mut [Option<T>];
    fn get(&self, x: usize, y: usize) -> &Option<T> {
        if x < self.get_width() && y < self.get_height() {
            &self.get_cells()[y * self.get_width() + x]
        } else {
            &None
        }
    }
    fn set(&mut self, value: T, x: usize, y: usize) {
        if x < self.get_width() && y < self.get_height() {
            let ind = y * self.get_width() + x;
            self.get_cells_mut()[ind] = Some(value);
        }
    }
    fn reshape(&mut self, width: usize, height: usize)
    where T: Clone;
    fn find(&self, val: T) -> Option<(usize, usize)>
    where T: Eq {
        for y in 0..self.get_height() {
            for x in 0..self.get_width() {
                if let Some(f) = self.get(x, y) {
                    if val == *f {
                        return Some((x, y));
                    }
                }
            }
        }
        None
    }
    fn find_start(&self) -> Option<(usize, usize)>
    where T: IsStart {
        for y in 0..self.get_height() {
            for x in 0..self.get_width() {
                if let Some(f) = self.get(x, y) {
                    if f.is_start() {
                        return Some((x, y));
                    }
                }
            }
        }
        None
    }
    fn to_string(&self) -> String
    where T: Debug {
        let mut s = String::new();
        for y in 0..self.get_height() {
            for x in 0..self.get_width() {
                if let Some(f) = self.get(x, y) {
                    s.push_str(&format!("{:?} ", f));
                } else {
                    s.push_str("_ ");
                }
            }
            s.push_str("\n");
        }
        s
    }
}

#[derive(Debug)]
pub struct ConstGrid<T, const S: usize> {
    pub cells: [Option<T>; S],
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Clone)]
pub struct DynGrid<T> {
    cells: Vec<Option<T>>,
    width: usize,
    height: usize,
}

impl<T: AsCode + Copy + Debug, const S: usize> Grid<T> for ConstGrid<T, S> {
    fn new(width: usize, height: usize) -> Self
    where T: Clone {
        Self {
            cells: [None; S],
            width,
            height,
        }
    }

    fn get_width(&self) -> usize {
        self.width
    }

    fn get_height(&self) -> usize {
        self.height
    }

    fn get_cells(&self) -> &[Option<T>] {
        &self.cells
    }

    fn get_cells_mut(&mut self) -> &mut [Option<T>] {
        &mut self.cells
    }

    fn reshape(&mut self, width: usize, height: usize)
    where T: Clone {
        assert_eq!(width * height, S);

        self.width = width;
        self.height = height;
    }
}

impl<T: AsCode + Copy + Debug, const S: usize> AsCode for ConstGrid<T, S> {
    fn as_code(&self) -> String {
        let mut s = String::from('[');
        for y in 0..self.height {
            s.push('[');
            for x in 0..self.width {
                s.push_str(&self.get(x, y).as_code());
                s.push_str(",");
            }
            s.push_str("],");
        }
        s.push(']');
        s
    }

    fn as_code_depth(&self, depth: u8) -> String {
        let k = (0..depth).map(|_| '\t').collect::<String>();
        let mut s = "[\n".to_string();
        for y in 0..self.height {
            for x in 0..self.width {
                s.push_str(k.as_str());
                s.push_str("\t");
                s.push_str(&self.get(x, y).as_code_depth(depth));
                s.push_str(",\n");
            }
        }
        s.push_str(k.as_str());
        s.push(']');
        s
    }
}

impl<T: AsCode + Copy + Debug> Grid<T> for DynGrid<T> {
    fn new(width: usize, height: usize) -> Self
    where T: Clone {
        Self {
            cells: vec![None; width * height],
            width,
            height,
        }
    }

    fn get_width(&self) -> usize {
        self.width
    }

    fn get_height(&self) -> usize {
        self.height
    }

    fn get_cells(&self) -> &[Option<T>] {
        &self.cells
    }

    fn get_cells_mut(&mut self) -> &mut [Option<T>] {
        &mut self.cells
    }

    fn reshape(&mut self, width: usize, height: usize)
    where T: Clone {
        assert_eq!(width * height, self.width * self.height);

        self.width = width;
        self.height = height;
    }
}

impl<T: AsCode + Copy + Debug> AsCode for DynGrid<T> {
    fn as_code(&self) -> String {
        let mut s = String::from('[');
        for y in 0..self.height {
            s.push('[');
            for x in 0..self.width {
                s.push_str(&self.get(x, y).as_code());
                s.push_str(",");
            }
            s.push_str("],");
        }
        s.push(']');
        s
    }

    fn as_code_depth(&self, depth: u8) -> String {
        let k = (0..depth).map(|_| '\t').collect::<String>();
        let mut s = "[\n".to_string();
        for y in 0..self.height {
            for x in 0..self.width {
                s.push_str(k.as_str());
                s.push_str("\t");
                s.push_str(&self.get(x, y).as_code_depth(depth));
                s.push_str(",\n");
            }
        }
        s.push_str(k.as_str());
        s.push(']');
        s
    }
}

impl<T: AsCode + Copy + Debug> DynGrid<T> {
    fn optimize(&mut self) {
        let mut rows = Vec::with_capacity(self.get_height());
        let mut cols = Vec::with_capacity(self.get_width());

        // Determine the empty rows
        for y in 0..self.get_height() {
            let mut empty = true;
            for x in 0..self.get_width() {
                if let Some(_) = self.get(x, y) {
                    empty = false;
                    break;
                }
            }
            if empty {
                rows.push(y);
            }
        }
        // Determine the empty cols
        for x in 0..self.get_width() {
            let mut empty = true;
            for y in 0..self.get_height() {
                if let Some(_) = self.get(x, y) {
                    empty = false;
                    break;
                }
            }
            if empty {
                cols.push(x);
            }
        }

        let w = self.get_width() - cols.len();
        let h = self.get_height() - rows.len();
        let mut new_cells = Vec::with_capacity(w * h);
        for y in 0..self.get_height() {
            if let Err(_) = rows.binary_search(&y) {
                for x in 0..self.get_width() {
                    if let Err(_) = cols.binary_search(&x) {
                        new_cells.push(self.get(x, y).clone());
                    }
                }
            }
        }

        self.cells = new_cells;
        self.width = w;
        self.height = h;
    }
}

impl<T: Copy, const S: usize> From<Vec<Vec<Option<T>>>> for ConstGrid<T, S> {
    fn from(vec: Vec<Vec<Option<T>>>) -> Self {
        let width = vec.first().unwrap().len();
        let height = vec.len();

        assert_eq!(width * height, S);

        let mut cells = [None; S];

        for y in 0..height {
            for x in 0..width {
                cells[y * width + x] = vec[y][x];
            }
        }
        
        Self {
            cells,
            width,
            height,
        }
    }
}

impl<T: Debug, const S: usize> From<Vec<Option<T>>> for ConstGrid<T, S> {
    fn from(vec: Vec<Option<T>>) -> Self {
        let width = vec.len();
        let height = 1;

        assert_eq!(width * height, S);
        
        Self {
            cells: vec.try_into().expect("Could not convert vec to grid"),
            width,
            height,
        }
    }
}

impl<T: Clone + Debug, const S: usize> From<&[Option<T>]> for ConstGrid<T, S> {
    fn from(vec: &[Option<T>]) -> Self {
        let width = vec.len();
        let height = 1;

        assert_eq!(width * height, S);
        
        Self {
            cells: vec
                .into_iter()
                .cloned()
                .collect::<Vec<_>>()
                .try_into()
                .expect("Could not convert vec to grid"),
            width,
            height,
        }
    }
}

impl<const S: usize> From<String> for ConstGrid<Block, S> {
    fn from(str: String) -> Self {
        let lines = str
            .lines()
            .into_iter()
            .map(|x| {
                x
                    .split(' ')
                    .map(|k| {
                        if k.len() <= 1 && k.chars().next().unwrap() == '_' {
                            None
                        } else {
                            let mut iter = k.split(&['(', ','][..]);
                            Some(Block::decode(&mut iter))
                        }
                    }).collect::<Vec<_>>()
            }).collect::<Vec<_>>();

        Self::from(lines)
    }
}

impl<T: Clone + Debug, const S: usize> From<[Option<T>; S]> for ConstGrid<T, S> {
    fn from(cells: [Option<T>; S]) -> Self {
        let width = cells.len();
        let height = 1;

        assert_eq!(width * height, S);
        
        Self {
            cells,
            width,
            height,
        }
    }
}

impl<L, T, const S: usize> From<(L, usize)> for ConstGrid<T, S>
where
    ConstGrid<T, S>: From<L>,
    T: AsCode + Debug + Clone + Copy
{
    fn from((list, width): (L, usize)) -> Self {
        let mut grid = ConstGrid::from(list);

        let height = grid.get_width() / width;
        assert_eq!(width * height, S);

        grid.reshape(width, height);
        
        grid
    }
}

impl<T: Debug> From<Vec<Option<T>>> for DynGrid<T> {
    fn from(vec: Vec<Option<T>>) -> Self {
        let width = vec.len();
        let height = 1;
        
        Self {
            cells: vec.try_into().expect("Could not convert vec to grid"),
            width,
            height,
        }
    }
}

impl<T: Clone + Debug> From<&[Option<T>]> for DynGrid<T> {
    fn from(vec: &[Option<T>]) -> Self {
        let width = vec.len();
        let height = 1;
        
        Self {
            cells: vec
                .into_iter()
                .cloned()
                .collect::<Vec<_>>()
                .try_into()
                .expect("Could not convert vec to grid"),
            width,
            height,
        }
    }
}

impl From<String> for DynGrid<Block> {
    fn from(str: String) -> Self {
        let lines = str
            .lines()
            .into_iter()
            .map(|x| {
                x
                    .split(' ')
                    .map(|k| {
                        if k.len() <= 1 && k.chars().next().unwrap() == '_' {
                            None
                        } else {
                            let mut iter = k.split(&['(', ','][..]);
                            Some(Block::decode(&mut iter))
                        }
                    }).collect::<Vec<_>>()
            }).collect::<Vec<_>>();

        Self::from(lines)
    }
}

impl<T: Copy> From<Vec<Vec<Option<T>>>> for DynGrid<T> {
    fn from(vec: Vec<Vec<Option<T>>>) -> Self {
        let height = vec.len();
        if height == 0 {
            panic!("Cannot create grid from empty vec");
        }
        let width = vec.first().unwrap().len();
        if width == 0 {
            panic!("Cannot create grid from empty vec");
        }

        let mut cells = vec![None; width * height];

        for y in 0..height {
            for x in 0..width {
                cells[y * width + x] = vec[y][x];
            }
        }
        
        Self {
            cells,
            width,
            height,
        }
    }
}

impl<T: Clone + Debug, const S: usize> From<[Option<T>; S]> for DynGrid<T> {
    fn from(cells: [Option<T>; S]) -> Self {
        let width = cells.len();
        let height = 1;

        assert_eq!(width * height, S);
        
        Self {
            cells: cells.try_into().expect("Could not convert vec to grid"),
            width,
            height,
        }
    }
}

impl<L, T> From<(L, usize)> for DynGrid<T>
where
    DynGrid<T>: From<L>,
    T: AsCode + Debug + Clone + Copy
{
    fn from((list, width): (L, usize)) -> Self {
        let mut grid = DynGrid::from(list);
        
        let height = grid.get_width() / width;
        
        grid.reshape(width, height);
        
        grid
    }
}

pub fn interpret<F: Fn(&str) -> String, O: Fn(String) -> (), G: Grid<Block>>(code: G, input: F, output: O) -> u8 {
    let mut pos = code.find_start().expect("No start found");
    let mut register = 0u64;
    let mut storage = [0u64; 256];
    let mut reg_pos = 0;
    let mut block = Block::End;
    let mut direction;
    
    if let Some(Block::Start(d)) = code.get(pos.0, pos.1) {
        direction = d.clone();
    } else {
        panic!("No start block found");
    }

    let mut none;
    while {
        let b = code.get(pos.0, pos.1).clone();
        none = b.is_none();
        if none {
            true
        } else {
            block = b.unwrap();
            block != Block::End
        }
     } {
        if !none {
            match block.clone() {
                Block::Start(_) => {},
                Block::Redirect(d) => direction = d,
                Block::Store => storage[reg_pos] = register,
                Block::Load => register = storage[reg_pos],
                Block::MoveRight(n) => reg_pos += n,
                Block::MoveLeft(n) => reg_pos -= n,
                Block::Goto(n) => reg_pos = n,
                Block::Set(v) => register = v,
                Block::Save(n) => storage[reg_pos] = n,
                Block::OpAdd => register += storage[reg_pos],
                Block::OpSub => register -= storage[reg_pos],
                Block::OpMul => register *= storage[reg_pos],
                Block::OpDiv => register /= storage[reg_pos],
                Block::CompLarger => register = if register > storage[reg_pos] { 1 } else { 0 },
                Block::CompSmaller => register = if register < storage[reg_pos] { 1 } else { 0 },
                Block::CompEqual => register = if register == storage[reg_pos] { 1 } else { 0 },
                Block::Conditional(d1, d2) => direction = if register == 0 { d2 } else { d1 },
                Block::Display => output(format!("{}", register)),
                Block::Print => output(format!("{}", register as u8 as char)),
                Block::Break => output("\n".to_string()),
                Block::Input => register = input(&format!("{}", register)).parse().unwrap(),
                Block::End => break,
            }
        }

        match direction {
            Direction::Up => {
                if pos.1 == 0 {
                    return 1;
                }
                pos.1 -= 1;
            }
            Direction::Down => {
                if pos.1 == code.get_height() - 1 {
                    return 1;
                }
                pos.1 += 1;
            }
            Direction::Left => {
                if pos.0 == 0 {
                    return 1;
                }
                pos.0 -= 1;
            }
            Direction::Right => {
                if pos.0 == code.get_width() - 1 {
                    return 1;
                }
                pos.0 += 1;
            }
        }
    }

    return 0;
}

pub fn compile(mut code: DynGrid<Block>) {
    code.optimize();

    println!("{}", code.to_string());

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
        write!(writer, "{}", code.get_cells().len()).expect("Unable to write to file");
        writer.write(br#"> = ConstGrid {
    cells: "#).expect("Unable to write to file");

        writer.write(code.as_code_depth(1).as_bytes()).expect("Unable to write to file");

        writer.write(b",\n\twidth: ").expect("Unable to write to file");
        writer.write(format!("{}", code.get_width()).as_bytes()).expect("Unable to write to file");
        writer.write(b",\n\theight: ").expect("Unable to write to file");
        writer.write(format!("{}", code.get_height()).as_bytes()).expect("Unable to write to file");

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
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROGRAM: &'static str = "Redirect(D) Store Set(1) Goto(4) Start(L) _ _ _ _ _ _ _ _ _ _
Goto(0) _ _ _ _ _ _ _ _ _ _ _ _ _ _
Set(64) _ _ _ _ _ _ _ _ _ _ _ _ _ _
Store _ _ _ _ _ _ _ _ _ _ _ _ _ _
MoveRight(2) _ _ _ _ _ _ _ _ _ _ _ _ _ _
Store _ _ _ _ _ _ _ _ _ _ _ _ _ _
Redirect(R) MoveLeft(1) Set(1) Store _ _ _ _ _ _ _ _ Goto(3) Load Redirect(D)
_ Redirect(D) MoveLeft(1) Load MoveRight(2) Store MoveLeft(1) Load MoveLeft(1) Store MoveRight(1) OpAdd MoveRight(1) Display Redirect(L)
_ Store _ _ _ _ _ _ _ _ _ _ _ _ Load
_ _ _ _ _ _ _ _ _ _ _ _ _ _ Goto(3)
_ Redirect(R) _ Goto(0) Load MoveRight(2) Break _ MoveLeft(2) Load MoveRight(1) OpSub MoveLeft(1) Store Conditional(U,D)
_ _ _ _ _ _ _ _ _ _ _ _ _ _ End";

    #[test]
    fn test_compile() {
        let program = DynGrid::<Block>::from(PROGRAM.to_string());
    }
}