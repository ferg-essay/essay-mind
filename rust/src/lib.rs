use pyo3::prelude::*;

use ticker::{TickerSystem, TickerSystemBuilder, Ticker, FiberId, Fiber, FiberBuilder, TickerBuilder};

type KeyArgs = (String,f32,f32);

struct FnWrapper {
    cb : Py<PyAny>,
}

impl FnWrapper {
    fn call(&self, id: i32, key: &String, value: f32, p: f32) {
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
       self.fiber.send(&(key, value, p));
   }

   fn __str__(&self) -> PyResult<String> {
       Ok(format!("FiberKeyRust({},{})", self.fiber.id.id, self.fiber.id.name))
   }

}

#[pyclass(unsendable)]
pub struct TickerRust {
   ticker : Ticker,
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
    fn to(&mut self, ticker: &TickerBuilderRust, cb: &PyAny) {
        let cb_safe = FnWrapper { cb: cb.into() };
        
        self.builder.to(&ticker.builder, 
            Box::new(move |fiber_id : &FiberId, args: &KeyArgs| cb_safe.call(fiber_id.id, &args.0, args.1, args.2)));

    }

    fn fiber(&mut self) -> PyResult<FiberKeyRust> {
        let fiber = self.builder.fiber();

        Ok(FiberKeyRust { id: fiber.id.clone(), fiber: fiber, })
    }
}

#[pyclass(unsendable)]
pub struct TickerBuilderRust {
    builder: TickerBuilder,
}
struct OnTickWrapper {
    on_tick : Py<PyAny>,
}

impl OnTickWrapper {
    fn call(&self, ticks: i32) {
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
    fn on_tick(&mut self, on_tick_py: &PyAny) {
        let on_tick = OnTickWrapper { on_tick: on_tick_py.into(), };
        
        self.builder.on_tick(Box::new(move |ticks: i32| on_tick.call(ticks)));

    }
}


#[pyclass(unsendable)]
pub struct TickerSystemRust {
   system: TickerSystem,
}

#[pymethods]
impl TickerSystemRust {
    pub fn ticks(&self) -> i32 {
        self.system.ticks()
    }

    pub fn tick(&self) {
        self.system.tick();
    }
}

#[pyclass(unsendable)]
pub struct TickerSystemBuilderRust {
    builder : TickerSystemBuilder,
}

#[pymethods]
impl TickerSystemBuilderRust {
    #[new]
    pub fn new() -> PyResult<TickerSystemBuilderRust> {
        Ok(Self { builder: TickerSystemBuilder::new() })
    }

    pub fn fiber(&mut self, name: &str)->PyResult<FiberKeyBuilderRust> {
        Ok(FiberKeyBuilderRust { builder: self.builder.fiber(name) })
    }

    pub fn ticker(&mut self, name: &str)->PyResult<TickerBuilderRust> {
        Ok(TickerBuilderRust { builder: self.builder.ticker(name) })
    }

    pub fn build(&mut self) -> TickerSystemRust {
        TickerSystemRust {
            system: self.builder.build()
        }
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn _essaymind(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<TickerSystemBuilderRust>()?;

    Ok(())
}