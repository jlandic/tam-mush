use native_tls;
use native_tls::Identity;
use tokio::net::{TcpListener, TcpStream};
use tokio::stream::{Stream, StreamExt};
use tokio::sync::{mpsc, Mutex};
use tokio_native_tls::{TlsAcceptor, TlsStream};
use tokio_util::codec::{Framed, LinesCodec, LinesCodecError};

use futures::SinkExt;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use crate::engine::commands::{LoginHandler, RegisterHandler};
use crate::engine::{Command, CommandHandler};

type Rx = mpsc::UnboundedReceiver<String>;
type Tx = mpsc::UnboundedSender<String>;

struct SharedState {
    peers: HashMap<SocketAddr, Tx>,
}

impl SharedState {
    fn new() -> Self {
        Self {
            peers: HashMap::new(),
        }
    }

    async fn broadcast(&mut self, ignore: SocketAddr, message: &str) {
        for peer in self.peers.iter_mut() {
            if *peer.0 != ignore {
                let _ = peer.1.send(message.into());
            }
        }
    }
}

struct Client {
    lines: Framed<TlsStream<TcpStream>, LinesCodec>,
    rx: Rx,
}

impl Client {
    async fn new(
        state: Arc<Mutex<SharedState>>,
        lines: Framed<TlsStream<TcpStream>, LinesCodec>,
    ) -> io::Result<Self> {
        let addr = lines.get_ref().get_ref().get_ref().get_ref().peer_addr()?;
        let (tx, rx) = mpsc::unbounded_channel();

        state.lock().await.peers.insert(addr, tx);

        Ok(Client { lines, rx })
    }
}

impl Stream for Client {
    type Item = Result<Message, LinesCodecError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if let Poll::Ready(Some(msg)) = Pin::new(&mut self.rx).poll_next(cx) {
            return Poll::Ready(Some(Ok(Message::Response(msg))));
        }

        let result: Option<_> = futures::ready!(Pin::new(&mut self.lines).poll_next(cx));

        Poll::Ready(match result {
            Some(Ok(msg)) => Some(Ok(Message::Received(msg))),
            Some(Err(e)) => Some(Err(e)),
            None => None,
        })
    }
}

#[derive(Debug)]
enum Message {
    Received(String),
    Response(String),
}

pub struct Server {}
impl Server {
    pub async fn start(binding: &str) -> Result<(), Box<dyn Error>> {
        let state = Arc::new(Mutex::new(SharedState::new()));
        let listener = TcpListener::bind(binding).await?;

        let mut der_file = File::open("identity.p12")?;
        let mut der = Vec::new();
        der_file.read_to_end(&mut der)?;

        let p12_password = env::var("P12_PASSWORD").unwrap_or("".to_string());
        let cert = Identity::from_pkcs12(&der, &p12_password)?;
        let tls_acceptor = TlsAcceptor::from(native_tls::TlsAcceptor::builder(cert).build()?);

        println!("Listening on {}", binding);

        loop {
            let (stream, addr) = listener.accept().await?;
            let state = Arc::clone(&state);
            let tls_acceptor = tls_acceptor.clone();

            tokio::spawn(async move {
                if let Err(e) = Server::process_client(state, stream, addr, tls_acceptor).await {
                    eprintln!("{:?}", e);
                }
            });
        }
    }

    async fn process_client(
        state: Arc<Mutex<SharedState>>,
        stream: TcpStream,
        addr: SocketAddr,
        tls_acceptor: TlsAcceptor,
    ) -> Result<(), Box<dyn Error>> {
        let stream = tls_acceptor.accept(stream).await.expect("Accept error");
        let mut lines = Framed::new(stream, LinesCodec::new());
        lines.send("Welcome!").await?;

        let mut client = Client::new(state.clone(), lines).await?;
        println!("{} connected", addr);

        while let Some(result) = client.next().await {
            match result {
                Ok(Message::Received(msg)) => {
                    let response = match Command::parse(&msg) {
                        Ok(cmd) => {
                            let mut command_handlers: Vec<Box<dyn CommandHandler>> = Vec::new();
                            command_handlers.push(Box::new(LoginHandler {}));
                            command_handlers.push(Box::new(RegisterHandler {}));

                            match command_handlers
                                .iter()
                                .find(|handler| handler.can_respond_to(&cmd))
                            {
                                Some(handler) => match handler.handle(&cmd, None) {
                                    Ok(_) => Some("Successfully logged in!".to_string()),
                                    Err(e) => Some(e),
                                },
                                None => Some("No handler found for your command".to_string()),
                            }
                        }
                        Err(e) => Some(e),
                    };

                    if let Some(response) = response {
                        client.lines.send(&response).await?;
                    }
                }
                Ok(Message::Response(msg)) => {
                    client.lines.send(&msg).await?;
                }
                Err(e) => {
                    eprintln!("{:?}", e);
                }
            }
        }

        println!("{} disconnected", addr);

        Ok(())
    }
}
