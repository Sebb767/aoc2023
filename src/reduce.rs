trait Reduce<T> {
    fn reduce<F>(&mut self, reduce_function: F) -> Option<T>
    where
        F: Fn(T, T) -> T;

    fn reduce_with_start_value<F>(&mut self, start_value: T, reduce_function: F) -> T
    where
        F: Fn(T, T) -> T;
}

impl<T, I> Reduce<T> for I
where
    I: Iterator<Item = T>,
{
    fn reduce<F>(&mut self, reduce_function: F) -> Option<T>
    where
        F: Fn(T, T) -> T,
    {
        let item = self.next()?;
        Some(self.reduce_with_start_value(item, reduce_function))
    }

    fn reduce_with_start_value<F>(&mut self, mut start_value: T, reduce_function: F) -> T
    where
        F: Fn(T, T) -> T,
    {
        for next in self.by_ref() {
            start_value = reduce_function(start_value, next);
        }
        start_value
    }
}
