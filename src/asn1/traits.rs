//@todo add Golang to the license
//@todo move this into error::ParseError
use super::ParseError;
use super::Result as ParseResult;

//@todo an Enum of types and their tags?

pub trait FromASN1: Sized {
    type Error;

    fn from_asn1(data: &[u8]) -> Result<Self, Self::Error>;
}

pub trait ToASN1: Sized {
    //@todo not sure about this result here, but it works for now.
    fn to_asn1(&self) -> Vec<u8>;
}

// ===== Boolean =====

impl FromASN1 for bool {
    type Error = ParseError;

    fn from_asn1(data: &[u8]) -> Result<Self, Self::Error> {
        match data {
            b"0x00" => Ok(false),
            b"0xff" => Ok(true),
            _ => Err(ParseError::InvalidValue),
        }
    }
}

impl ToASN1 for bool {
    fn to_asn1(&self) -> Vec<u8> {
        if *self {
            vec![0xff; 1]
        } else {
            vec![0; 1]
        }
    }
}

// ===== Integer =====
//@todo move these to helpers.
fn check_integer(bytes: &[u8]) -> ParseResult<()> {
    //@todo is this unneeded?
    if bytes.len() == 0 {
        // return StructuralError{"empty integer"} @todo
        Err(ParseError::InvalidValue)
    } else if bytes.len() == 1 {
        Ok(())
    } else if bytes[0] == 0 && bytes[1] & 0x80 == 0 || (bytes[0] == 0xff && bytes[1] & 0x80 == 0x80)
    {
        // return StructuralError{"integer not minimally-encoded"}
        Err(ParseError::InvalidValue)
    } else {
        Ok(())
    }
}

//@todo return/params seem inefficient here. If we are going to return a new vector, we should
//be taking ownership of the byte array, but I'm not sure necessarily how to do that.
fn remove_int_leading_bytes(bytes: &[u8]) -> Vec<u8> {
    let mut start = 0;
    for i in 0..bytes.len() {
        // Removes a leading byte when the first NINE bits are the same

        // If the first 8 bits are not the same, skip
        if bytes[i] != 0x00 && bytes[i] != 0xff {
            break;
        }

        if (bytes[i] & 0x80) != (bytes[i + 1] & 0x80) {
            break;
        }

        // Remove a leading byte.
        start += 1;
    }

    bytes[start..].to_vec()
}

//@todo turn this into a macro please.
impl FromASN1 for u64 {
    type Error = ParseError;

    fn from_asn1(mut bytes: &[u8]) -> Result<Self, Self::Error> {
        check_integer(bytes)?;

        //Error on Negatives
        if bytes[0] & 0x80 == 0x80 {
            return Err(ParseError::InvalidValue);
        }

        //Check for leftover non-negative signing bytes.
        if bytes.len() == 9 && bytes[0] == 0 {
            bytes = &bytes[1..]
        }

        //Overflow
        if bytes.len() > 8 {
            return Err(ParseError::IntegerOverflow);
        }

        let mut fixed = [0; 8];
        fixed[8 - bytes.len()..].copy_from_slice(bytes);
        let mut ret = u64::from_be_bytes(fixed);

        // Shift up and down in order to sign extend the result.
        ret <<= (64) - bytes.len() * 8;
        ret >>= 64 - bytes.len() * 8;

        Ok(ret)
    }
}

//@todo turn this into a macro please.
impl ToASN1 for u64 {
    fn to_asn1(&self) -> Vec<u8> {
        //@todo can we use Vec::with_capacity instead here?
        let mut buf = vec![0_u8; 8];

        for i in 0..buf.len() {
            let shift = 8 * (buf.len() - i - 1);
            let mask = 0xff << shift;
            buf[i] = ((*self & mask) >> shift) as u8;
        }

        remove_int_leading_bytes(&buf)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_u64() {
        let value = 100_u64;

        let asn = value.to_asn1();

        let value_2 = u64::from_asn1(&asn).unwrap();

        assert_eq!(value, value_2);
    }
}
