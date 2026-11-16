use std::{
    cell::{OnceCell, RefCell},
    rc::Rc,
};

pub struct Eval<A>(Rc<_Eval<A>>);

enum _Eval<A> {
    Now(A),
    Later {
        // FnOnce is consumed when OnceCell initializes the value
        thunk: RefCell<Option<Box<dyn FnOnce() -> A>>>,
        cache: OnceCell<A>,
    },
    Always(Box<dyn Fn() -> A>),
    FlatMap(Box<dyn Fn() -> Eval<A>>),
}

impl<A: 'static> Eval<A> {
    // Strict / eager evaluation (equivalent to Scala Cats "Eval.now")
    pub fn now(value: A) -> Self {
        Eval(Rc::new(_Eval::Now(value)))
    }

    /// Lazy, memoized evaluation (equivalent to Scala Cats "Eval.later")
    pub fn later<F>(f: F) -> Self
    where
        F: FnOnce() -> A + 'static,
    {
        Eval(Rc::new(_Eval::Later {
            thunk: RefCell::new(Some(Box::new(f))),
            cache: OnceCell::new(),
        }))
    }

    pub fn always<F>(f: F) -> Self
    where
        F: Fn() -> A + 'static,
    {
        Eval(Rc::new(_Eval::Always(Box::new(f))))
    }

    pub fn value(&self) -> A
    where
        A: Clone,
    {
        match &*self.0 {
            _Eval::Now(a) => a.clone(),
            _Eval::Later { thunk, cache } => cache
                .get_or_init(|| {
                    let f = thunk.borrow_mut().take().expect("thunk already consumed");
                    f()
                })
                .clone(),
            _Eval::Always(f) => f(),
            _Eval::FlatMap(f) => f().value(),
        }
    }

    /// Lazy transformation (preserves laziness via Always)
    pub fn map<B, F>(self, f: F) -> Eval<B>
    where
        B: 'static,
        A: Clone,
        F: Fn(A) -> B + 'static,
    {
        Eval::always(move || f(self.value()))
    }

    /// Monadic bind, with laziness preserved.
    pub fn flat_map<B, F>(self, f: F) -> Eval<B>
    where
        B: 'static,
        A: Clone,
        F: Fn(A) -> Eval<B> + 'static,
    {
        Eval(Rc::new(_Eval::FlatMap(Box::new(move || f(self.value())))))
    }
}

impl<A> Clone for Eval<A> {
    fn clone(&self) -> Self {
        Eval(Rc::clone(&self.0))
    }
}
