use crate::types::*;
use std::ffi::c_void;
use std::ptr::NonNull;
pub use wasmer_wasi_types::*;

#[link(wasm_import_module = "wasi_experimental_network_unstable")]
extern "C" {
    /// `socket_create` creates an endpoint for communication.
    pub fn socket_create(
        fd_out: *mut __wasi_fd_t,
        domain: __wasi_socket_domain_t,
        r#type: __wasi_socket_type_t,
        protocol: __wasi_socket_protocol_t,
    ) -> __wasi_errno_t;

    /// When a socket is created (with [`socket_create`]), it exists
    /// in a name space (address family) but has no address assigned
    /// to it. `socket_bind` assigns the address specified by
    /// `address` (see [`SocketAddress`] and [`SocketAddress6`]) to
    /// the socket referred to by the file descriptor
    /// `fd`. `address_size` specifies the size, in bytes, of the
    /// address structure pointed to by `address`. Traditionnaly, this
    /// operation is called “assigning a name to a socket”.
    pub fn socket_bind(
        fd: __wasi_fd_t,
        address: NonNull<c_void>,
        address_size: u32,
    ) -> __wasi_errno_t;

    /// `socket_listen` marks the socket referred to by `fd` as a
    /// passive socket, that is, a socket that will be used to accept
    /// incoming connection requests using [`socket_accept`].
    pub fn socket_listen(fd: __wasi_fd_t) -> __wasi_errno_t;

    /// The `socket_accept` function is used with connection-based
    /// socket types (e.g. [`SOCK_STREAM`]). It extracts the first
    /// connection request on the queue of pending connections for the
    /// listening socket, `fd`, creates a new connected socket, and
    /// returns a new file descriptor referring to that socket in
    /// `fd_out`. The newly created socket is not in the listening
    /// state. The original socket `fd` is unaffected by this call.
    ///
    /// The argument `fd` is a socket that has been created with
    /// [`socket_create`], bound to a local address with
    /// [`socket_bind`], and is listening for connections after a
    /// [`socket_listen`].
    ///
    /// The argument `address` is a pointer to an address (see
    /// [`SocketAddress`] and [`SocketAddress6`]). This structure is
    /// filled in with the address of the peer socket, as known to the
    /// communications layer. The exact format of the address returned
    /// `address` is determined by the socket's address family (see
    /// [`socket_create`]). When `address` is `null`, nothing is
    /// filled in; in this case, `address_size` is not used, and
    /// should also be `null`.
    pub fn socket_accept(
        fd: __wasi_fd_t,
        address: *mut u8,
        address_size: *mut u32,
        fd_out: *mut __wasi_fd_t,
    ) -> __wasi_errno_t;

    /// The `socekt_connect` function connects the socket referred to
    /// by the file descriptor `fd` to the address specified by
    /// `address`. The `address_size` argument specifies the size of
    /// `address`. The format of the address in `address` is
    /// determined by the address space of the socket `fd`; see
    /// [`socket_create`].
    ///
    /// If the socket `fd` is of type `SOCK_DGRAM` then `address` is
    /// the address to which datagrams are sent by default, and the
    /// only address from which datagrams are received. If the socket
    /// is of type `SOCK_STREAM`, this call attempts to make a
    /// connection to the socket that is bound to the address by
    /// `address`.
    pub fn socket_connect(
        fd: __wasi_fd_t,
        address: NonNull<c_void>,
        address_size: u32,
    ) -> __wasi_errno_t;

    /// The `socket_send` function is used to transmit a message to
    /// another socket referred to by the file descriptor `fd`. This
    /// function works like `writev(2)`. It writes `iov_size` buffers
    /// of data described by `iov` (I/O vector) to the file associated
    /// with the file descriptor `fd` (“gather output”).
    ///
    /// The `iov_flags` represents the “write flags”, represented by
    /// `__wasi_siflags_t`.
    ///
    /// `io_size_out` is filled in by the number of bytes actually
    /// written.
    pub fn socket_send(
        fd: __wasi_fd_t,
        iov: NonNull<__wasi_ciovec_t>,
        iov_size: u32,
        iov_flags: __wasi_siflags_t,
        io_size_out: *mut u32,
    ) -> __wasi_errno_t;

    /// The `socket_recv` function is used to receive messages from a
    /// socket referred to by the file descriptor `fd`. This function
    /// works like `readv(2)`. It reads `iov_size` buffers from the
    /// file associated with the file descriptor `fd` into buffers
    /// described by `iov` (“scatter input”).
    ///
    /// The `iov_flags` represents the “read flags”, represented by
    /// `__wasi_siflags_t`.
    ///
    /// `io_size_out` is filled in by the number of bytes actually
    /// read.
    pub fn socket_recv(
        fd: __wasi_fd_t,
        iov: NonNull<__wasi_ciovec_t>,
        iov_size: u32,
        iov_flags: __wasi_siflags_t,
        io_size_out: *mut u32,
    ) -> __wasi_errno_t;

    /// The `shutdown` function causes all or part of a full-duplex
    /// connection on the socket with `fd` to be shut down.
    pub fn socket_shutdown(fd: __wasi_fd_t) -> __wasi_errno_t;
}