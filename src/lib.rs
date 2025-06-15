use std::{mem::{transmute, zeroed}, net::Ipv4Addr};
use libc::{c_char, c_int, dlsym, sa_family_t, sockaddr, sockaddr_in, socklen_t, AF_INET, RTLD_NEXT};


type ConnectFN = unsafe extern "C" fn (
    c_int,
    *const sockaddr,
    socklen_t,
) -> c_int;


#[no_mangle]
pub unsafe extern "C" fn connect(
    socket: c_int,
    address: *const sockaddr,
    len: socklen_t,
) -> c_int {

    //pointer to original connect function
    let original: ConnectFN = transmute(dlsym(RTLD_NEXT, "connect\0".as_ptr() as *const c_char));

    if !address.is_null() {

        let address_type = (*address).sa_family as c_int;
        // for ipv4 only
        if address_type == AF_INET {

            // original ip and port
            let address = *(address as *const sockaddr_in);
            let ip = address.sin_addr.s_addr.to_le();
            let port = u16::from_be(address.sin_port);
            println!("Original Address:\nIP: {}\nPORT: {}", Ipv4Addr::from(ip), port);

            // connect to proxy
            // modifying request ip -> 127.0.0.1 and port -> 4444
            let mut new_address: sockaddr_in = zeroed();
            new_address.sin_family = AF_INET as sa_family_t;
            new_address.sin_addr.s_addr = u32::from_be_bytes([127,0,0,1]).to_be();
            new_address.sin_port = 4444_u16.to_be();

            let ip = u32::from_be(new_address.sin_addr.s_addr).to_le();
            let port = u16::from_be(new_address.sin_port);
            println!("\nRedirecting the connection.....\nIP: {}\nPORT: {}", Ipv4Addr::from(ip), port);

            // sending proxy connection toward nc -lnvp 4444
            return original(socket, &new_address as *const sockaddr_in as *const sockaddr, len);

        }
    }
    
    -1
}




