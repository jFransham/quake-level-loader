pub struct Cache<T>(Vec<Weak<T>>);

impl<T> Cache<T> {
    pub fn get_or_insert<P, I>(&self, p: P, f: I) -> Rc<T>
        where
            P: FnMut(&Weak<T>) -> Ordering,
            I: FnOnce() -> T
    {
        let out_or_insert_loc = self.0
            .binary_search_by(
                |probe|
                p
            );
    }
}
