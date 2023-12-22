use std::str::FromStr;

pub struct Segment {
    segment: _Segment,
    segment_assembly_name: String
}

impl FromStr for Segment {
    type Err = ();
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "argument" => Ok(
                Segment{
                    segment: _Segment::Argument,
                     
                }
            )
        }
    }

}

enum _Segment {
   Argument,
   Local,
   Static,
   Constant,
   This,
   That,
   Pointer,
   Temp 
}

