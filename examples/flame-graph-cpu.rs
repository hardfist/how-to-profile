use std::time::Duration;
fn heavy_task(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => heavy_task(n-1) + heavy_task(n-2)
    }
}
#[inline(never)]
fn task1(){
    println!("start task1");
    heavy_task(40);
    println!("end task1");
}
#[inline(never)]
fn task2(){
    println!("start task2");
    heavy_task(40);
    println!("end task2");
}

fn main(){
    for i in 1..10 {
        task1();
        std::thread::sleep(Duration::from_millis(10));
        task2();
    }
}