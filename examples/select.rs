use std::{error::Error, ops::ControlFlow, time::Duration};

use tokio::{
    select,
    signal::unix::{signal, SignalKind},
    sync::mpsc,
    time,
};

use into_controlflow::OptionExt as _;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut cancel = signal(SignalKind::hangup())?;
    let mut msgs = messages();

    while let ControlFlow::Continue(msg) = select! { biased;
        _ = cancel.recv() => ControlFlow::Break(()),
        m = msgs.recv() => m.continue_or(())
    } {
        println!("msg = {}", msg)
    }
    Ok(())
}

fn messages() -> mpsc::Receiver<i32> {
    let (tx, rx) = mpsc::channel(1);
    tokio::spawn(async move {
        for i in 0..10 {
            time::sleep(Duration::from_secs(1)).await;
            let _ = tx.send(i).await;
        }
    });
    rx
}
