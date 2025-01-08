/// La structure qui contient les états
pub struct Computer<T> {
    pub state: T,
}

/// Les états
pub struct On;
#[derive(Debug, PartialEq, Eq)]
pub struct Off;

impl Computer<Off> {
    pub fn boot(&self) -> Computer<On> {
        Computer { state: On }
    }
}

impl Computer<On> {
    pub fn shutdown(&self) -> Computer<Off> {
        Computer { state: Off }
    }
}

impl<T> Computer<T> {
    pub fn new() -> Computer<Off> {
        Computer { state: Off }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computer_move_state() {
        let computer_off: Computer<Off> = Computer::<Off>::new();
        let computer_on: Computer<On> = computer_off.boot();
        let computer_off: Computer<Off> = computer_on.shutdown();

        assert_eq!(computer_off.state, Off);
    }
}
