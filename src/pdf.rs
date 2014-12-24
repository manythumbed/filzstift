use std::io::IoResult;

struct IndexWriter<W>	{
	inner: Option<W>,
	pos: uint
}

impl<W: Writer> IndexWriter<W>	{
	fn new(w: W) -> IndexWriter<W> { 
		IndexWriter { inner: Some(w), pos: 0 }
	}
}

impl<W: Writer> Writer for IndexWriter<W>	{
	fn write(&mut self, buf: &[u8]) -> IoResult<()> { 
		self.pos += buf.len();
		self.inner.as_mut().unwrap().write(buf) 
	}
}

fn eol(w: &mut Writer) -> IoResult<()>	{ w.write_str("\r\n") }

fn header(w: &mut Writer) -> IoResult<()>	{
	try!(w.write_str("%PDF-1.7"));
	try!(eol(w));	
	Ok(())
}

fn body(w: &mut Writer) -> IoResult<()>	{
	let index = IndexWriter::new(w);
	Ok(())
}

#[cfg(test)]
mod test {
	use super::IndexWriter;
	use super::eol;
	use super::header;

	#[test]
	fn check_index_writer()	{
		let out = Vec::<u8>::new();
		let mut w = IndexWriter::new(out);
		w.write_str("test").unwrap();

		assert_eq!(w.inner.unwrap(), "test".as_bytes());
		assert_eq!(w.pos, "test".as_bytes().len());
	}

	#[test]
	fn check_eol()	{
		let mut out = Vec::<u8>::new();
		eol(&mut out).unwrap();
		assert_eq!(out.len(), 2);
		assert_eq!(out, "\r\n".as_bytes());
	}

	#[test]
	fn check_header()	{
		let mut out = Vec::<u8>::new();
		header(&mut out).unwrap();
		assert_eq!(out, "%PDF-1.7\r\n".as_bytes());
	}
}
