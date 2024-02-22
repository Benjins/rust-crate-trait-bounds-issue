
struct Vector;
struct MyDist;

// NOTE: This crate depends on version 0.18.0 of the space crate
// This makes it incompatible with the 0.17.0 version of space that remote_01 depends on

impl space::Metric<Vector> for MyDist  {
    type Unit = u32;
    fn distance(&self, _: &Vector, _: &Vector) -> u32 {
        0
    }
}

// Here, we get a generic "trait bounds were not satisfied" error
// It can be confusing since we are satisfying the bounds, just for a trait with the same name,
// in an incompatible version of the same crate
fn use_remote_bound() {
	let rb = remote_01::RemoteBound::<MyDist, Vector>::new();
	rb.insert();
}

// Similar issue here
fn use_remote_generic_arg() {
	//remote_01::remote_generic_func::<MyDist, Vector>();
}

// If we uncomment this, we see a slightly nicer compiler message in the case of an
// argument type mismatch:
// It realises that the two types are potentially from incompatible versions of the same crate,
fn use_remote_arg() {
	//let arg = space::Neighbor::<u32> {
	//	index: 0,
	//	distance: 0
	//};
	//remote_01::remote_func(&arg);
}


fn main() { }
