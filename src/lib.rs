pub mod logic;

#[cfg(test)]
mod tests {
    use crate::logic::interpretation::*;

    use super::logic::structure::*;

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
    fn test_read() {
        assert!(std::panic::catch_unwind(|| DynGrid::<Block>::from(PROGRAM.to_string())).is_ok());
    }

    #[test]
    fn test_debug() {
        let grid = DynGrid::<Block>::from(PROGRAM.to_string());
        let mut str = String::new();
        let mut debugger = GridDebugger::new(grid, |_| "0".to_string(), |s| {
            str.push_str(&s);
        }, vec![(0, 0)]);

        debugger.run().expect("Failed to run debugger");
    }
}