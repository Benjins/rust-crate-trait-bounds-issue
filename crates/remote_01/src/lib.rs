use core::marker::PhantomData;

pub struct RemoteBound<Met, T> {
	m : PhantomData<Met>,
	t : PhantomData<T>
}

impl<Met, T> RemoteBound<Met, T>
{
	pub fn new() -> Self {
		Self {
			m: PhantomData::<Met>::default(),
			t: PhantomData::<T>::default()
		}
	}
}

impl<Met, T> RemoteBound<Met, T>
where
    Met: space::Metric<T>,
{
	pub fn insert(&self) -> usize {
		0
	}
}

pub fn remote_generic_func<Met : space::Metric<T>, T>() {
	let _ = PhantomData::<Met>::default();
	let _ = PhantomData::<T>::default();
}

pub fn remote_func(_ : &space::Neighbor<u32>) { }

