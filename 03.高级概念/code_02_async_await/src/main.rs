use std::thread::{self};
use std::{
    sync::mpsc::{channel, Receiver},
    time,
};
use tokio::runtime::Builder;
use tokio::time::sleep;

// @author https://www.bilibili.com/video/BV1CN411f7nF/?spm_id_from=333.999.0.0&vd_source=0cc0401ee122346d6680e90658b0ed1a

/*

rust提供了原生了Future和poll等接口。然后可以有不同的运行时实现，比如tokio

==================================================================================================
||                                                                                              ||
||    poll队列                                                                                   ||
||    =========================================================                                 ||
||    ||                                                                                        ||
||    ||                                                       <==== 等待被poll                  ||
||    ||                                                                                        ||
||    =========================================================                                 ||   <==== Processor 1
||                                                                                              ||
||    等待唤醒队列                                                                                ||
||    =========================================================                                 ||
||    ||                                                                                        ||
||    ||                                                       <==== 等待被唤醒，然后poll          ||
||    ||                                                                                        ||
||    =========================================================                                 ||
||                                                                                              ||
==================================================================================================


==================================================================================================
||                                                                                              ||
||    poll队列                                                                                   ||
||    =========================================================                                 ||
||    ||                                                                                        ||
||    ||                                                       <==== 等待被poll                  ||
||    ||                                                                                        ||
||    =========================================================                                 ||   <==== Processor 2
||                                                                                              ||
||    等待唤醒队列                                                                                ||
||    =========================================================                                 ||
||    ||                                                                                        ||
||    ||                                                       <==== 等待被唤醒，然后poll          ||
||    ||                                                                                        ||
||    =========================================================                                 ||
||                                                                                              ||
==================================================================================================


==================================================================================================
||                                                                                              ||
||    poll队列                                                                                   ||
||    =========================================================                                 ||
||    ||                                                                                        ||
||    ||                                                       <==== 等待被poll                  ||
||    ||                                                                                        ||
||    =========================================================                                 ||   <==== Processor n
||                                                                                              ||
||    等待唤醒队列                                                                                ||
||    =========================================================                                 ||
||    ||                                                                                        ||
||    ||                                                       <==== 等待被唤醒，然后poll          ||
||    ||                                                                                        ||
||    =========================================================                                 ||
||                                                                                              ||
==================================================================================================

在tokio的实现中，如果不同的processor之间可以相互的抢占任务，从而实现高效的运行。
说人话就是，如果processor 1有一大堆任务，processor 2已经执行完自己的了，空了，则帮processor 1工作

每次有新的状态机(小)任务，都是放到processor来处理，即使是任务不断地在嵌套
*/

fn main() {
    // tokio有一个调度器。默认情况下，会启动很多个线程，把任务放到队列，然后线程到队列中抢Future来运行

    // new_current_thread()：调度器只启动一个线程
    // enable_all()：打开调度器的所有特性
    // build()：构造runtime，可能会不成功
    // let rt = Builder::new_current_thread().enable_all().build().unwrap();

    let rt = Builder::new_multi_thread().enable_all().build().unwrap();

    // let job = async {
    //     // async是一个语法糖，rust会将它编译为一个对象(状态机)，这个状态机实现了Future接口
    //     sleep(time::Duration::from_millis(2000)).await;
    //     println!("hello world!")
    // };

    // 让runtime运行状态机，直到它完成
    // rt.block_on(job);

    let v = rt.block_on(async {
        // 这里就会产生一个新的调度任务，放到poll队列。
        // 同时各个队列的调度器可以相互抢任务，使得任务调度高效。
        tokio::spawn(async {
            // 如果我们在这里等待1分钟，则YesYes不会被打印出来，因为主线程结束了
            println!("YesYes");

            // 当然我们仍然可以新建很多任务
            // tokio.spawn.....
        });

        // 还没到1秒的时候，会一直返回Pending。1秒之后，就会返回Ready然后重新调度
        sleep(time::Duration::from_secs(1)).await;
        println!("HiHi");

        42
    });

    println!("{}", v);

    // let job2 = Hello::Started {};
    // let v = rt.block_on(job2);
}

enum Hello {
    Started,
    Working(Receiver<i32>),
    Done,
}

// 为hello实现Future接口
impl std::future::Future for Hello {
    type Output = i32;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let mut h = self.get_mut();
        let waker = cx.waker().clone();

        // 第一次来到这里的时候，如果返回的是Pending，则被挂起，直到被唤醒来再次判断
        match *h {
            // 这个小的状态机，可能被多次调度，也就可能会在不同的线程。因此需要跨线程
            Hello::Started => {
                let (tx, rx) = channel();

                // 刚开始的时候，将状态机的状态设置为Working
                *h = Hello::Working(rx);

                // 这里我们来模拟一下给状态机Future的一个信号
                // 休眠1秒之后发送100给Future，然后程序会再次poll，进而Hello:Working会接收到东西。
                thread::spawn(move || {
                    thread::sleep(time::Duration::from_secs(1));
                    // 发送数据
                    tx.send(100);

                    // 必须在上下文中唤醒，否则此状态机是不会醒过来，然后检查状态机的"邮箱"的
                    // 通知将 item 从 "等待唤醒队列" 放到 "poll队列"
                    waker.wake();

                    // 检查“邮箱”：执行poll
                });

                // 此线程x不会被线程y阻塞，直接返回Pending
                std::task::Poll::Pending
            }

            // 不可以用rx。因为`h`是mut
            Hello::Working(ref rx) => {
                let v = rx.recv().unwrap();
                std::task::Poll::Ready(v) // Ready之后，将它从调度队列删除
            }

            // 这个分支不会被执行，可以不写
            Hello::Done => {
                panic!("Not here!");
            }
        }

        // println!("Hello, my future");
        // std::task::Poll::Ready(10)

        // std::task::Pool::Pending
        // 如果是返回Pending，则会将此Future实例挂起，放到某个队列中。
        // 某个事件唤醒此Future之后，会再次运行poll，由此重复，直到ready
    }
}
