use std::cell::RefCell;
use std::boxed::FnBox;
use std::ops::Deref;
use std::mem::transmute;
use std::mem::replace;

pub struct Lazy<T, F = Box<FnBox() -> T>> where F: FnOnce() -> T {
    store: RefCell<LazyStorage<T, F>>,
}

enum LazyStorage<T, F> where F: FnOnce() -> T {
    Func(F),
    Stored(Option<T>),
}

impl<T, F> LazyStorage<T, F> where F: FnOnce() -> T {
    fn as_ref(&self) -> Option<&T> {
        match *self {
            LazyStorage::Stored(Some(ref t)) => Some(t),
            _ => None,
        }
    }

    fn consume(&mut self) -> Option<T> {
        match *self {
            LazyStorage::Stored(ref mut opt) => replace(opt, None),
            _ => None,
        }
    }
}

impl<T, F> Lazy<T, F> where F: FnOnce() -> T {
    pub fn new(f: F) -> Lazy<T, F> {
        Lazy {
            store: RefCell::new(LazyStorage::Func(f)),
        }
    }

    pub fn consume(self) -> T {
        self.consume_fn();

        self.store.borrow_mut().consume().unwrap()
    }

    fn consume_fn(&self) {
        use self::LazyStorage::*;

        if let Stored(_) = *self.store.borrow() {
            return;
        }

        let f = match replace(
            &mut *self.store.borrow_mut(),
            Stored(None)
        ) {
            Func(f) => f,
            _ => unreachable!(),
        };

        *self.store.borrow_mut() = Stored(Some(f()));
    }
}

impl<T, F> Deref for Lazy<T, F> where F: FnOnce() -> T {
    type Target = T;

    fn deref(&self) -> &T {
        self.consume_fn();
        unsafe { transmute(self.store.borrow().as_ref().unwrap()) }
    }
}
