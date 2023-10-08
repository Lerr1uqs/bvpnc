use std::io;
use std::net::UdpSocket;
use std::sync::Arc;
// ------------ extern -----------------
use wintun;
use tokio;

struct UserArgs {
    tun_name: String,
    tun_ip: String, // e.g. 192.168.0.1
    mask: u32,  // e.g. 24
    port: u32,
    srv_addr: String
}
fn main() {
    // --------------- temp ----------------
    let config = UserArgs {
        tun_name: "tun0".into(), 
        tun_ip: "192.168.0.1".into(),
        mask: 24,
        port: 55555,
        srv_addr: "192.168.110.128".into()
    };
    // -------------------------------------

    // let main_channel = UdpSocket::bind(format!("{}:{}", config.tun_ip ,config.port)).unwrap();

    // 1.connect 

    // 2.create if(tun dev)

    // 3.tun recv data transfer to 


    
    start();


}
#[tokio::main]
async fn start(){// TODO: ioresult

    let wintun = unsafe { 
        wintun::load_from_path("F:\\Programming\\bvpnc\\wintun.dll")
               .expect("Failed to load wintun dll") 
    };
    
    //Try to open an adapter with the name "Demo"
    let adapter = match wintun::Adapter::open(&wintun, "tun0") {// TODO:
        Ok(a) => a,
        Err(_) => {
            //If loading failed (most likely it didn't exist), create a new one
            wintun::Adapter::create(&wintun, "tun0", "Example"/* TODO: */, None)
                            .expect("Failed to create wintun adapter!")
        }
    };

    let session = Arc::new(adapter.start_session(wintun::MAX_RING_CAPACITY).unwrap());
    
    loop {
        tokio::select! {
            _ = async {1} => {

            }
        }

    }

    session.shutdown().unwrap();

}