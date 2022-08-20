use futures::Future;
use std::{
    sync::{Arc, Mutex},
    task::{Poll, Waker},
};

pub struct MsgFuture {
    state: SharedState,
}

#[derive(Debug, Default)]
pub struct MsgState {
    pub msg: Option<String>,
    /// The waker for the task that `TimerFuture` is running on.
    /// The thread can use this after setting `completed = true` to tell
    /// `TimerFuture`'s task to wake up, see that `completed = true`, and
    /// move forward.
    pub waker: Option<Waker>,
}
pub type SharedState = Arc<Mutex<MsgState>>;

impl Future for MsgFuture {
    type Output = Result<String, String>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        // Look at the shared state to see if the timer has already completed.
        let mut state = self.state.lock().unwrap();
        if let Some(msg) = &state.msg {
            Poll::Ready(Ok(msg.clone()))
        } else {
            state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl MsgFuture {
    pub fn new() -> (Self, SharedState) {
        let state = Arc::new(Mutex::new(MsgState {
            msg: None,
            waker: None,
        }));
        (
            Self {
                state: state.clone(),
            },
            state,
        )
    }
}
