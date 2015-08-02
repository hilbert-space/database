//! Structured query language.

struct Buffer(Vec<String>);

impl Buffer {
    fn new() -> Buffer {
        Buffer(vec![])
    }

    fn push<T: ToString>(&mut self, chunk: T) -> &mut Self {
        self.0.push(chunk.to_string());
        self
    }

    fn join(self, delimiter: &str) -> String {
        let mut result = String::new();
        for (i, ref chunk) in self.0.iter().enumerate() {
            if i > 0 {
                result.push_str(delimiter)
            }
            result.push_str(chunk);
        }
        result
    }

    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}

macro_rules! take(
    ($from:ident, $what:ident) => (
        match $from.$what.take() {
            Some(value) => value,
            _ => raise!(concat!("expected `", stringify!($what), "` to be set")),
        }
    );
);

macro_rules! shortcut(
    ($(#[$comment:meta] $from:ident -> $into:ident,)*) => (
        $(
            #[inline]
            #[$comment]
            pub fn $from() -> $into {
                $into::new()
            }
        )*
    );
);

pub mod statement;
