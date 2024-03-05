use std::fmt;


use nalgebra::{DMatrix, DVector};

use crate::formating::as_f64_formated;

#[derive(Debug)]
pub struct Equation {
    pub matrix: DMatrix<f64>,
    pub res_vec: DVector<f64>,
    pub solve: Option<DVector<f64>>,
    pub max_iterations: usize,
    pub epsilon: f64,
}

#[derive(Debug)]
pub enum SolveError {
    ZerosOnDiagonal,
    OutOfMaxIterations(DVector<f64>),
    NotDiagonalDominant
}

impl fmt::Display for SolveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZerosOnDiagonal => write!(f, "Zeroes on diagonal found"),
            Self::OutOfMaxIterations(solve) => write!(f, "Maximum iteration limit reached, reached solve value: {}", solve.map(|x| as_f64_formated(x))),
            Self::NotDiagonalDominant => write!(f, "Matrix is not diagonal dominant")
        }
    }
}

impl Equation {

    pub fn try_solve(&mut self) -> Result<&Self, SolveError> {

        // println!("Init matrix: {}", self.matrix);

        if !self.is_dominant_diagonal() {
            return Err(SolveError::NotDiagonalDominant);
        }

        self.as_simple_iterations()?.as_zero_solved();

        // println!("Matrix in simple iteration format: {}", self.matrix);

        let mut new_iteration_solve: DVector<f64> = self.solve.as_ref().unwrap().clone();

        for _ in 0..self.max_iterations {
            
            new_iteration_solve = self.get_next_solve();

            if self.is_accurate_enough(&new_iteration_solve) {
                self.solve = Some(new_iteration_solve);
                return Ok(self);
            } else {
                self.solve = Some(new_iteration_solve);
            }
        }

        Err(SolveError::OutOfMaxIterations(self.solve.as_ref().unwrap().clone()))
        // Ok(self)
    }

    fn is_dominant_diagonal(&self) -> bool {

        for i in 0..self.res_vec.len() {
            if self.matrix[(i, i)].abs() < self.matrix
                                                .row(i)
                                                .iter()
                                                .enumerate()
                                                .map(|(j, &value)| if j != i {value.abs()} else {0.})
                                                .sum() {
                                                    return false;
                                                }
        }
        true
    }

    fn as_simple_iterations(&mut self) -> Result<&mut Self, SolveError> {
        let matrix_size = self.matrix.column_iter().count();

        for i in 0..matrix_size {

            let diagonal_el = self.matrix[(i, i)];

            if diagonal_el == 0. {return Err(SolveError::ZerosOnDiagonal);}

            for j in 0..matrix_size {
                self.matrix[(i, j)] = if i != j {
                        -self.matrix[(i, j)] / diagonal_el
                    } else {
                        0.
                    }
            }

            self.res_vec[i] /= diagonal_el;
        };
        Ok(self)
    }

    pub fn as_zero_solved(&mut self) -> &Self {
        self.solve = Some(vec![0.; self.matrix.shape().0].into());
        self
    }

    fn get_next_solve(&self) -> DVector<f64> {

        let mut new_iteration_solve = self.solve.as_ref().unwrap().clone().map(|_| 0.);
        let self_solve = self.solve.as_ref().unwrap().clone();

        for i in 0..new_iteration_solve.len() {
            for j in 0..new_iteration_solve.len() {
                new_iteration_solve[i] += self_solve[j] * self.matrix[(i, j)];
            }
            new_iteration_solve[i] += self.res_vec[i];
        }
        // println!("New solve vector: {}", new_iteration_solve);
        new_iteration_solve
    }

    fn is_accurate_enough(&self, new_solve: &DVector<f64>) -> bool {
    
        let mut difference: Vec<f64> = Vec::new();
    
        for i in 0..new_solve.len() {
            difference.push(new_solve[i]-self.solve.as_ref().unwrap()[i]);
        }
    
        difference.iter()
            .map(|x| x.abs())
            .fold(f64::NAN, f64::max)
            <= self.epsilon
    }
}