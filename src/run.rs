pub struct Run;

impl Run {
    pub fn new() {
        // TODO this is where the program should start
        println!("Program Started");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_has_std_practices() {
        Run::new();
        assert!(true);
    }
}
