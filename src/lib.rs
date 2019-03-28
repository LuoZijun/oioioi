
use std::fmt;


pub struct SpannedError<T: fmt::Debug + fmt::Display> {
    inner: T,
    filename: &'static str,
    module_path: &'static str,
    line: usize,
    column: usize,
}

impl<T: fmt::Debug + fmt::Display> SpannedError<T> {
    pub fn new(inner: T,
               filename: &'static str,
               module_path: &'static str,
               line: usize,
               column: usize) -> Self {
        Self { inner, filename, module_path, line, column, }
    }
}


impl<T: fmt::Debug + fmt::Display> fmt::Display for SpannedError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<T: fmt::Debug + fmt::Display> fmt::Debug for SpannedError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}:{}:{} #{}: {:?}", self.filename, self.line, self.column, self.module_path, self.inner)
    }
}


#[macro_export]
macro_rules! spanned_err {
    ($e:expr) => (
        Err($crate::SpannedError::new($e,
                              file!(),
                              module_path!(), 
                              line!() as usize,
                              column!() as usize))
    )
}




// pub fn ttt() -> Result<(), Error> {
//     spanned_err!(MyError::new("又一个错误 :D"))
// }