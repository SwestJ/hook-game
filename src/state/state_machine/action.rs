pub trait Action: Sized {
    type OkOutput;
    type ErrOutput;
    fn execute<T: Execute<Self>>(self, state: T) -> Result<T::OkState, T::ErrState>;
}

pub trait Execute<T: Action>: Sized {
    fn prepare(&self) -> T;
    fn execute(self) -> Result<Self::OkState, Self::ErrState> {
        self.prepare().execute(self)
    }
    type OkState;
    fn move_to_ok_state(self, output: T::OkOutput) -> Self::OkState;
    type ErrState;
    fn move_to_err_state(self, output: T::ErrOutput) -> Self::ErrState;
}