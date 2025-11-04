mod emitter;
mod parser;
mod rules;
mod scanner;

#[macro_export]
macro_rules! current_bloq_mut {
    ($bloqs:expr) => {
        $bloqs.last_mut().unwrap()
    };
}
