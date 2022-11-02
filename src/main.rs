use std::net::UdpSocket;
use std::thread;
// use std::sync::mpsc;
use std::time;

fn main() {

let socket  = UdpSocket::bind("127.0.0.1:7878").expect("couldn't bind to address");
    // let (tx, rx) = mpsc::channel();
let sock  = UdpSocket::bind("127.0.0.1:21543").expect("couldn't bind to address");

let thread_join_handle = thread::spawn(move || {
    handle_request(&socket);
});

let thread_join_handle2 = thread::spawn(move || {
    recieve_from_servers(&sock);
});


let _res = thread_join_handle.join();
thread_join_handle2.join().unwrap();


        // thread::spawn(move || {
        //     let (_, src_addr) = socket.recv_from(&mut buf).expect("Didn't receive data");
        //     println!("Recieved successsfully from {}",src_addr);
        //     tx.send(src_addr).unwrap();
        //     thread::sleep(Duration::from_secs(1));
        // });
        // let src_addr = rx.recv().unwrap();
        // let client_reply = String::from_utf8(buf.to_vec()).unwrap();
        // println!("client sent : {}",client_reply);
        // let reply = String::from("Ack");
        // let reply = reply.as_bytes();
        // println!("Source address = {}", src_addr);
        // tx.send(src_addr).unwrap();
        // thread::spawn(move || {
        //     let addr = rx.recv().unwrap();
        //     socket.send_to(reply, addr).expect("couldn't send data");
        // });
}

fn handle_request(socket : &UdpSocket) {
    loop 
    {
        let mut buf = [0;1000];
        let (_, src_addr) = socket.recv_from(&mut buf).expect("Didn't receive data");
        println!("Recieved successsfully from {}",src_addr);
        let client_reply = String::from_utf8(buf.to_vec()).unwrap();
        println!("client sent : {}",client_reply);
        let reply = String::from("Ack");
        let reply =reply.as_bytes();
        socket.send_to(reply, src_addr).expect("couldn't send data");
    };
}   

fn recieve_from_servers(socket : &UdpSocket){
    let duration = time::Duration::from_secs(1);
    socket.set_read_timeout(Some(duration)).unwrap();
    loop{
        let mut buf = [0;1000];

        let timer = time::Duration::from_secs(1);

        let recv_res = socket.recv_from(&mut buf);
        match recv_res {
            Ok((_, src_addr)) =>  println!("Recieved successsfully from server {}",src_addr) ,
            Err(_) => ()
        }
        thread::sleep(timer);
        println!("1 Sec Elapsed");
    }
}
//127.0.0.1:7878