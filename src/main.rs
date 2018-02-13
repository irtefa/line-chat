#[macro_use()]
extern crate futures;
extern crate tokio;
#[macro_use()]
extern crate tokio_io;
extern crate bytes;

use tokio::executor::current_thread;
use tokio::net::{TcpListener, TcpStream};
use tokio_io::{AsyncRead};
use futures::prelude::*;
use futures::sync::mpsc;
use futures::future::{self, Either};
use bytes::{BytesMut, Bytes, BufMut};

use std::io::{self, Write};
use std::cell::RefCell;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::rc::Rc;

/// Shorthand for the transmit half of the message channel
type Tx = mpsc::UnboundedSender<Bytes>;

/// Shorthand for the receive half of the message channel
type Rx = mpsc::UnboundedReceiver<Bytes>;


fn main() {
    let addr: SocketAddr = match "127.0.0.1:6142".parse() {
        Ok(socket) => socket,
        Err(e) => {
            panic!("Could not parse socket {:?}", e);
        }
    };
    
    let listener = match TcpListener::bind(&addr) {
        Ok(listener) => listener,
        Err(e) => {
            panic!("Could not bind to the requested socket {:?}", e);
        }
    };

    let server = listener.incoming().for_each(move |socket| {
        // TODO: Process socket
        Ok(())
    })
    .map_err(|e| {
        //Handle eror by printing to STDOUT
        println!("accept error = {:?}", e);
    });

    //Start the executor and spawn the server task.
    current_thread::run(|_| {
        current_thread::spawn(server);

        println!("server running on localhost:6142");
    });
}
