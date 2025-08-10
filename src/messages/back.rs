use std::net::TcpStream;
use std::io::{Error, Read};
use std::io;
use std::os::fd::AsRawFd;
use libc::{poll, pollfd, POLLIN};
use std::os::unix::io::RawFd;

pub enum BackTypes {
    AuthenticationOk,
    CommandComplete,
    ErrorResponse,
}

pub struct BackMessage {
    msg_type: BackTypes,
    data: Vec<u8>,
}

impl BackMessage {
    pub fn from_buf(stream: &mut TcpStream) -> io::Result<BackMessage> {
        let mut header = [0; 5];

        wait_for_data(&stream.as_raw_fd(), 2000).unwrap();

        stream.read_exact(&mut header)?;

        let msg_type = match header[0] {
            b'R' => BackTypes::AuthenticationOk,
            b'C' => BackTypes::CommandComplete,
            _ => BackTypes::ErrorResponse,
        };

        let msg_len = u32::from_be_bytes(
            [header[1], header[2], header[3], header[4]]
        ) as usize;

        let mut body = vec![0; msg_len - 4];
        stream.read_exact(&mut body)?;

        Ok(BackMessage {
            msg_type: msg_type, 
            data: body,
        })
    }

    #[inline]
    pub fn msg_type(&self) -> &BackTypes {
        &self.msg_type
    }

    #[inline]
    pub fn data(&self) -> String {
        String::from_utf8(self.data.clone()).unwrap()
    }
}

fn wait_for_data(fd: &RawFd, timeout_ms: i32) -> Result<(), String> {
   let mut fds = [pollfd {
        fd: *fd,
        events: POLLIN,
        revents: 0, 
   }];

   let ret = unsafe { poll(fds.as_mut_ptr(), 1, timeout_ms) };

   if ret == -1 {
       return Err(format!("ERROR: cannot wait for file descriptor {}", Error::last_os_error()));
   } else if ret == 0 {
       return Err("ERROR: time out during reading file descriptor".to_string());
   }

   Ok(())
}
