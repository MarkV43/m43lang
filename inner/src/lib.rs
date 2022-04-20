mod utils;

use wasm_bindgen::prelude::*;
use m43lang_visual::*;

#[cfg(feature = "wee_allo")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
	fn alert(s: &str);

    fn prompt(s: &str) -> String;

    fn print_to_console(s: String);
}

const PROGRAM: ConstGrid<Block, 15> = ConstGrid {
    cells: [
		Some(Block::Start(Direction::Down)),
		None,
		Some(Block::Redirect(Direction::Right)),
		Some(Block::Redirect(Direction::Down)),
		Some(Block::End),
		Some(Block::Set(43)),
		None,
		Some(Block::Store),
		Some(Block::OpAdd),
		Some(Block::Print),
		Some(Block::Redirect(Direction::Right)),
		Some(Block::Display),
		Some(Block::Redirect(Direction::Up)),
		Some(Block::Redirect(Direction::Right)),
		Some(Block::Redirect(Direction::Up)),
	],
	width: 5,
	height: 3,
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

#[wasm_bindgen]
pub fn get_code_str(code: String) -> String {
	DynGrid::<Block>::from(code).as_code()
}