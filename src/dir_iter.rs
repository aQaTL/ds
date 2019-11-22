use std::{
	path::Path,
	fs::{DirEntry, Metadata},
	io,
};

pub struct DirIter {
	stack: Vec<DirEntry>,
}

impl DirIter {
	pub fn new(path: &Path) -> io::Result<Self> {
		let mut dir_iter = DirIter {
			stack: Vec::new(),
		};
		dir_iter.read_dir(path)?;
		Ok(dir_iter)
	}

	fn read_dir(&mut self, dir: &Path) -> io::Result<()> {
		let mut entries = dir
			.read_dir()?
			.filter_map(Result::ok)
			.collect::<Vec<_>>();
		self.stack.append(&mut entries);
		Ok(())
	}
}

impl Iterator for DirIter {
	type Item = io::Result<(DirEntry, Metadata)>;

	fn next(&mut self) -> Option<Self::Item> {
		let last = match self.stack.pop() {
			Some(last) => last,
			None => return None,
		};

		let metadata = match last.metadata() {
			Ok(m) => m,
			Err(e) => return Some(Err(e)),
		};
		
		if metadata.is_dir() {
			if let Err(e) = self.read_dir(&last.path()) {
				return Some(Err(e));
			}
		}
		
		Some(Ok((last, metadata)))
	}
}
