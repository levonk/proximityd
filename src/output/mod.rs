pub mod aggregates;
pub mod empty;
pub mod schema;
pub mod suggestions;
pub mod toon;
pub mod truncation;

pub use aggregates::{ListAggregate, PartyAggregate, DeviceAggregate, SystemAggregate};
pub use empty::{EmptyContext, EmptyFormatter};
pub use schema::{CommandField, OutputSchema, PartyOutput, DeviceOutput, StatusOutput};
pub use suggestions::{Suggestion, SuggestionContext, SuggestionEngine, format_suggestions_toon, format_suggestions_human};
pub use toon::{ToonEncoder, ToonDecoder, ToonValue, ToonError};
pub use truncation::{truncate, truncate_text, truncate_with_limit, TruncatedText, TruncationConfig, DEFAULT_TRUNCATION_LIMIT};
