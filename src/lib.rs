use std::{
    io::{BufReader, Error, ErrorKind, Read, Write},
    net::{SocketAddr, TcpListener},
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::Duration,
};

use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

static CANCEL: AtomicBool = AtomicBool::new(false);

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("oauth").build()
}

/// Start the localhost (using 127.0.0.1) server. Returns the port its listening on.
/// You need to verify each response you get in your handler function, because of the system-wide localhost port.
///
/// # Arguments
///
/// * `response` - Optional static html string the user sees after being redirected. Default: "<html><body>Please return to the app.</body></html>".
/// * `handler` - Closure which gets called on every incoming connection.
///
/// # Errors
///
/// - Returns ErrorKind::AlreadyExists if there is an open server already.
/// - Returns std::io::Error if the server creation fails.
///
/// # Panics
///
/// The seperate server thread can panic if its unable to send the html response to the client. This may change after more real world testing.
pub fn start<F: FnMut(String) + Send + 'static>(
    response: Option<&'static str>,
    mut handler: F,
) -> Result<u16, std::io::Error> {
    if CANCEL.load(Ordering::SeqCst) {
        return Err(Error::new(
            ErrorKind::AlreadyExists,
            "Server already running, call stop() first.",
        ));
    }

    CANCEL.store(false, Ordering::SeqCst);

    let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], 0)))?;
    listener.set_nonblocking(true)?;

    let port = listener.local_addr()?.port();

    thread::spawn(move || {
        for conn in listener.incoming() {
            match conn {
                Ok(mut conn) => {
                    let mut conn_reader = BufReader::new(&conn);
                    let mut buffer = String::new();
                    if let Err(io_err) = conn_reader.read_to_string(&mut buffer) {
                        log::error!("Error reading incoming connection: {}", io_err.to_string());
                    };
                    buffer.pop();

                    let response =
                        response.unwrap_or("<html><body>Please return to the app.</body></html>");

                    // TODO: Test if unwrapping here is safe (enough).
                    conn.write_all(
                        format!(
                            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                            response.len(),
                            response
                        )
                        .as_bytes(),
                    )
                    .unwrap();
                    conn.flush().unwrap();

                    handler(buffer);
                }
                Err(err) if err.kind() == ErrorKind::WouldBlock => {
                    if CANCEL.load(Ordering::Relaxed) {
                        break;
                    } else {
                        thread::sleep(Duration::from_millis(500));
                    }
                }
                Err(err) => {
                    log::error!("Error reading incoming connection: {}", err.to_string());
                }
            }
        }
    });
    Ok(port)
}

/// Stop the currently running server.
/// The server needs up to 500ms to shutdown, but you don't need to wait before you can call [`start`] agian.
pub fn stop() {
    CANCEL.store(true, Ordering::SeqCst);
}
