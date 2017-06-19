use std::cell::RefCell;
use std::boxed::FnBox;
use std::ops::Deref;
use std::sync::RwLock;
use std::mem::transmute;
use std::mem::replace;

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

pub struct AsyncLazy<T, F = Box<FnBox() -> T>> where F: FnOnce() -> T {
    store: RwLock<LazyStorage<T, F>>,
}

impl<T, F> AsyncLazy<T, F> where F: FnOnce() -> T {
    pub fn new(f: F) -> AsyncLazy<T, F> {
        AsyncLazy {
            store: RwLock::new(LazyStorage::Func(f)),
        }
    }

    pub fn consume(self) -> Option<T> {
        self.consume_fn();

        self.store
            .write()
            .unwrap()
            .consume()
    }

    fn consume_fn(&self) {
        use self::LazyStorage::*;

        if let Stored(_) = *self.store.read().unwrap() {
            return;
        }

        let f = match replace(
            &mut *self.store.write().unwrap(),
            Stored(None)
        ) {
            Func(f) => f,
            _ => unreachable!(),
        };

        *self.store.write().unwrap() = Stored(Some(f()));
    }
}

impl<T, F> Deref for AsyncLazy<T, F> where F: FnOnce() -> T {
    type Target = T;

    fn deref(&self) -> &T {
        self.consume_fn();
        unsafe {
            transmute(
                self.store
                    .read()
                    .unwrap()
                    .as_ref()
            )
        }
    }
}

pub struct Lazy<T, F = Box<FnBox() -> T>> where F: FnOnce() -> T {
    store: RefCell<LazyStorage<T, F>>,
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
