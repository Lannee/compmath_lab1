use std::fmt;
use nalgebra::{DMatrix, DVector};
use serde::Deserialize;
use serde_json;
use std::fmt::Debug;
use std::io;

use crate::alg_eq::Equation;
use crate::env_args::EnvArgs;


pub enum GetEquationError {
    FileError(std::io::Error),
    ParseError(serde_json::Error),
    InputError(String),
} 

impl fmt::Display for GetEquationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FileError(err) => write!(f, "Error with input file: {}", err),
            Self::ParseError(err) => write!(f, "Invalid parse format: {}", err),
            Self::InputError(message) => write!(f, "Invalid input data: {}", message),
        }
    }
}

#[derive(Deserialize, Debug)]
struct EquesionAsJson {
    pub matrix: Vec<Vec<f64>>,
    pub res_vec: Vec<f64>,
    pub max_iterations: usize,
    pub epsilon: f64,
}

impl std::convert::TryInto<Equation> for EquesionAsJson {
    type Error = GetEquationError;

    fn try_into(self) -> Result<Equation, Self::Error> {

        let res_size = self.res_vec.len();

        if res_size == 0 ||
            self.matrix.len() != res_size ||
            self.matrix[0].len() != res_size {
                return Err(GetEquationError::InputError("Incorrect matrix or result vector dimensions".into()));
            }

        let matrix = 
            DMatrix::from_vec(
                res_size, 
                res_size, 
                self.matrix
                    .iter()
                    .flatten()
                    .map(|&x| x)
                    .collect::<Vec<f64>>()
            ).transpose();

        let res_vec = DVector::from_vec(self.res_vec);

        Ok(Equation {
            matrix,
            res_vec,
            solve: None,
            max_iterations: self.max_iterations,
            epsilon: self.epsilon
        })
    } 
}


pub fn try_get_equation() -> Result<Equation, GetEquationError> {
    let content = get_input().map_err(|err| GetEquationError::FileError(err))?;

    parse_input(&content)?.try_into()
}

fn get_input() -> io::Result<String> {
    let env_args = EnvArgs::get();

    let mut file: Box<dyn io::Read> = match env_args.file_path {
        Some(file_path) => Box::new(std::fs::File::open(file_path)?),
        None => Box::new(io::stdin())
    };

    let mut content = String::new();

    file.read_to_string(&mut content)?;

    Ok(content)
}

fn parse_input(content: &String) -> Result<EquesionAsJson, GetEquationError> {
    serde_json::from_str::<EquesionAsJson>(content).map_err(|err| GetEquationError::ParseError(err))
}