pub type BoxError = Box<dyn std::error::Error>;
use nom::bytes::complete::{is_not, take_while};
use nom::character::complete::{char, line_ending, not_line_ending};
use nom::multi::fold_many1;
use nom::sequence::terminated;
use nom::IResult;

#[derive(Default, Clone, Debug)]
pub struct Record {
    id: String,
    desc: Option<String>,
    seq: String,
}

pub struct Records<'a>(pub &'a str);

impl<'a> Iterator for Records<'a> {
    type Item = Result<Record, String>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            return None;
        }
        match record(self.0) {
            Ok((i, r)) => {
                self.0 = i;
                return Some(Ok(r));
            }
            Err(e) => {
                let mut msg = format!("{:?}", e);
                msg.push_str(self.0);
                return Some(Err(msg));
            }
        }
    }
}

fn start_tag(i: &str) -> IResult<&str, char> {
    char('>')(i)
}
fn line_seq(i: &str) -> IResult<&str, &str> {
    terminated(is_not(">\r\n"), line_ending)(i)
}
fn seq(i: &str) -> IResult<&str, String> {
    fold_many1(
        line_seq,
        String::with_capacity(256),
        |mut acc: String, x| {
            acc.push_str(x);
            acc
        },
    )(i)
}
pub fn record(i: &str) -> IResult<&str, Record> {
    let (i, _) = start_tag(i)?;
    let (i, title) = not_line_ending(i)?;
    let (i, _) = line_ending(i)?;
    let (i, seq) = seq(i)?;
    let (i, _) = take_while(|x| x != '>')(i)?;
    let mut header_fields = title.trim_end().splitn(2, char::is_whitespace);
    let id = header_fields.next().map(|s| s.to_owned()).unwrap();
    let desc = header_fields.next().map(|s| s.to_owned());
    Ok((i, Record { id, desc, seq }))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_start_tag() {
        assert_eq!(start_tag(">4823DSJ4"), Ok(("4823DSJ4", '>')))
    }
}
