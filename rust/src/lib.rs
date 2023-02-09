use pyo3::prelude::*;

use ticker::{TickerSystem, SystemBuilder, Fiber, FiberBuilder, TickerBuilder, OnFiberFn};
use ticker::test_thread;
extern crate env_logger;
use log::info;

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
   fiber: Fiber<KeyArgs>
}

#[pymethods]
impl FiberKeyRust {
   fn __call__(&self, key:String, value : f32, p : f32) {
        info!("call {}", key);
        self.fiber.send((key, value, p));
   }

   /*
   fn __str__(&self) -> PyResult<String> {
       Ok(format!("FiberKeyRust({},{})", self.fiber.id.id, self.fiber.id.name))
   }
   */
}

/* 
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
*/

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

        Ok(FiberKeyRust { fiber: fiber, })
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
    fn call(&self, ticks: u64) {
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

    pub fn fiber(&mut self, name: Option<&str>) -> PyResult<FiberKeyBuilderRust> {
        let mut fiber = FiberKeyBuilderRust { builder: self.builder.fiber() };

        match &name {
            Some(name) => { fiber.name(name); }
            _ => {}
        }

        Ok(fiber)
    }

    fn on_tick(&mut self, on_tick_py: &PyAny) {
        let on_tick = OnTickWrapper { on_tick: on_tick_py.into(), };
        
        self.builder.on_tick(Box::new(move |ticks: u64| on_tick.call(ticks)));

    }

    fn on_fiber(&mut self, fiber: &mut FiberKeyBuilderRust, on_fiber_py: &PyAny) {
        let on_fiber = FnWrapper { cb: on_fiber_py.into() };
        
        let fun: Box<OnFiberFn<KeyArgs>> = Box::new(
            move |from_ticker: usize, args: KeyArgs| 
            on_fiber.call(from_ticker, &args.0, args.1, args.2)
        );
        info!("on_fiber");
        self.builder.on_fiber(&mut fiber.builder, Box::new(fun));
    }
}


#[pyclass(unsendable)]
pub struct TickerSystemRust {
   system: TickerSystem<KeyArgs>,
}

#[pymethods]
impl TickerSystemRust {
    pub fn ticks(&self) -> u64 {
        self.system.ticks()
    }

    pub fn tick(&mut self) {
        self.system.tick();
    }
}

#[pyclass(unsendable)]
pub struct TickerSystemBuilderRust {
    builder : SystemBuilder<KeyArgs>,
}

#[pymethods]
impl TickerSystemBuilderRust {
    #[new]
    pub fn new() -> PyResult<TickerSystemBuilderRust> {
        Ok(Self { builder: SystemBuilder::new() })
    }

    pub fn ticker(&mut self, name: Option<&str>) -> PyResult<TickerBuilderRust> {
        let mut ticker = TickerBuilderRust { builder: self.builder.ticker() };

        match &name {
            Some(name) => { ticker.name(name); }
            _ => {}
        }

        Ok(ticker)
    }

    pub fn external_fiber(&mut self, name: Option<&str>) -> PyResult<FiberKeyBuilderRust> {
        let mut fiber = FiberKeyBuilderRust { builder: self.builder.external_fiber() };

        match &name {
            Some(name) => { fiber.name(name); }
            _ => {}
        }

        Ok(fiber)
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
    env_logger::Builder::from_env(
        env_logger::Env::default()
        .default_filter_or("info")
    )
    .format_timestamp(None)
    .init();
    info!("logging-more");
    //log!("logging");



    m.add_function(wrap_pyfunction!(test_thread_py, m)?)?;
    m.add_class::<TickerSystemBuilderRust>()?;

    Ok(())
}