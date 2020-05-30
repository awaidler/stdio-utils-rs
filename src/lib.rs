// Copyright 2020 Andreas Waidler
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::{
    error,
    fmt,
    io,
    num,
};

#[derive(Debug)]
pub struct ParsingError
{
    input: String,
    error: num::ParseIntError,
}

impl fmt::Display for ParsingError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(
            f,
            "Could not parse \"{}\" to number: {:?}",
            self.input, self.error
        )
    }
}

impl error::Error for ParsingError {}

#[derive(Debug)]
pub enum ApplicationError
{
    InputError(io::Error),
    ParsingError(ParsingError),
}

impl From<io::Error> for ApplicationError
{
    fn from(e: io::Error) -> ApplicationError
    {
        ApplicationError::InputError(e)
    }
}

impl From<ParsingError> for ApplicationError
{
    fn from(e: ParsingError) -> ApplicationError
    {
        ApplicationError::ParsingError(e)
    }
}

type Number = isize;

fn as_number(line: &str) -> Result<Number, ParsingError>
{
    // We cannot use From here because ParseIntError
    // does not contain a reference to offending input.
    line.trim().parse().map_err(|err| ParsingError {
        input: String::from(line),
        error: err,
    })
}

pub fn sum<T>(lines: T) -> Result<Number, ApplicationError>
where
    T: Iterator<Item = Result<String, io::Error>>,
{
    lines.map(|line| Ok(as_number(&line?)?)).sum()
}

pub fn sum_strings<'a, T>(strings: T) -> Result<Number, ApplicationError>
where
    T: Iterator<Item = &'a str>,
{
    sum(strings.map(|s| Ok(String::from(s))))
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn parses_a_number()
    {
        assert_eq!(as_number("42").unwrap(), 42);
    }

    #[test]
    fn parses_a_number_with_whitespace()
    {
        assert_eq!(as_number("\t 42\n").unwrap(), 42);
    }

    #[test]
    fn fails_on_invalid_character()
    {
        let result = as_number(bad_input_char());
        let msg = result.unwrap_err().to_string();
        assert!(
            msg.contains(bad_input_char()),
            "Offending character not part of error message \"{}\"",
            msg
        );
    }

    #[test]
    fn fails_on_empty_input()
    {
        let msg = as_number("").unwrap_err().to_string();
        assert!(
            !msg.contains(bad_input_char()),
            "Unexpected (hardcoded?) text in error message \"{}\"",
            msg
        );
    }

    #[test]
    fn empty_stream_returns_zero()
    {
        let stream = std::iter::empty();
        assert_eq!(sum(stream).unwrap(), 0);
    }

    #[test]
    fn single_element_is_equal_to_sum()
    {
        let stream = vec!["42"].into_iter();

        assert_eq!(sum_strings(stream).unwrap(), 42);
    }

    #[test]
    fn sums_two_elements()
    {
        let stream = vec!["39", "30"].into_iter();

        assert_eq!(sum_strings(stream).unwrap(), 69);
    }

    #[test]
    fn propagates_internal_errors()
    {
        let stream = vec![""].into_iter();
        sum_strings(stream).unwrap_err();
    }

    #[test]
    fn propagates_external_errors()
    {
        let stream = vec![Err(create_io_error())].into_iter();
        sum(stream).unwrap_err();
    }

    fn bad_input_char() -> &'static str
    {
        "$"
    }

    fn create_io_error() -> io::Error
    {
        io::Error::new(io::ErrorKind::Other, "Mock Error")
    }
}
