use std::net::UdpSocket;
use std::thread;
use std::time;
use rand::Rng;
use std::sync::{Arc, Mutex};
    

fn main() {
let token = Arc::new(Mutex::new(true));

let socket  = UdpSocket::bind("127.0.0.1:7879").expect("couldn't bind to address");
//we can automate this part LATER
    // let (tx, rx) = mpsc::channel();
let sock  = UdpSocket::bind("127.0.0.1:21543").expect("couldn't bind to address");

//let sock2  = UdpSocket::bind("127.0.0.1:21545").expect("couldn't bind to address");
let token_main = Arc::clone(&token);

let thread_join_handle = thread::spawn(move || {
    handle_request(&socket, &token_main);
});

let token_main = Arc::clone(&token);

let thread_join_handle2 = thread::spawn(move || {
    election(&sock, &token_main);
});

//let thread_join_handle3 = thread::spawn(move || {
//    send_to_servers(&sock2);
//});


let _res = thread_join_handle.join();
thread_join_handle2.join().unwrap();

}
//we can add token here as well to make it really sleep
fn handle_request(socket : &UdpSocket, token_fn : &Arc<Mutex<bool>>) {
    loop 
    {
        let token_2 = token_fn.lock().unwrap();
        if *token_2{
            let sec = time::Duration::from_secs(10);
            thread::sleep(sec);

        }
        std::mem::drop(token_2);

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

fn election(socket : &UdpSocket, token_fn : &Arc<Mutex<bool>>){
    const NUM_SERVERS : usize = 2;
    let duration = time::Duration::from_secs(1);
    socket.set_read_timeout(Some(duration)).unwrap();
    loop{
        let load_list = ["127.0.0.1:7879","127.0.0.1:7878"];      
        let mut buf = [0;4];
        let mut i = 0;
        
        let sleep_message= String::from("0");
        let awake_message= String::from("1");

        let timer = time::Duration::from_secs(30);
        let mut token_1 = token_fn.lock().unwrap();

        if *token_1 {

            socket.send_to(sleep_message.as_bytes(), "127.0.0.1:7878").unwrap();
            thread::sleep(timer);
            socket.send_to(awake_message.as_bytes(), "127.0.0.1:7878").unwrap();

        }
        else
        {
            let (_, _src_addr) = socket.recv_from(&mut buf).expect("Didn't receive data");   //keep blocking until election initiation
            
            let mut list: [i32; NUM_SERVERS] = Default::default(); 
            let rng:i32 = rand::thread_rng().gen_range(1..=100);
            let random = rng.to_ne_bytes();
            list[i]=rng;
            i=i+1;
            while i < NUM_SERVERS 
            {
                socket.send_to(&random, load_list[i]).expect("couldn't send data");
                let (_, _src_addr) = socket.recv_from(&mut buf).expect("Didn't receive data");
                //append buf to list and clear it
                let rand = i32::from_ne_bytes(buf);
                list[i] = rand;
                i = i + 1;
            }

            i = 0;
            let mut max= 0;

            while i < NUM_SERVERS {
                if list[i] > max {
                    max = list[i];
                }
                i = i + 1;
            }
            if max == list[0] {
                *token_1 = true;
            }
            else {
                *token_1 = false;
            }

            
        }
    std::mem::drop(token_1);

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
    }
}
//127.0.0.1:7878
//fn send_to_servers(socket : &UdpSocket){

//}

//pseudocode:
/*
while(1)
{
    if(token)
    {
        eb3at lel agent sleep
        sleep 30 sanya
        eb3at lel agent awake
        eb3at le server tany el token
    }
    else
    {
        keep listening for someone to send token
    }

}
*/