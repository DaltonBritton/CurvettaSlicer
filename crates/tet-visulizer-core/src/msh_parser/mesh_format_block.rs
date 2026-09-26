use nom::{
    Parser,
    branch::alt,
    bytes::complete::tag,
    character::{
        self,
        complete::{line_ending, space1},
    },
    combinator::{map, value},
    error::ParseError,
    number::complete::double,
};

use crate::msh_parser::block;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeshFileType {
    Ascii,
    Binary,
}

pub struct MeshFormat {
    file_version: f64,
    file_type: MeshFileType,
    data_size: u8,
}

fn parse_format_type<'a, E: ParseError<&'a [u8]>>()
-> impl Parser<&'a [u8], Output = MeshFileType, Error = E> {
    alt((
        value(MeshFileType::Ascii, tag("0")),
        value(MeshFileType::Binary, tag("1")),
    ))
}

fn parse_data_size<'a, E: ParseError<&'a [u8]>>() -> impl Parser<&'a [u8], Output = u8, Error = E> {
    character::complete::u8
}

fn msh_header_line<'a, E: ParseError<&'a [u8]>>()
-> impl Parser<&'a [u8], Output = MeshFormat, Error = E> {
    map(
        (
            double,
            space1,
            parse_format_type(),
            space1,
            parse_data_size(),
            line_ending,
        ),
        |(file_version, _, file_type, _, data_size, _)| MeshFormat {
            file_version,
            file_type,
            data_size,
        },
    )
}

pub fn parser<'a, E: ParseError<&'a [u8]>>() -> impl Parser<&'a [u8], Output = MeshFormat, Error = E>
{
    block::block("MeshFormat", msh_header_line())
}

#[cfg(test)]
mod tests {
    use super::*;

    type TestErr<'a> = nom::error::Error<&'a [u8]>;

    #[test]
    fn parse_format_type_test() {
        let (_, ascii) = parse_format_type::<TestErr>()
            .parse_complete("0".as_bytes())
            .unwrap();
        let (_, bin) = parse_format_type::<TestErr>()
            .parse_complete("1".as_bytes())
            .unwrap();

        assert_eq!(ascii, MeshFileType::Ascii);
        assert_eq!(bin, MeshFileType::Binary);
    }

    #[test]
    fn parse_data_size_test() {
        let (_, a) = parse_data_size::<TestErr>()
            .parse_complete("0".as_bytes())
            .unwrap();
        let (_, b) = parse_data_size::<TestErr>()
            .parse_complete("4".as_bytes())
            .unwrap();
        let (_, c) = parse_data_size::<TestErr>()
            .parse_complete("8".as_bytes())
            .unwrap();

        assert_eq!(a, 0);
        assert_eq!(b, 4);
        assert_eq!(c, 8);
    }

    #[test]
    fn parse_mesh_header_test() {
        let header_data = "$MeshFormat\n2.2 0 8\n$EndMeshFormat\n".as_bytes();

        let (_, header) = parser::<TestErr>().parse_complete(header_data).unwrap();

        assert_eq!(header.file_version, 2.2);
        assert_eq!(header.file_type, MeshFileType::Ascii);
        assert_eq!(header.data_size, 8);
    }
}
