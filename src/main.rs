
struct Vector;
struct MyDist;


impl space::Metric<Vector> for MyDist  {
    type Unit = u32;
    fn distance(&self, _: &Vector, _: &Vector) -> u32 {
        0
    }
}

fn foo_bar() {
	let rb = remote_01::RemoteBound::<MyDist, Vector>::new();
	rb.insert();
}


fn main() {
	foo_bar()
}
