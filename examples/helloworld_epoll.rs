use f_stack_binding::constant::*;
use f_stack_binding::*;
use ioctl_rs::*;
use libc::{c_char, c_int, c_ulong, c_void, INADDR_ANY};
use std::ffi::CString;
use std::ptr;

const MAX_EVENTS: c_int = 512;

const REPLY: &'static str = r#"HTTP/1.1 200 OK\r
Server: F-Stack\r
Date: Sat, 25 Feb 2017 09:26:33 GMT\r
Content-Type: text/html\r
Content-Length: 438\r
Last-Modified: Tue, 21 Feb 2017 09:44:03 GMT\r
Connection: keep-alive\r
Accept-Ranges: bytes\r
\r
<!DOCTYPE html>\r
<html>\r
<head>\r
<title>Welcome to F-Stack!</title>\r
<style>\r
    body {  \r
        width: 35em;\r
        margin: 0 auto; \r
        font-family: Tahoma, Verdana, Arial, sans-serif;\r
    }\r
</style>\r
</head>\r
<body>\r
<h1>Welcome to F-Stack!</h1>\r
\r
<p>For online documentation and support please refer to\r
<a href=\http://F-Stack.org/\>F-Stack.org</a>.<br/>\r
\r
<p><em>Thank you for using F-Stack.</em></p>\r
</body>\r
</html>"#;

static mut EPFD: c_int = 0;
static mut SOCK_FD: c_int = 0;

static mut EV: epoll_event = epoll_event {
    events: 0,
    data: epoll_data { u64_: 0 },
};
static mut EVENTS: [epoll_event; MAX_EVENTS as usize] = [epoll_event {
    events: 0,
    data: epoll_data { u64_: 0 },
}; MAX_EVENTS as usize];

extern "C" fn main_loop(_: *mut c_void) -> c_int {
    unsafe {
        let nevents = ff_epoll_wait(EPFD, EVENTS.as_mut_ptr(), MAX_EVENTS, 0);
        for i in 0..nevents {
            let i = i as usize;
            if EVENTS[i].data.fd == SOCK_FD {
                loop {
                    let nclientfd = ff_accept(SOCK_FD, ptr::null_mut(), ptr::null_mut());
                    if nclientfd < 0 {
                        break;
                    }

                    /* Add to event list */
                    EV.data.fd = nclientfd;
                    EV.events = EPOLLIN;
                    if ff_epoll_ctl(
                        EPFD,
                        EPOLL_CTL_ADD as c_int,
                        nclientfd,
                        &mut EV as *mut epoll_event,
                    ) != 0
                    {
                        println!("ff_epoll_ctl failed");
                        break;
                    }
                }
            } else {
                if EVENTS[i].events & EPOLLERR != 0 {
                    /* Simply close socket */
                    ff_epoll_ctl(
                        EPFD,
                        EPOLL_CTL_DEL as c_int,
                        EVENTS[i].data.fd,
                        ptr::null_mut(),
                    );
                    ff_close(EVENTS[i].data.fd);
                } else if EVENTS[i].events & EPOLLIN != 0 {
                    let mut buf = [0 as c_char; 256];
                    let readlen = ff_read(
                        EVENTS[i].data.fd,
                        buf.as_mut_ptr() as *mut c_void,
                        std::mem::size_of::<[c_char; 256]>() as c_ulong,
                    );
                    if readlen > 0 {
                        ff_write(
                            EVENTS[i].data.fd,
                            REPLY as *const _ as *const c_void,
                            REPLY.len() as c_ulong,
                        );
                    } else {
                        ff_epoll_ctl(
                            EPFD,
                            EPOLL_CTL_DEL as c_int,
                            EVENTS[i].data.fd,
                            ptr::null_mut(),
                        );
                        ff_close(EVENTS[i].data.fd);
                    }
                } else {
                    println!("unknown event: {:8.8}", EVENTS[i].events);
                }
            }
        }
    }
    0
}

fn main() {
    unsafe {
        // create a vector of zero terminated strings
        let args = std::env::args()
            .map(|arg| CString::new(arg).unwrap())
            .collect::<Vec<CString>>();

        // convert the strings to raw pointers
        let c_args = args
            .iter()
            .map(|arg| arg.as_ptr() as *mut c_char)
            .collect::<Vec<*mut c_char>>();
        ff_init(c_args.len() as c_int, c_args.as_ptr());

        SOCK_FD = ff_socket(AF_INET as c_int, SOCK_STREAM as i32, 0);
        println!("SOCK_FD: {}", SOCK_FD);
        if SOCK_FD < 0 {
            panic!("ff_socket failed");
        }

        let on = 1;
        ff_ioctl(SOCK_FD, FIONBIO as c_ulong, &on);

        let mut my_addr = sockaddr_in::default();
        my_addr.sin_family = AF_INET as u16;
        my_addr.sin_port = htons(80);
        my_addr.sin_addr.s_addr = htonl(INADDR_ANY);

        let my_addr_linux: *const linux_sockaddr =
            std::mem::transmute(&my_addr as *const sockaddr_in);

        let mut ret = ff_bind(
            SOCK_FD,
            my_addr_linux,
            std::mem::size_of::<sockaddr_in>() as socklen_t,
        );
        if ret < 0 {
            panic!("ff_bind failed");
        }

        ret = ff_listen(SOCK_FD, MAX_EVENTS);
        if ret < 0 {
            panic!("ff_listen failed\n");
        }

        EPFD = ff_epoll_create(0);
        EV.data.fd = SOCK_FD;
        EV.events = EPOLLIN;
        ff_epoll_ctl(
            EPFD,
            EPOLL_CTL_ADD as c_int,
            SOCK_FD,
            &mut EV as *mut epoll_event,
        );

        ff_run(Some(main_loop), ptr::null_mut());
    }
}
