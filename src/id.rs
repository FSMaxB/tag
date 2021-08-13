use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use std::ops::Index;

/// Opaque [`Id`] of an agent. This also gets used as an index into the array of agents in the [`World`]
#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub struct Id(u32);

impl Display for Id {
	fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
		write!(formatter, "Id({})", self.0)
	}
}

impl TryFrom<usize> for Id {
	type Error = &'static str;

	fn try_from(id: usize) -> Result<Self, Self::Error> {
		match u32::try_from(id) {
			Ok(id) => Ok(Self(id)),
			Err(_) => Err("Ids larger than u32::MAX are not supported"),
		}
	}
}

impl From<u32> for Id {
	fn from(id: u32) -> Self {
		Self(id)
	}
}

impl From<Id> for u32 {
	fn from(id: Id) -> Self {
		id.0
	}
}

impl<Element> Index<Id> for Vec<Element> {
	type Output = Element;

	fn index(&self, id: Id) -> &Self::Output {
		self.index(usize::try_from(id.0).unwrap())
	}
}
