use std::time::{Duration, Instant};

pub trait Solved {
    fn solve(&self, input: &str) -> Result<String, String>;

    fn solve_timed(&self, input: &str) -> Result<(String, Duration), String> {
        let t0 = Instant::now();
        let output = self.solve(input)?;
        let t1 = Instant::now();

        Ok((output, t1.duration_since(t0)))
    }

    fn test(&self, input: &str, output: &str) -> Result<bool, String> {
        let solve_output = self.solve(input)?;
        Ok(&solve_output == output)
    }

    fn print_timed(&self, input: &str) {
        match self.solve_timed(input) {
            Ok((output, duration)) => {
                let runtime = duration.as_micros() as f32 / 1000.0;
                println!("Solved in {:.3}ms, answer:", runtime);
                println!("{output}");
            },
            Err(error) => {
                println!("Failed with error:");
                println!("{error}");
            },
        }
    }
}

impl<F> Solved for F 
    where F: Fn(&str) -> Result<String, String>
{
    fn solve(&self, input: &str) -> Result<String, String> {
        self(input)
    }
}