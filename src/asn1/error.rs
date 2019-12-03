/// ParseError are returned when there is an error parsing the ASN.1 data.
#[derive(Debug, PartialEq)]
pub enum ParseError {
    /// Something about the value was invalid.
    InvalidValue,
    /// An unexpected tag was encountered.
    UnexpectedTag { actual: u8 },
    /// There was not enough data available to complete parsing.
    ShortData,
    /// An internal computation would have overflowed.
    IntegerOverflow,
    /// There was extraneous data in the input.
    ExtraData,
}
