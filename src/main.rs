use math::round;

use input::try_get_equation;

mod alg_eq;
mod input;
mod env_args;

fn main() {
    match try_get_equation() {
        Err(e) => println!("{}", e),
        Ok(mut equation) => {
            match equation.try_solve() {
                Err(e) => println!("{}", e),
                Ok(eq) => println!("Result: {}", eq.solve.as_ref().unwrap().map(|x| round::floor(x, 4)))
            };
        }
    };

    // let mut equation = Equation {
    //     matrix: dmatrix![5.5, 1.6, 1.7;
    //                      0.8, 3.4, 0.9;
    //                      2.4, -2.0, -4.5],
    //     res_vec: dvector![1., 3., -1.5],
    //     solve: None,
    //     max_iterations: 10,
    //     epsilon: 0.01
    // };

    // match equation.try_solve() {
    //     Err(e) => println!("{}", e),
    //     Ok(vec) => println!("Result: {:?}", vec.solve.as_ref().unwrap())
    // };
}
