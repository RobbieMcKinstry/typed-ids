use flume::{bounded, Receiver, Sender};

pub struct Generator<T: Send + 'static> {
    _shutdown: Sender<()>,
    recv: Receiver<T>,
}

impl<T: Send + 'static> Generator<T> {
    pub fn new<F>(mut next: F) -> Self
    where
        F: FnMut() -> T + Send + 'static,
    {
        // Create channels to send values on
        // and to listen for the shutdown signal.
        // TODO: Do something with the shutdown receiver.
        let (id_sender, id_receiver) = bounded(0);
        let (shutdown_sender, _shutdown_receiver) = bounded(0);
        // Kick off a thread to generte the values.
        tokio::spawn(async move {
            loop {
                let val = next();
                let res = id_sender.send_async(val).await;
                if res.is_err() {
                    break;
                }
                tokio::task::yield_now().await;
            }
        });
        // Return the handle to the receiver and shutdown channels.
        Self {
            recv: id_receiver,
            _shutdown: shutdown_sender,
        }
    }

    pub async fn next(&self) -> T {
        self.recv.recv_async().await.unwrap()
    }
}
