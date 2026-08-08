pub fn compare_opt<T>(one: Option<T>, two: T) -> bool
where
    T: Eq,
{
    if let Some(one_unwrapped) = one
        && one_unwrapped == two
    {
        return true;
    }
    false
}
