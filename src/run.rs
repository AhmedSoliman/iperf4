use std::net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs};
use std::time::Duration;

use anyhow::Context;
use log::debug;

use crate::opts::IPerfCliOpts;

pub fn run_iperf(opts: IPerfCliOpts) -> anyhow::Result<()> {
    if opts.server.server {
        // server mode
        let server_socket =
            TcpListener::bind(("::", opts.common.port)).context("Could not bind to socket")?;
        println!(" -----------------------------------------------------------");
        println!("Server listening on {} (test #1)", opts.common.port);
        println!("-----------------------------------------------------------");

        debug!("Waiting for connection");
        let (stream, peer) = server_socket
            .accept()
            .context("Failed to accept more connections")?;
        debug!("Connection accepted from {:?}", peer);
        // stream exists (server)
    } else {
        // client mode
        let addresses =
            format!("{}:{}", opts.client.client.unwrap(), opts.common.port).to_socket_addrs()?;
        let stream = connect(
            addresses,
            opts.client.connect_timeout.map(Duration::from_millis),
        )?;

        // stream exists (server)
    }
    Ok(())
}

fn connect(
    addresses: impl IntoIterator<Item = SocketAddr>,
    timeout: Option<Duration>,
) -> anyhow::Result<TcpStream> {
    for address in addresses {
        debug!("Attempting connection to {:?}", address);
        let maybe_stream = match timeout {
            Some(timeout) => TcpStream::connect_timeout(&address, timeout),
            None => TcpStream::connect(address),
        };
        if let Ok(stream) = maybe_stream {
            debug!("Connected to {:?}", stream.peer_addr());
            return Ok(stream);
        }
    }

    Err(anyhow::anyhow!(
        "Failed to connect to any address for the peer name"
    ))
}
