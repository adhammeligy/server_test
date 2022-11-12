use std::net::UdpSocket;
use std::thread;
// use std::sync::mpsc;
//use std::time;

fn main() {

let socket  = UdpSocket::bind("127.0.0.1:7879").expect("couldn't bind to address");
    // let (tx, rx) = mpsc::channel();
let _sock  = UdpSocket::bind("127.0.0.1:21543").expect("couldn't bind to address");

//let sock2  = UdpSocket::bind("127.0.0.1:21545").expect("couldn't bind to address");

let thread_join_handle = thread::spawn(move || {
    handle_request(&socket);
});

//let thread_join_handle2 = thread::spawn(move || {
//    election(&sock);
//});

//let thread_join_handle3 = thread::spawn(move || {
//    send_to_servers(&sock2);
//});


let _res = thread_join_handle.join();
//thread_join_handle2.join().unwrap();


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

//fn election(socket : &UdpSocket){
//    let duration = time::Duration::from_secs(1);
//    socket.set_read_timeout(Some(duration)).unwrap();
//    loop{
//        let mut load_list: [String; 3] = Default::default();        
//        let mut buf = [0;1000];
//        let mut i = 0;

//        let timer = time::Duration::from_secs(1);

//        let recv_res = socket.recv_from(&mut buf);
//        match recv_res {
//            Ok((_, _src_addr)) =>  println!("ok"),
//            Err(_) => ()
//        }
//        let load = String::from_utf8(buf.to_vec()).unwrap();
//        load_list[i] = load ;
//        i+=1;
//        if i == 3 {
//             i = 0;
//        }
//        thread::sleep(timer);
//        println!("1 Sec Elapsed");
//    }
//}
//127.0.0.1:7878
//fn send_to_servers(socket : &UdpSocket){

//}