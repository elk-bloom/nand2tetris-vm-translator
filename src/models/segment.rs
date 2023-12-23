use std::str::FromStr;

use crate::errors::parse_segment_error::ParseSegmentError;

pub enum Segment {
    Argument,
    Local,
    Static,
    Constant,
    This,
    That,
    Pointer,
    Temp,
}

impl Segment {
    pub fn to_assembly(&self) -> String {
        match self {
            Segment::Argument => "ARG".to_string(),
            Segment::Local => "LCL".to_string(),
            Segment::This => "THIS".to_string(),
            Segment::That => "THAT".to_string(),
            _ => todo!(),
        }
    }
}

impl FromStr for Segment {
    type Err = ParseSegmentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "argument" => Ok(Segment::Argument),
            "local" => Ok(Segment::Local),
            "static" => Ok(Segment::Static),
            "constant" => Ok(Segment::Constant),
            "this" => Ok(Segment::This),
            "that" => Ok(Segment::That),
            "pointer" => Ok(Segment::Pointer),
            "temp" => Ok(Segment::Temp),
            _ => Err(ParseSegmentError {
                segment: s.to_string(),
            }),
        }
    }
}
