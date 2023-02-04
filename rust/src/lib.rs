use std::fmt;

use pyo3::prelude::*;

use core_lib::test_core;
use core_lib::FiberBuilder as FiberBuilderImpl;
use core_lib::FiberId;
use core_lib::Fiber;
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

    pub fn fiber_key(&mut self, s:&str) -> PyResult<FiberKey> {
        let fiber = Fiber::new(&mut self.builder, s);

        Ok(FiberKey { fiber, })
    }
}

impl fmt::Display for FiberKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FiberKey[{},{}]", self.fiber.id.id, self.fiber.id.name)
    }
}

type KeyArgs = (String,f32,f32);

#[pyclass]
pub struct FiberKey {
    fiber : Fiber<KeyArgs>,
}

struct FnWrapper {
    cb : Py<PyAny>,
}

impl FnWrapper {
    fn call(&self, id: &FiberId, key: &String, value: f32, p: f32) {
        println!("before call");
        match Python::with_gil(|py|->PyResult<Py<PyAny>> {
            self.cb.call1(py, (id.id, key, value, p))
        }) {
            Ok(_v) => { }
            Err(err) => { panic!("{}", err); }
        }
        println!("after call");
    }
}

impl FnWrapper {
    fn wrap(cb: &PyAny)->FnWrapper
    {
        //let _: Py<PyAny> = cb.into();

        Self {
            cb: cb.into(),
        }
    }
}

#[pymethods]
impl FiberKey {
    fn to(&mut self, cb: &PyAny)
    {
        let cb_safe = FnWrapper::wrap(cb);
        
        self.fiber.to(Box::new(move |fiber_id, args| cb_safe.call(fiber_id, &args.0, args.1, args.2)));
    }

    fn __call__(&self, key:String, value : f32, p : f32) {
        self.fiber.send(&(key, value, p));
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("FiberKey({},{})", self.fiber.id.id, self.fiber.id.name))
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
    m.add_class::<FiberKey>()?;

    //add_node_functions(m)?;
    Ok(())
}