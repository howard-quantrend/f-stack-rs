#![allow(
    deprecated,
    unused,
    clippy::useless_attribute,
    clippy::not_unsafe_ptr_arg_deref,
    clippy::trivially_copy_pass_by_ref,
    clippy::many_single_char_names
)]
#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case)]

mod binding;

use libc::c_char;
use libc::c_int;
use libc::c_void;
use std::ffi::CString;

use binding::in_addr;
pub use binding::{
    epoll_data, epoll_event, ff_accept, ff_bind, ff_close, ff_epoll_create, ff_epoll_ctl,
    ff_epoll_wait, ff_init, ff_ioctl, ff_listen, ff_read, ff_run, ff_socket, ff_write, htonl,
    htons, linux_sockaddr, loop_func_t, sockaddr_in, socklen_t, strerror,
};

pub mod constant {
    pub use crate::binding::{
        __socket_type_SOCK_STREAM as SOCK_STREAM, AF_INET, EPOLL_CTL_ADD, EPOLL_CTL_DEL,
        EPOLL_EVENTS_EPOLLERR as EPOLLERR, EPOLL_EVENTS_EPOLLIN as EPOLLIN,
    };
}

impl Default for epoll_event {
    fn default() -> Self {
        Self {
            events: Default::default(),
            data: epoll_data { u64_: 0 },
        }
    }
}

impl Default for in_addr {
    fn default() -> Self {
        Self {
            s_addr: Default::default(),
        }
    }
}

impl Default for sockaddr_in {
    fn default() -> Self {
        Self {
            sin_family: Default::default(),
            sin_port: Default::default(),
            sin_addr: Default::default(),
            sin_zero: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
