use std::collections::HashMap;

pub struct EntityRegistry<E> {
	entities: HashMap<String, E>,
}

#[allow(dead_code)]
impl<E> EntityRegistry<E> {
	pub fn new() -> Self {
		Self {
			entities: HashMap::new(),
		}
	}

	pub fn register<T: ToString>(&mut self, name: &T, e: E) {
		self.entities.insert(name.to_string(), e);
	}

	pub fn get<T: ToString>(&self, name: &T) -> Option<&E> {
		self.entities.get(&name.to_string())
	}

	pub fn get_mut<T: ToString>(&mut self, name: &T) -> Option<&mut E> {
		self.entities.get_mut(&name.to_string())
	}

	pub fn values(&self) -> Vec<&E> {
		self.entities.values().collect::<Vec<&E>>()
	}
}

impl<E> std::fmt::Debug for EntityRegistry<E> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		write!(f, "{:?}", self.entities.keys())
	}
}

impl<E> std::fmt::Display for EntityRegistry<E> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		write!(f, "{}", self)
	}
}
