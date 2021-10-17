pub(crate) fn join<T>(it: T, sep: &str) -> Option<String>
where
    T: IntoIterator<Item = String>,
{
    it.into_iter().reduce(|a, b| (a + sep + &b))
}
