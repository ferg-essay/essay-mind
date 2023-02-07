use pyo3::prelude::*;

use ticker::{TickerSystem, TickerSystemBuilder, Ticker, FiberId, Fiber, FiberBuilder, TickerBuilder};
use ticker::test_thread;

type KeyArgs = (String,f32,f32);

struct FnWrapper {
    cb : Py<PyAny>,
}

impl FnWrapper {
    fn call(&self, id: usize, key: &String, value: f32, p: f32) {
        match Python::with_gil(|py|->PyResult<Py<PyAny>> {
            self.cb.call1(py, (id, key, value, p))
        }) {
            Ok(_v) => { }
            Err(err) => { panic!("{}", err); }
        }
    }
}

#[pyclass(unsendable)]
pub struct FiberKeyRust {
   pub id: FiberId,
   fiber: Fiber<KeyArgs>
}

#[pymethods]
impl FiberKeyRust {
   fn __call__(&self, key:String, value : f32, p : f32) {
       self.fiber.send((key, value, p));
   }

   fn __str__(&self) -> PyResult<String> {
       Ok(format!("FiberKeyRust({},{})", self.fiber.id.id, self.fiber.id.name))
   }

}

#[pyclass(unsendable)]
pub struct TickerRust {
   ticker : Ticker<KeyArgs>,
}

#[pymethods]
impl TickerRust {
   fn __str__(&self) -> PyResult<String> {
       Ok(format!("Ticker[{}]", self.ticker.name))
   }
}

#[pyclass(unsendable)]
pub struct FiberKeyBuilderRust {
    builder: FiberBuilder<KeyArgs>,
}

#[pymethods]
impl FiberKeyBuilderRust {
    fn name(&mut self, name: &str) {
        self.builder.name(name);
    }

    fn fiber(&mut self) -> PyResult<FiberKeyRust> {
        let fiber = self.builder.fiber();

        Ok(FiberKeyRust { id: fiber.id.clone(), fiber: fiber, })
    }
}

#[pyclass(unsendable)]
pub struct TickerBuilderRust {
    builder: TickerBuilder<KeyArgs>,
}
struct OnTickWrapper {
    on_tick : Py<PyAny>,
}

impl OnTickWrapper {
    fn call(&self, ticks: u32) {
        match Python::with_gil(|py|->PyResult<Py<PyAny>> {
            self.on_tick.call1(py, (ticks, ))
        }) {
            Ok(_v) => { }
            Err(err) => { panic!("{}", err); }
        }
    }
}

#[pymethods]
impl TickerBuilderRust {
    fn name(&mut self, name: &str) {
        self.builder.name(name);
    }

    fn on_tick(&mut self, on_tick_py: &PyAny) {
        let on_tick = OnTickWrapper { on_tick: on_tick_py.into(), };
        
        self.builder.on_tick(Box::new(move |ticks: u32| on_tick.call(ticks)));

    }

    fn on_fiber(&mut self, fiber: &FiberKeyBuilderRust, on_fiber_py: &PyAny) {
        let on_fiber = FnWrapper { cb: on_fiber_py.into() };
        
        self.builder.on_fiber(&fiber.builder, 
            Box::new(move |fiber_id : &FiberId, args: &KeyArgs| on_fiber.call(fiber_id.id, &args.0, args.1, args.2)));
    }
}


#[pyclass(unsendable)]
pub struct TickerSystemRust {
   system: TickerSystem<KeyArgs>,
}

#[pymethods]
impl TickerSystemRust {
    pub fn ticks(&self) -> u32 {
        self.system.ticks()
    }

    pub fn tick(&self) {
        self.system.tick();
    }
}

#[pyclass(unsendable)]
pub struct TickerSystemBuilderRust {
    builder : TickerSystemBuilder<KeyArgs>,
}

#[pymethods]
impl TickerSystemBuilderRust {
    #[new]
    pub fn new() -> PyResult<TickerSystemBuilderRust> {
        Ok(Self { builder: TickerSystemBuilder::new() })
    }

    pub fn fiber(&mut self, name: Option<&str>) -> PyResult<FiberKeyBuilderRust> {
        let mut fiber = FiberKeyBuilderRust { builder: self.builder.fiber() };

        match &name {
            Some(name) => { fiber.name(name); }
            _ => {}
        }

        Ok(fiber)
    }

    pub fn ticker(&mut self, name: Option<&str>) -> PyResult<TickerBuilderRust> {
        let mut ticker = TickerBuilderRust { builder: self.builder.ticker() };

        match &name {
            Some(name) => { ticker.name(name); }
            _ => {}
        }

        Ok(ticker)
    }

    pub fn build(&mut self) -> TickerSystemRust {
        TickerSystemRust {
            system: self.builder.build()
        }
    }
}

#[pyfunction]
fn test_thread_py()
{
    test_thread();
}

/// A Python module implemented in Rust.
#[pymodule]
fn _essaymind(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(test_thread_py, m)?)?;
    m.add_class::<TickerSystemBuilderRust>()?;

    Ok(())
}