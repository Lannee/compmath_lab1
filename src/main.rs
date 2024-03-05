use input::try_get_equation;
use formating::as_f64_formated;

mod alg_eq;
mod input;
mod env_args;
mod formating;

fn main() {
    match try_get_equation() {
        Err(e) => println!("{}", e),
        Ok(mut equation) => {
            match equation.try_solve() {
                Err(e) => println!("{}", e),
                Ok(eq) => println!("Result: {}", eq.solve.as_ref().unwrap().map(|x| as_f64_formated(x)))
            };
        }
    };
}
