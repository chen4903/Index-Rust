#[allow(dead_code, unused_imports)]
use std::thread;
use std::time::Duration;

struct Hello {
    i: i64,
    v: Vec<i32>, // in heap
}

fn main() {
    let h = Hello {
        i: 10,
        v: vec![1, 2, 3, 4],
    };

    // 变量存储在stack

    // print h's addresses here
    println!("&h {:?}", &h as *const _);
    println!("&h.i {:?}", &h.i as *const _);
    println!("&h.v {:?}", &h.v as *const _);
    println!("&h.v[0] {:?}", &h.v[0] as *const _);

    /*

    thread::spawn(move || {
        // h moved here
        // print h's addresses here again
        println!("==========");
        println!("&h {:?}", &h as *const _);
        println!("&h.i {:?}", &h.i as *const _);
        println!("&h.v {:?}", &h.v as *const _);
        println!("&h.v[0] {:?}", &h.v[0] as *const _);
    });


        (前三个是stack上的地址，第四个是heap上的地址)
        &h      0x16d7764e0       h 在main线程上的地址
        &h.i    0x16d7764f8       i 在main线程上的地址
        &h.v    0x16d7764e0       v 在main线程上的地址。(可以发现它和&h的结果一样。但这不是固定的，也有可能是&h.i和&h一样，这取决于rust如何处理内存)
        &h.v[0] 0x14ae06000       v本身是一个vector，数据存储在heap。因此这是heap上的一个地址

        ==========

        (前三个是stack上的地址，第四个是heap上的地址)
        &h      0x16d97ec50
        &h.i    0x16d97ec68
        &h.v    0x16d97ec50
        &h.v[0] 0x14ae06000
        (
        我们可以发现:
            - move：将原来stack上的内容，copy到新一个新的线程的新的stack位置，stack上的地址(前三个)变化了
            - 第四个地址没有变0x14ae06000，因为它是在heap
            - 第二次打印的结果(地址空间)和第一次打印的相差较大
        )

    thread::sleep(Duration::from_secs(1));

    */

    /*
    let f = move || {
        // h moved here
        // print h's addresses here again
        println!("==========");
        println!("&h {:?}", &h as *const _);
        println!("&h.i {:?}", &h.i as *const _);
        println!("&h.v {:?}", &h.v as *const _);
        println!("&h.v[0] {:?}", &h.v[0] as *const _);
    };


        (这里的情况和上面的例子一样)
        &h 0x16ba8e4f0
        &h.i 0x16ba8e508
        &h.v 0x16ba8e4f0
        &h.v[0] 0x134e06000

        ==========

        &h 0x16ba8e6b0
        &h.i 0x16ba8e6c8
        &h.v 0x16ba8e6b0
        &h.v[0] 0x134e06000
        (
        我们可以发现:
            - move：将原来stack上的内容，copy到新一个新的线程的新的stack位置，stack上的地址(前三个)变化了
            - 第四个地址没有变0x14ae06000，因为它是在heap
            - 第二次打印的结果(地址空间)和第一次打印的相差较小。因此是在同一个stack上
        )

    f();
    */

    // move: 本质是将stack上某数据的内存内容，换到(拷贝)到了另外一个位置。
    //        可能换到了同一个线程的stack的不同位置，也有可能哈顿熬了其他线程的stack位置。
    //        因为：不同的线程有自己的stack位置(范围)。
    //        一个内存位置的内容被move之后，你就不可以再次使用: rust 的 borrow机制。
    //        对heap的内容无影响

    let f = || {
        // h moved here
        // print h's addresses here again
        println!("==========");
        println!("&h {:?}", &h as *const _);
        println!("&h.i {:?}", &h.i as *const _);
        println!("&h.v {:?}", &h.v as *const _);
        println!("&h.v[0] {:?}", &h.v[0] as *const _);
    };

    /*

        没有move关键字，发现两次打印的结果都一样。没有拷贝(移动)，只是通过了指针来引用

        &h 0x16ba0e520
        &h.i 0x16ba0e538
        &h.v 0x16ba0e520
        &h.v[0] 0x128606000

        ==========

        &h 0x16ba0e520
        &h.i 0x16ba0e538
        &h.v 0x16ba0e520
        &h.v[0] 0x128606000

    */

    f();
}
