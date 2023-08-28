use std::ops::{RangeFrom, RangeTo};

use nom::{
    error::{ErrorKind, ParseError},
    AsBytes, Compare, CompareResult, Err, ExtendInto, FindSubstring, FindToken, IResult, InputIter,
    InputLength, InputTake, InputTakeAtPosition, Needed, Offset, ParseTo, Slice,
};

#[derive(Debug, Clone)]
pub struct WebIDLInput<'a, T> {
    pub input: T,

    /// Keep track of the definition that is currently being parsed.
    pub definition: Option<&'a str>,
}

impl<'a> From<&'a str> for WebIDLInput<'a, &'a str> {
    fn from(input: &'a str) -> Self {
        let definition = None;

        Self { input, definition }
    }
}

impl<'a, T> AsBytes for WebIDLInput<'a, T>
where
    T: AsBytes,
{
    fn as_bytes(&self) -> &[u8] {
        self.input.as_bytes()
    }
}

impl<'a, T, U> Compare<U> for WebIDLInput<'a, T>
where
    T: Compare<U>,
{
    fn compare(&self, t: U) -> CompareResult {
        self.input.compare(t)
    }

    fn compare_no_case(&self, t: U) -> CompareResult {
        self.input.compare_no_case(t)
    }
}

impl<'a, T> ExtendInto for WebIDLInput<'a, T>
where
    T: ExtendInto,
{
    type Extender = T::Extender;
    type Item = T::Item;

    fn extend_into(&self, acc: &mut Self::Extender) {
        self.input.extend_into(acc);
    }

    fn new_builder(&self) -> Self::Extender {
        self.input.new_builder()
    }
}

impl<'a, T, U> FindSubstring<U> for WebIDLInput<'a, T>
where
    T: FindSubstring<U>,
{
    fn find_substring(&self, substr: U) -> Option<usize> {
        self.input.find_substring(substr)
    }
}

impl<'a, T, U> FindToken<U> for WebIDLInput<'a, T>
where
    T: FindToken<U>,
{
    fn find_token(&self, token: U) -> bool {
        self.input.find_token(token)
    }
}

impl<'a, T> InputIter for WebIDLInput<'a, T>
where
    T: InputIter,
{
    type Item = T::Item;
    type Iter = T::Iter;
    type IterElem = T::IterElem;

    fn iter_elements(&self) -> Self::IterElem {
        self.input.iter_elements()
    }

    fn iter_indices(&self) -> Self::Iter {
        self.input.iter_indices()
    }

    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.input.position(predicate)
    }

    fn slice_index(&self, count: usize) -> Result<usize, Needed> {
        self.input.slice_index(count)
    }
}

impl<'a, T> InputLength for WebIDLInput<'a, T>
where
    T: InputLength,
{
    fn input_len(&self) -> usize {
        self.input.input_len()
    }
}

impl<'a, T> InputTake for WebIDLInput<'a, T>
where
    T: InputTake,
    Self: Slice<RangeFrom<usize>> + Slice<RangeTo<usize>>,
{
    fn take(&self, count: usize) -> Self {
        self.slice(..count)
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        (self.slice(count..), self.slice(..count))
    }
}

impl<'a, T> InputTakeAtPosition for WebIDLInput<'a, T>
where
    T: InputTakeAtPosition + InputIter + InputTake + InputLength,
    Self: Slice<RangeFrom<usize>> + Slice<RangeTo<usize>> + Clone,
{
    type Item = <T as InputIter>::Item;

    fn split_at_position<P, E: ParseError<Self>>(&self, predicate: P) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.input.position(predicate) {
            Some(idx) => Ok(self.take_split(idx)),
            None => Err(Err::Incomplete(nom::Needed::new(1))),
        }
    }

    fn split_at_position1<P, E: ParseError<Self>>(
        &self,
        predicate: P,
        e: ErrorKind,
    ) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.input.position(predicate) {
            Some(0) => Err(Err::Error(E::from_error_kind(self.clone(), e))),
            Some(idx) => Ok(self.take_split(idx)),
            None => Err(Err::Incomplete(nom::Needed::new(1))),
        }
    }

    fn split_at_position1_complete<P, E: ParseError<Self>>(
        &self,
        predicate: P,
        e: ErrorKind,
    ) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.input.position(predicate) {
            Some(0) => Err(Err::Error(E::from_error_kind(self.clone(), e))),
            Some(idx) => Ok(self.take_split(idx)),
            None if self.input.input_len() == 0 => {
                Err(Err::Error(E::from_error_kind(self.clone(), e)))
            }
            None => Ok(self.take_split(self.input_len())),
        }
    }

    fn split_at_position_complete<P, E: ParseError<Self>>(
        &self,
        predicate: P,
    ) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.split_at_position(predicate) {
            Err(Err::Incomplete(_)) => Ok(self.take_split(self.input_len())),
            res => res,
        }
    }
}

impl<'a, T> Offset for WebIDLInput<'a, T>
where
    T: Offset,
{
    fn offset(&self, second: &Self) -> usize {
        self.input.offset(&second.input)
    }
}

impl<'a, T, R> ParseTo<R> for WebIDLInput<'a, T>
where
    T: ParseTo<R>,
{
    fn parse_to(&self) -> Option<R> {
        self.input.parse_to()
    }
}

impl<'a, T, R> Slice<R> for WebIDLInput<'a, T>
where
    T: Slice<R>,
{
    fn slice(&self, range: R) -> Self {
        let input = self.input.slice(range);
        let definition = self.definition;

        Self { input, definition }
    }
}