/**
 * Domain Errors: Enums for domain-specific failures.
 */

#[derive(Debug)]
#[non_exhaustive]
pub enum AppError {
    InvalidInput(&'static str, &'static str),
    NotFound(&'static str, &'static str),
    Aborted(&'static str, String),
}
