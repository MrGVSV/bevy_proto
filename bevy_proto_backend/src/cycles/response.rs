/// Enum used to determine the response to a [`Cycle`].
///
/// [`Cycle`]: crate::cycles::Cycle
pub enum CycleResponse {
    /// The operation that found the cycle is canceled with an error.
    ///
    /// Generally, cycles are found during [prototype] registration,
    /// which means that registration process is canceled.
    ///
    /// [prototype]: crate::proto::Prototypical
    Cancel,
    /// The cycle is ignored and recursion is skipped.
    ///
    /// It's typically not a good idea to ignore cycles outright as they can
    /// indicate a need for improved [prototype] design.
    ///
    /// [prototype]: crate::proto::Prototypical
    Ignore,
    /// The cycle should result in a panic.
    Panic,
}
