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

