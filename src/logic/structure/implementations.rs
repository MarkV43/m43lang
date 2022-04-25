use std::fmt::Debug;
pub use m43lang_derive::AsCode;
use super::*;

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