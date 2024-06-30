use tokio::sync::mpsc;
use tokio::task::JoinHandle;

// A type that implements BlockFunctionality consumes instances of T and
// produces either Ok(Some(U)) if an output is ready, Ok(None) if an output
// is not ready, or an Err(_) if the operation fails
pub trait BlockFunctionality<T, U> {
    fn apply(&mut self, input: T) -> Result<Option<U>, &'static str>;
}

pub struct Block<T, U> {
    pub tx_input: mpsc::Sender<T>,
    pub rx_output: mpsc::Receiver<U>,
    pub handle: JoinHandle<Result<(), &'static str>>,
}

impl<T: Send, U: Send> Block<T, U> {
    pub fn from<B: BlockFunctionality<T, U> + Send>(b: B) -> Self {
        let (tx_input, mut rx_input) = mpsc::channel(10);
        let (mut tx_output, rx_output) = mpsc::channel(10);

        let handle: JoinHandle<Result<(), &'static str>> = tokio::spawn(async move {
            let mut owned_b = b;

            while let Some(t) = rx_input.recv().await {
                match owned_b.apply(t)? {
                    Some(u) => tx_output
                        .send(u)
                        .await
                        .map_err(|_| "Unable to send output")?,
                    None => (),
                }
            }

            Ok(())
        });

        Block {
            tx_input,
            rx_output,
            handle,
        }
    }
}

fn main() {}
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{sleep, Duration};

    struct MyBlockFunctionality;

    impl BlockFunctionality<i32, i32> for MyBlockFunctionality {
        fn apply(&mut self, input: i32) -> Result<Option<i32>, &'static str> {
            Ok(Some(input + 1))
        }
    }

    #[tokio::test]
    async fn test_block() {
        let mut block = Block::from(MyBlockFunctionality);
        block.tx_input.send(1).await.unwrap();
        assert_eq!(block.rx_output.recv().await.unwrap(), 2);
    }
}