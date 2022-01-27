use std::borrow::{Borrow, BorrowMut};
use std::ops::Add;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, mpsc, Mutex};

fn simple_spawn() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

fn closure() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    // variable moved due to use in closure
    // mah(v);

    handle.join().unwrap();
}

fn boh<F>(f: F)
    where F: FnOnce() -> () //, F: Send + 'static
{
    f();
}

fn mah(v: Vec<u32>) {}

fn multiple_consumers() {
    let num_threads = 10;
    let (tx, rx) = mpsc::channel();
    let senders = (0..num_threads).map(
        |x| {
            let tx = tx.clone();
            thread::spawn(
                move || {
                    let val = String::from(format!("hi from thread {}", x));
                    tx.send(val).unwrap();
                }
            )
        }
    );

    let recv_server = thread::spawn(move || {
        (0..num_threads).for_each(|x| {
            let v = rx.recv().unwrap();
            println!("Received {}", v);
        })
    });
    senders.for_each(|j| j.join().unwrap());
    recv_server.join().unwrap();
}

fn play_with_arc() {
    let m = Arc::new(12345);
    let m_thread = Arc::clone(&m);
    let j = thread::spawn(move || {
        println!("Subthread attempting to get Arc value");
        let mut num = *m_thread;
        println!("Subthread got Arc value, sleeping");
        thread::sleep(Duration::from_millis(5000));
        num += 1;
    });
    thread::sleep(Duration::from_millis(1000));
    println!("Main thread attempting to get Arc value");
    let mut x = *m;
    x += 12345;
    println!("Main thread got Arc value, wait for thread to finish");
    j.join().unwrap();

}
fn mutex(){
    println!("Another test on mutexes");
    let m = Arc::new(Mutex::new(5));
    let m_thread = Arc::clone(&m);
    let j = thread::spawn(move || {
        let mut num = m_thread.lock().unwrap();
        panic!("Intentionally panicked while keeping a lock");
    });

    let mut num = m.lock().unwrap();
    *num = 6;
    println!("m = {:?}", m);
    j.join().unwrap_or("");
}


fn _play_with_mutable_arc_thread(thread: u8, m: &Arc<Mutex<String>>, i: u32) {
    let mut s = m.lock().unwrap();
    let old_s = String::from(&*s);
    let s_new = old_s + format!("thread {}/{}", thread, i).as_str();
    *s = s_new;
}

fn play_with_mutable_arc() {
    println!("Playing with mutable arc");
    let m = Arc::new(Mutex::new(String::from("hello")));
    let mut mt = Arc::clone(&m);
    let mut mt2 = Arc::clone(&m);
    let j = thread::spawn(move || {
        for i in 0..100 {
            _play_with_mutable_arc_thread(1, &mt, i);

            thread::sleep(Duration::from_millis(100));
        }
    });
    let j2 = thread::spawn(move || {
        for i in 0..100 {
            _play_with_mutable_arc_thread(2, &mt2, i);
            thread::sleep(Duration::from_millis(100));
        }
    });
    j.join().unwrap_or(());
    j2.join().unwrap_or(());
    println!("play_with_mutable_arc {:?}", *m);
}

fn main() {

    simple_spawn();
    closure();
    let v = vec![1, 2, 3];
    boh(|| {
        println!("Here's a vector: {:?}", v);
    });

    multiple_consumers();
    play_with_arc();

    play_with_mutable_arc();
    mutex();
}