
//extern crate core_lib;
//use core_lib;
//use py_ext;


//#[macro_use]
//extern crate core_lib;

//pub mod node;
extern crate pyo3;
use pyo3::prelude::*;

//use crate::node::add_node_functions;
use core_lib::test_core;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    test_core();
    Ok((a + b).to_string())
}




/// A Python module implemented in Rust.
#[pymodule]
fn _essaymind(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;

    //add_node_functions(m)?;
    Ok(())
}