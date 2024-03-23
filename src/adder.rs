pub fn add(x: i32, y: i32) -> i32 {
    print!("{} + {} = ", x, y);
    x + y
}

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// use hello_world::adder::add_one;
/// let arg = 5;
/// let answer = add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}