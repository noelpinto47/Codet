use iced::{stream, Subscription};
use crate::app::message::{Message, PtyHandle, WriterHandle};
use portable_pty::{CommandBuilder, NativePtySystem, PtySize, PtySystem};
use std::sync::{Arc, Mutex};
use std::io::Read;
use iced::futures::SinkExt;
use iced::futures::channel::mpsc::Sender;

pub fn terminal_subscription() -> Subscription<Message> {
    Subscription::run(pty_stream)
}

fn pty_stream() -> impl iced::futures::Stream<Item = Message> {
    stream::channel(100, |mut sender: Sender<Message>| async move {
        let (pty_arc, writer_arc, reader) =
            tokio::task::spawn_blocking(|| {
                let pty_system = NativePtySystem::default();
                let pair = pty_system
                    .openpty(PtySize { rows: 24, cols: 80, pixel_width: 0, pixel_height: 0 })
                    .expect("failed to open PTY");

                let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string());
                pair.slave
                    .spawn_command(CommandBuilder::new(shell))
                    .expect("failed to spawn shell");

                let writer = pair.master.take_writer().expect("writer");
                let reader = pair.master.try_clone_reader().expect("reader");
                let pty_arc = Arc::new(Mutex::new(pair.master));
                let writer_arc = Arc::new(Mutex::new(writer));
                (pty_arc, writer_arc, reader)
            })
            .await
            .expect("PTY spawn failed");

        let _ = sender.send(Message::TerminalStarted(
            PtyHandle(pty_arc),
            WriterHandle(writer_arc),
        )).await;

        let reader = Arc::new(Mutex::new(reader));
        loop {
            let reader = Arc::clone(&reader);
            let result = tokio::task::spawn_blocking(move || {
                let mut buf = [0u8; 1024];
                let mut r = reader.lock().unwrap();
                r.read(&mut buf).map(|n| (n, buf))
            })
            .await;

            match result {
                Ok(Ok((0, _))) | Err(_) => break,
                Ok(Ok((n, buf))) => {
                    let s = String::from_utf8_lossy(&buf[..n]).to_string();
                    let _ = sender.send(Message::TerminalOutput(s)).await;
                }
                Ok(Err(_)) => break,
            }
        }

        std::future::pending::<()>().await;
    })
}