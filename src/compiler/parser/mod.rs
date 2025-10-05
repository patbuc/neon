mod emitter;
mod parser;
mod rules;
mod scanner;

#[macro_export]
macro_rules! current_brick_mut {
    ($bricks:expr) => {
        $bricks.last_mut().unwrap()
    };
}
