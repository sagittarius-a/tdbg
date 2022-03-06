//! Quick and dirty way to trace code execution and count associated hits.
//!
//! # Use-case
//! You're debugging a piece of code and you want to determine if a piece
//! of code is hit, and if so, how many times it is hit.
//!
//! `tdbg` provides the `tdbg!` macro to associate an identifier to a code
//! location. Using the macro will send a message to the `tdbg` server
//! (standalone binary), that will report every hit and the associated
//! number of time it has been hit.
//!
//! Simply fire up the server, insert `tdbg!` macros and you're done.
//!
//! # Philosophy
//!
//! - `tdbg` tries to interrupt the original workflow as less as possible
//! - `tdbg` should never panic in order to not disrupt the origin code
//! - `tdbg` tries to stay as minimal and portable as possible
//! - This is not really production ready. Use it at your own risk. Keep
//!   in mind that it is a tool I personally use and I do not expect
//!   anyone else to do as well.

use std::io::{Error, Write};
use std::net::TcpStream;

#[macro_export]
/// Macro that will send the message to the `tdbg` server.
///
/// # Example
///
/// Make sure to run the server beforehand.
/// ```rust
/// use tdbg::tdbg;
///
/// fn process_message() {
///     // Very long and complex code
///     // ---- 8< ----
///
///     if unlikely_condition {
///         tdbg!("message");
///         // Emergency actions
///         // ---- 8< ----
///     }
///     
///     // ---- 8< ----
/// }
///
/// fn close_socket() {
///     // Closing socket properly
///     // ---- 8< ----
///     tdbg!("close");
/// }
///
/// ```
macro_rules! tdbg {
    ($e:expr) => {{
        tdbg::send_tdbg($e).unwrap();
    }};
}

/// Send the message to the `tdbg` server.
///
/// Always return Ok.
///
/// Silently fails if connection is refused. This way, it does not disrupt the workflow
/// of the original program.
pub fn send_tdbg(message: &str) -> Result<(), Error> {
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:31337") {
        let _ = stream.write(message.as_bytes());
    }

    Ok(())
}
