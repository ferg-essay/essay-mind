use std::fmt;

use pyo3::prelude::*;

use core_lib::test_core;
use core_lib::FiberBuilder as FiberBuilderImpl;
use core_lib::FiberKey as FiberKeyImpl;
use core_lib::MindNode as MindNodeImpl;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    test_core();
    Ok((a + b).to_string())
}

#[pyclass]
pub struct FiberBuilder {
    builder : FiberBuilderImpl,
}

#[pymethods]
impl FiberBuilder {
    #[new]
    pub fn new() -> PyResult<Self> {
        Ok(FiberBuilder { builder: FiberBuilderImpl::new()})
    }

    pub fn fiber_id(&mut self) -> PyResult<i32> {
        Ok(self.builder.fiber_id())
    }

    pub fn fiber_key(&mut self, s:&str) -> PyResult<FiberKey> {
        Ok(FiberKey { _fiber : self.builder.fiber_key(s) })
    }
}

#[pyclass]
pub struct FiberKey {
    _fiber : FiberKeyImpl,
}

impl fmt::Display for FiberKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FiberKey[{},{}]", self._fiber.id, self._fiber.name)
    }
}

#[pymethods]
impl FiberKey {
    fn __call__(&self, key:&str, value : f32, p : f32) {
        println!("call {} ({}, {}, {})", self, key, value, p)
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("FiberKey({},{})", self._fiber.id, self._fiber.name))
    }

}

#[pyclass]
pub struct MindNode {
    _node : MindNodeImpl,
}

#[pymethods]
impl MindNode {
    #[new]
    pub fn new(s : &str) -> PyResult<Self> {
        Ok(MindNode { _node: MindNodeImpl { name : String::from(s) }})
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("MindNode[{}]", self._node.name))
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn _essaymind(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<FiberBuilder>()?;
    m.add_class::<MindNode>()?;

    //add_node_functions(m)?;
    Ok(())
}