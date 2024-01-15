use webgpu_computes::*;
// use webgpu_computes::create_device_and_queue;

fn main() {
    let (device, queue) = pollster::block_on(create_device_and_queue());
    // let res = pollster::block_on(run_simple_add(&device, &queue));
    // println!("run_simple_add => #{:?}", res);
    let res = pollster::block_on(basic_compute(&device, &queue));
    println!("basic_compute => #{:?}", res)

}
