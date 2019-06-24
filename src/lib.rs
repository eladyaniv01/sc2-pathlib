use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pathfinding::prelude::{absdiff, astar};
use std::time::Instant;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);
//static SQRT2: f32 = 1.4142135623730950488016887242097;
static SQRT2: u32 = 14142;
static MULT: u32 = 10000;

impl Pos {
    fn distance(&self, other: &Pos) -> u32 {
        (absdiff(self.0, other.0) + absdiff(self.1, other.1)) as u32
    }

  fn successors(&self, grid: &Vec<Vec<u32>>) -> Vec<(Pos, u32)> {
        let &Pos(x, y) = self;
        let mut arr = Vec::<(Pos, u32)>::with_capacity(8);
        //let arr = Vec<(Pos, f32)>();

        let mut val_left = 0;
        let mut val_down = 0;
        let mut val_right = 0;
        let mut val_up = 0;

        if x > 0 {
            val_left =  grid[x - 1][y];
        }

        if y > 0 {
            val_down = grid[x][y - 1];
        }

        if x + 1 < grid.len() {
            val_right = grid[x + 1][y];
        }

        if y + 1 < grid[0].len() {
            val_up = grid[x][y + 1];
        }

        if val_left > 0
        {
            arr.push((Pos(x - 1, y), val_left * MULT));

            if val_down > 0
            {
                let diag_val = grid[x - 1][y - 1];

                if diag_val > 0 {
                    arr.push((Pos(x - 1, y - 1), diag_val * SQRT2));
                }
            }

            if val_up > 0
            {
                let diag_val = grid[x - 1][y + 1];

                if diag_val > 0 {
                    arr.push((Pos(x - 1, y + 1), diag_val * SQRT2));
                }
            }
        }

        if val_right > 0
        {
            arr.push((Pos(x + 1, y), val_right * MULT));

            if val_down > 0
            {
                let diag_val = grid[x + 1][y - 1];

                if diag_val > 0 {
                    arr.push((Pos(x + 1, y - 1), diag_val * SQRT2));
                }
            }

            if val_up > 0
            {
                let diag_val = grid[x + 1][y + 1];

                if diag_val > 0 {
                    arr.push((Pos(x + 1, y + 1), diag_val * SQRT2));
                }
            }
        }

        if val_up > 0
        {
            arr.push((Pos(x, y + 1), val_up * MULT));
        }

        if val_down > 0
        {
            arr.push((Pos(x, y - 1), val_down * MULT));
        }

        arr.clone()
    }
}

#[pyfunction]
/// Formats the sum of two numbers as string
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
/// Tests a path and returns a string defining the tested path
fn debug_path(grid: Vec<Vec<u32>>, start_x: usize, start_y: usize, x: usize, y: usize) -> PyResult<String> {
    let now = Instant::now();
    let start: Pos = Pos(start_x, start_y);
    let goal: Pos = Pos(x, y);
    
    let result = astar(&start, |p| p.successors(&grid), |p| p.distance(&goal) / 3, |p| *p == goal);
    //assert_eq!(result.expect("no path found").len(), 5);
    let unwrapped = result.unwrap();
    let path = unwrapped.0;
    let distance = unwrapped.1;

    let time_taken = now.elapsed().as_micros();
    let steps = path.len().to_string();
    let mut path_text = String::new();
    for step in path {
        path_text.push_str(&format!("{},{} ", step.0, step.1));
    }
    Ok(format!("time taken: {}µs len: {} distance: {} start: {},{} goal: {},{} Path: {}", time_taken, steps, distance, start_x, start_y, x, y, &path_text))
}


#[pyfunction]
/// Find the path and returns the path
fn find_path(grid: Vec<Vec<u32>>, start: (usize, usize), end: (usize, usize)) -> PyResult<(Vec<(usize, usize)>, u32)> {
    let start: Pos = Pos(start.0, start.1);
    let goal: Pos = Pos(end.0, end.1);

    let result = astar(&start, |p| p.successors(&grid), |p| p.distance(&goal), |p| *p == goal);
    
    let mut path: Vec<(usize, usize)>;
    let distance: u32;

    if result.is_none(){
        path = Vec::<(usize, usize)>::new();
        distance = 0
    }
    else {
        let unwrapped = result.unwrap();
        distance = unwrapped.1;
        path = Vec::<(usize, usize)>::with_capacity(unwrapped.0.len());
        for pos in unwrapped.0 {
            path.push((pos.0, pos.1))    
        }
    }
    
    Ok((path, distance))
}

/// This module is a python module implemented in Rust.
#[pymodule]
fn sc2pathlib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(debug_path))?;
    m.add_wrapped(wrap_pyfunction!(find_path))?;

    Ok(())
}