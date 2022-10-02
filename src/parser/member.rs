use nom::IResult;

use crate::{Member, Parser};

impl Parser<Member> for Member {
    fn parse(input: &str) -> IResult<&str, Member> {
        // TODO
        Ok((
            input,
            Member::Stringifer(crate::Stringifer { ext_attrs: vec![] }),
        ))
    }
}
