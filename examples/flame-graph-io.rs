use std::time::Duration;
#[inline(never)]
fn task1(){
    println!("start task1");
    std::thread::sleep(Duration::from_millis(100));
    println!("end task1");
}
#[inline(never)]
fn task2(){
    println!("start task2");
    std::thread::sleep(Duration::from_millis(200));
    println!("end task2");
}

fn main(){
    for i in 1..20 {
        task1();
        task2();
    }
}