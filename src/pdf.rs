use std::io::IoResult;

fn eol(w: &mut Writer) -> IoResult<()>	{
	w.write_str("\r\n")
}

fn header(w: &mut Writer) ->IoResult<()>	{
	try!(w.write_str("%PDF–1. 7"));
	try!(eol(w));	
	Ok(())
}

#[cfg(test)]
mod test {
	use super::eol;
	use super::header;

	#[test]
	fn check_eol()	{
		let mut out = Vec::<u8>::new();
		eol(&mut out).unwrap();
		assert_eq!(out.len(), 2);
		assert_eq!(out[0], b'\r');
		assert_eq!(out[1], b'\n');
	}

	#[test]
	fn check_header()	{
		let mut out = Vec::<u8>::new();
		header(&mut out).unwrap();
		assert_eq!(out, vec!(b'%', b'P', b'D', b'F'));
	}
}
