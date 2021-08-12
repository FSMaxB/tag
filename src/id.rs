pub struct Id(usize);

#[derive(Default)]
pub struct IdSource {
	next_id: usize,
}

impl Iterator for IdSource {
	type Item = Id;

	fn next(&mut self) -> Option<Self::Item> {
		if self.next_id == usize::MAX { // debatable if this is necessary
			return None;
		}

		let id = self.next_id;
		self.next_id += 1;

		Some(Id(id))
	}
}
