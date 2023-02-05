use std::cell::RefCell;
use std::fmt;

use pyo3::prelude::*;

use core_lib::FiberIdBuilder;
use core_lib::FiberId;
use core_lib::Fiber;
use core_lib::FiberBuilder;
use core_lib::MindNode;
use core_lib::FiberFn;

/// Formats the sum of two numbers as string.
//#[pyfunction]
//fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
//    test_core();
// Ok((a + b).to_string())
//}

#[pyclass]
pub struct FiberIdBuilderRust {
    builder : FiberIdBuilder,
}

#[pymethods]
impl FiberIdBuilderRust {
    #[new]
    pub fn new() -> PyResult<Self> {
        Ok(FiberIdBuilderRust { builder: FiberIdBuilder::new()})
    }

    //pub fn fiber_key(&mut self, s:&str) -> PyResult<FiberKey> {
    //    let fiber = Fiber::new(&mut self.builder, s);
    //
    //    Ok(FiberKey { fiber, })
    //}
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

    fn wrap(cb: &PyAny)->FnWrapper
    {
        //let _: Py<PyAny> = cb.into();

        Self {
            cb: cb.into(),
        }
    }
}

type KeyArgs = (String,f32,f32);
type KeyFn = FiberFn<KeyArgs>;

#[pyclass]
pub struct FiberKeyBuilderRust {
    builder: FiberBuilder<KeyArgs>,

//    to: Vec<Box<KeyFn>>,
}

//unsafe impl Send for FiberKey {
//
//}
/*
impl fmt::Display for FiberKeyRust {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FiberKeyRust[{},{}]", self.fiber.id.id, self.fiber.id.name)
    }
}
*/

#[pymethods]
impl FiberKeyBuilderRust {
    #[new]
    fn new(mut builder_ref: PyRefMut<FiberIdBuilderRust>, name: &str) -> PyResult<FiberKeyBuilderRust> {
        Ok(Self {
            builder: builder_ref.builder.fiber(name),
        })
    //fn new(builder_ref: &PyAny, name: &str) -> PyResult<FiberKeyRust> {
    /*
        Python::with_gil(|py| {
            //let obj: &PyAny = builder_ref.into_ref(py);
            //let bb_cell : &PyCell<FiberBuilderRust> = builder_ref.downcast(py)?;
            //let bb: &FiberBuilderRust = bb_cell.borrow().py();
            //let bb: &FiberBuilderRust = bb_cell.borrow().py();
            //let bb: &FiberBuilderRust = builder_ref.extract(py)?;
            //let bb_mut: FiberBuilder = bb_cell.borrow_mut().into();
            //let bb: &mut FiberBuilderImpl = bb_cell.borrow_mut().builder?;
            //let bb: &FiberBuilderRust = obj.extract();
            //let b: &FiberBuilder = &bb.builder;

            //let obj: &PyAny = builder_ref.into_ref(py);
            //let mut bb: PyRefMut<FiberBuilderRust> = obj.extract()?;

            let mut bb: PyRefMut<FiberBuilderRust> = builder_ref.extract()?;

            Ok(Self {
                fiber : Fiber::new(&mut bb.builder, name),
            })
        })
        */
    }

    fn to(&mut self, cb: &PyAny)
    {
        let cb_safe = FnWrapper::wrap(cb);
        
        self.builder.to(Box::new(move |fiber_id, args| cb_safe.call(fiber_id, &args.0, args.1, args.2)));
        //self.to.push(cb_safe);
        //self.to.push(Box::new(move |fiber_id, args| cb_safe.call(fiber_id, &args.0, args.1, args.2)));
    }

    fn build(&mut self) -> FiberKeyRust
    {
        FiberKeyRust { fiber : self.builder.build() }        
    }
}

 #[pyclass]
 pub struct FiberKeyRust {
    fiber: Fiber<KeyArgs>
}

#[pymethods]
impl FiberKeyRust {
    fn __call__(&self, key:String, value : f32, p : f32) {
        self.fiber.send(&(key, value, p));
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("FiberKeyRust({},{})", self.fiber.id.id, self.fiber.id.name))
    }

}

#[pyclass]
pub struct MindNodeRust {
    _node : MindNode,
}

#[pymethods]
impl MindNodeRust {
    #[new]
    pub fn new(s : &str) -> PyResult<Self> {
        Ok(MindNodeRust { _node: MindNode { name : String::from(s) }})
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("MindNode[{}]", self._node.name))
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn _essaymind(_py: Python, m: &PyModule) -> PyResult<()> {
    //m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<FiberIdBuilderRust>()?;
    m.add_class::<MindNodeRust>()?;
    m.add_class::<FiberKeyBuilderRust>()?;

    //add_node_functions(m)?;
    Ok(())
}