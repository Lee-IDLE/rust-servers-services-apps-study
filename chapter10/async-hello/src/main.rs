use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread::sleep;
use std::time::{Duration, Instant};

struct ReadFileFuture {}

impl Future for ReadFileFuture {
    type Output = String;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("Tokio! Stop polling me");
        // Waker 인스턴스에 대한 wake_by_ref() 함수가 호출된다.
        // 이는 차례로 Tokio 런타임에게 해당 비동기 태스크를 실행을 위해
        // 다시 스케줄링할 준비가 되었음을 알린다.
        cx.waker().wake_by_ref(); // 깨어나서 실행할 준비가 되었으니 다시 깨워주세요
        //Poll::Pending // 대기 모드로 전환
        Poll::Ready(String::from("Hello, there from file 1"))
    }
}

struct AsyncTimer {
    expiration_time: Instant
}

impl Future for AsyncTimer {
    type Output = String;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.expiration_time {
            // 시간 됨
            println!("Hello, it's time for future");
            Poll::Ready(String::from("Future 1 has completed"))
        } else {
            println!("Hello, it's not yet time for Future 1. Going to sleep");
            let waker = cx.waker().clone();
            let expiration_time = self.expiration_time;
            
            std::thread::spawn(move || {
                let current_time = Instant::now();
                if current_time < expiration_time {
                    std::thread::sleep(expiration_time - current_time);
                    waker.wake();
                }
            });
            
            Poll::Pending
        }
    }
}

#[tokio::main]
async fn main() {
    println!("Hello before reading file!");

    /*
    let h1 = tokio::spawn(async {
        let future1 = ReadFileFuture {};
        future1.await
    });
    */
    let h1 = tokio::spawn(async {
        let future1 = AsyncTimer {
            expiration_time: Instant::now() + Duration::from_millis(4000),
        };
        println!("{:?}", future1.await);
    });

    let h2 = tokio::spawn(async {
        let file2_contents = read_from_file2().await;
        println!("{:?}", file2_contents);
    });

    let _ = tokio::join!(h1, h2);
}

// 파일 읽기를 시뮬레이션 하는 함수
fn read_from_file2() -> impl Future<Output = String> {
    async {
        sleep(Duration::new(2, 0));
        println!("{:?}", "Processing file 2");
        String::from("Hello, there from file 2")
    }
}
