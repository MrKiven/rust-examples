use std::thread;

#[no_mangle]
pub extern fn process() {
    let handlers: Vec<_> = (0..10).map(|_| {
        thread::spawn(|| {
            let mut x = 0;
            for _ in 0..5_000_000 {
                x += 1
            }
            x
        })
    }).collect();

    for h in handlers {
        println!("Thread finished with count={}",
                 h.join().map_err(|_| "Cound not join a thread!").unwrap());
    }
}
