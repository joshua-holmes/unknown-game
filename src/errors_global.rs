use std::fmt::Debug;

pub trait GameErrorHandling {
    /// Function for handling errors within the game. This function should take
    /// into account any possible error that could be passed in and either exit
    /// the application with a message or return a default value for whatever
    /// is calling the function to use.
    fn handle_error<T, E: Debug>(error: E) -> T;
}
