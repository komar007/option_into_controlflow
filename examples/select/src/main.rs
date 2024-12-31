use std::{error::Error, ops::ControlFlow, time::Duration};

use tokio::{
    select,
    signal::unix::{signal, SignalKind},
    sync::mpsc,
    time,
};
use tokio_util::sync::CancellationToken;

use option_into_controlflow::OptionExt as _;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let token = CancellationToken::new();
    let process = process_messages(messages(), token.child_token());

    let mut hangup = signal(SignalKind::hangup())?;
    tokio::spawn(async move {
        hangup.recv().await.unwrap();
        token.cancel();
    });

    process.await;
    Ok(())
}

async fn process_messages(mut msgs: mpsc::Receiver<i32>, token: CancellationToken) {
    while let ControlFlow::Continue(msg) = select! { biased;
        _ = token.cancelled() => ControlFlow::Break(()),
        m = msgs.recv() => m.continue_or(()),
    } {
        println!("msg = {}", msg)
    }
}

fn messages() -> mpsc::Receiver<i32> {
    let (tx, rx) = mpsc::channel(1);
    tokio::spawn(async move {
        for i in 0..5 {
            time::sleep(Duration::from_secs(1)).await;
            let _ = tx.send(i).await;
        }
    });
    rx
}
