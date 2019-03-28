oioioi
=========


原始版本: https://github.com/casey/oi

.. code:: rust
    
    #[macro_use]
    extern crate oioioi;

    use std::fmt;

    pub type Error = oioioi::SpannedError<MyError>;

    pub struct MyError {
        message: String,
    }

    impl MyError {
        pub fn new<M: Into<String>>(m: M) -> Self {
            Self { message: m.into() }
        }
    }

    impl fmt::Display for MyError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            writeln!(f, "{:?}", self)
        }
    }

    impl fmt::Debug for MyError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            writeln!(f, "{}", self.message)
        }
    }

    fn run() -> Result<(), Error> {
        spanned_err!(MyError::new("又一个错误 :D"))
    }

    fn main() {
        if let Err(e) = run() {
            println!("{:?}", e);
        }
    }


.. code:: text
    
        Compiling oioioi v0.1.0 (/Users/luozijun/Project/oioioi)
          Finished dev [unoptimized + debuginfo] target(s) in 0.89s
            Running `target/debug/examples/oi`
    
    examples/oi.rs:40:5 #oi: 又一个错误 :D

