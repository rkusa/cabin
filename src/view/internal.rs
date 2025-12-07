pub struct Internal<T>(State<T>);

enum State<T> {
    Builder(T),
    Error(crate::Error),
}

impl<T> Internal<T> {
    pub fn new(builder: T) -> Self {
        Self(State::Builder(builder))
    }

    pub fn error(error: crate::Error) -> Self {
        Self(State::Error(error))
    }

    pub fn builder_mut<'a>(&'a mut self) -> Option<&'a mut T> {
        match &mut self.0 {
            State::Builder(builder) => Some(builder),
            State::Error(_) => None,
        }
    }

    pub fn take_builder(self) -> Result<T, crate::Error> {
        match self.0 {
            State::Builder(builder) => Ok(builder),
            State::Error(err) => Err(err),
        }
    }

    pub fn errored(&mut self, error: crate::Error) {
        self.0 = State::Error(error);
    }
}
