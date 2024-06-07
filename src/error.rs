use thiserror::Error;

#[derive(Error, Debug)]
pub enum MetadataError {
    #[error("item ({item}) is null")]
    NullItem { item: String },
    #[error("attribute ({attr}) is null")]
    NullAttribute { attr: String },
    // TODO: implement a way to see the type the user wanted
    #[error("failed to get ({attr}) as (desired_type). value is of type ({real_type})")]
    ConversionError {
        attr: String,
        // expected_type: String,
        real_type: String,
    },
    #[error("unknown data store error")]
    Unknown,
}
