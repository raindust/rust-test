use std::{
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};
use tower::{reconnect::Reconnect, service_fn, BoxError, Service, ServiceExt};

#[tokio::main]
async fn main() {
    let make_service = service_fn(|_target: ()| {
        // the future produced by the service must be `Unpin`
        Box::pin(async {
            let svc = MyService::default();
            println!("reconnecting");
            tokio::time::sleep(Duration::from_secs(2)).await;
            Ok::<_, BoxError>(svc)
        })
    });

    let mut svc = Reconnect::new::<MyService, String>(make_service, ());

    loop {
        let response = svc.ready().await.unwrap().call(()).await.unwrap();
        dbg!(&response);
    }
}

#[derive(Debug, Copy, Clone, Default)]
struct MyService {
    counter: usize,
}

impl Service<()> for MyService {
    type Response = String;
    type Error = BoxError;
    type Future = Pin<
        Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>,
    >;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        if self.counter < 10 {
            self.counter += 1;
            Poll::Ready(Ok(()))
        } else {
            // `MyService` dies after 10 requests
            Poll::Ready(Err(BoxError::from("dead".to_string())))
        }
    }

    fn call(&mut self, _request: ()) -> Self::Future {
        let count = self.counter;
        Box::pin(async move { Ok(format!("request {}", count)) })
    }
}
