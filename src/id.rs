use std::fmt::{Display, Formatter};
use std::ops::Index;

/// Opaque [`Id`] of an agent. This also gets used as an index into the array of agents in the [`World`]
#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub struct Id(usize);

impl Display for Id {
	fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
		write!(formatter, "Id({})", self.0)
	}
}

impl From<usize> for Id {
	fn from(id: usize) -> Self {
		Self(id)
	}
}

impl From<Id> for usize {
	fn from(id: Id) -> Self {
		id.0
	}
}

impl<Element> Index<Id> for Vec<Element> {
	type Output = Element;

	fn index(&self, id: Id) -> &Self::Output {
		self.index(id.0)
	}
}
