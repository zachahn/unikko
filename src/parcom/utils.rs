use nom::IResult;
use std::fmt;

#[allow(dead_code)]
fn dbg_dmp_s<'a, F, O, E: fmt::Debug>(
    mut f: F,
    context: &'static str,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    use nom::HexDisplay;
    move |i: &'a str| match f(i) {
        Err(e) => {
            println!("{}: Error({:?}) at:\n{}", context, e, i.to_hex(8));
            Err(e)
        }
        a => a,
    }
}
