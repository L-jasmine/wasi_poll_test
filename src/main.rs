fn main() {
    println!("Hello, world!");
    unsafe {try_net_read_poll_oneoff()}
}


unsafe fn try_net_read_poll_oneoff() {
    let mut fd: u32 = 0;
    sock_open(
        AddressFamily::Inet4 as u8,
        SocketType::Stream as u8,
        &mut fd,
    );
    println!("open a tcp socket({})",fd);

    // bind 0.0.0.0:8000
    let wasi_addr = [0,0,0,0];
    let mut wasi_addr = WasiAddress{ buf: wasi_addr.as_ptr(), size:4 };
    sock_bind(fd, &mut wasi_addr, 8000 as u32);
    sock_listen(fd,128);
    println!("fd:{} bind(0.0.0.0:8000) & listen(128)",fd);

    println!("poll...");
    let poll_fd = libc::pollfd{
        fd: fd as i32,
        events: libc::POLLRDNORM,
        revents: 0
    };
    let mut poll_fds = [poll_fd];
    let n = libc::poll((&mut poll_fds).as_mut_ptr(), 1, 1);

    let poll_fd = &poll_fds[0];
    println!("poll_return:{}, poll_fd.revents:{}",n,poll_fd.revents);
}

#[derive(Copy, Clone)]
#[repr(u8)]
enum SocketType {
    Datagram,
    Stream,
}

#[derive(Copy, Clone)]
#[repr(u8)]
enum AddressFamily {
    Inet4,
    Inet6,
}

pub struct WasiAddress {
    pub buf: *const u8,
    pub size: usize,
}

pub struct IovecRead {
    pub buf: *mut u8,
    pub size: usize,
}
pub struct IovecWrite {
    pub buf: *const u8,
    pub size: usize,
}

#[link(wasm_import_module = "wasi_snapshot_preview1")]
extern "C" {
    pub fn sock_open(addr_family: u8, sock_type: u8, fd: *mut u32) -> u32;
    pub fn sock_close(fd: u32);
    pub fn sock_bind(fd: u32, addr: *mut WasiAddress, port: u32) -> u32;
    pub fn sock_listen(fd: u32, backlog: u32) -> u32;
    pub fn sock_accept(fd: u32, port: u32, new_fd: *mut u32) -> u32;
    pub fn sock_connect(fd: u32, addr: *mut WasiAddress, port: u32) -> u32;
    pub fn sock_recv(
        fd: u32,
        buf: *const IovecRead,
        buf_len: usize,
        flags: u16,
        recv_len: *mut usize,
        oflags: *mut usize,
    ) -> u32;
    pub fn sock_recv_from(
        fd: u32,
        buf: *mut u8,
        buf_len: u32,
        addr: *mut u8,
        addr_len: *mut u32,
        flags: u16,
    ) -> u32;
    pub fn sock_send(
        fd: u32,
        buf: *const IovecWrite,
        buf_len: u32,
        flags: u16,
        send_len: *mut u32,
    ) -> u32;
    pub fn sock_send_to(
        fd: u32,
        buf: *const u8,
        buf_len: u32,
        addr: *const u8,
        addr_len: u32,
        flags: u16,
    ) -> u32;
    pub fn sock_shutdown(fd: u32, flags: u8) -> u32;
}