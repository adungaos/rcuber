use thiserror::Error;

/// Errors.
///    Error 1: There is not exactly one facelet of each colour
///    Error 2: Not all 12 edges exist exactly once
///    Error 3: Flip error: One edge has to be flipped
///    Error 4: Not all corners exist exactly once
///    Error 5: Twist error: One corner has to be twisted
///    Error 6: Parity error: Two corners or two edges have to be exchanged
///    Error 7: No solution exists for the given maxDepth
///    Error 8: Probe limit exceeded, no solution within given probMax
#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid color value")]
    InvalidColor,
    #[error("Invalid edge value")]
    InvalidEdge,
    #[error("Invalid corner value")]
    InvalidCorner,
    #[error("Invalid scramble string")]
    InvalidScramble,
    #[error("Invalid facelet string")]
    InvalidFaceletString,
    #[error("Invalid facelet reperesentation")]
    InvalidFaceletValue,
    #[error("Invalid cubie reperesentation")]
    InvalidCubieValue,
    #[error("No solution exists for the given max_depth")]
    NoSolutionForMaxDepth,
    #[error("Probe limit exceeded")]
    ProbeLimitExceeded,
}


