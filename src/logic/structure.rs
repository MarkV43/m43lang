pub mod implementations;
pub use implementations::*;

use std::fmt::Debug;
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

pub type Index = usize;
pub type Value = u64;

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

pub trait IsStart {
    fn is_start(&self) -> bool;
}

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
    fn get_pos(&self, pos: (usize, usize)) -> &Option<T> {
        self.get(pos.0, pos.1)
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

impl<T: AsCode + Copy + Debug> DynGrid<T> {
    pub fn optimize(&mut self) {
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