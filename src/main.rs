use std::net::UdpSocket;
use std::thread;
use std::time;
//use std::time::Instant;
use rand::Rng;
use std::sync::{Arc, Mutex};
use local_ip_address::local_ip;


fn main() {
let my_local_ip = local_ip().unwrap();

println!("This is my local IP address: {:?}", my_local_ip);
//let token = Arc::new(Mutex::new(true));

let gahez = Arc::new(Mutex::new(0));


let my_server_socket  = UdpSocket::bind(my_local_ip.to_string()+":21543").expect("couldn't bind to address");
//we can automate this part LATER
    // let (tx, rx) = mpsc::channel();
let election_socket  = UdpSocket::bind(my_local_ip.to_string()+":6000").expect("couldn't bind to address");
let initiate_socket  = UdpSocket::bind(my_local_ip.to_string()+":2155").expect("couldn't bind to address");


//let sock2  = UdpSocket::bind("127.0.0.1:21545").expect("couldn't bind to address");
//let token_main = Arc::clone(&token);

let thread_join_handle = thread::spawn(move || {
    handle_request(&my_server_socket);
});

//let token_main = Arc::clone(&token);

let gahez_main = Arc::clone(&gahez);


let thread_join_handle2 = thread::spawn(move || {
    election(&election_socket, &gahez_main );
});

let gahez_main = Arc::clone(&gahez);


let thread_join_handle3 = thread::spawn(move || {
    gahzeen(&initiate_socket, &gahez_main );
});




let _res = thread_join_handle.join();
thread_join_handle2.join().unwrap();
thread_join_handle3.join().unwrap();
}

fn handle_request(socket : &UdpSocket)
{
    // let sec = time::Duration::from_secs(10);
    // thread::sleep(sec);
    loop {
        let mut buffer = [0;1000];
        let (_, src_addr) = socket.recv_from(&mut buffer).expect("Didn't receive data");
        //println!("Recieved successsfully from {}",src_addr);
        let client_reply = String::from_utf8(buffer.to_vec()).unwrap();
        println!("client sent : {}",client_reply);
        let reply = String::from("Ack");
        let reply =reply.as_bytes();
        socket.send_to(reply, src_addr).expect("couldn't send data");
        // println!("Sent ack");
    }
}
//we can add token here as well to make it really sleep

fn election(socket : &UdpSocket,gahez : &Arc<Mutex<i32>>) {
    let my_local_ip = local_ip().unwrap();
    let agent_list = [my_local_ip.to_string()+":7882", "10.40.55.44:7882".to_string()]; //my_local_ip.to_string()+":7882", 
    loop 
    {
        let gahez_1 = {
            let gahez_1 = gahez.lock().unwrap();
            *gahez_1
        };
        if gahez_1 == 1 
        {
            let rng:i32 = rand::thread_rng().gen_range(10..99);
            // let rng = 10;
            println!("generated rng = {}",rng);
            let mut buf = [0;1000];
            // let str_rng = String::from("goodmorning");
            // let str_rng = str_rng.as_bytes();
            // let mut max = 0;
            let str_rng = format!("{}",rng);
            println!("Sending {}", str_rng);
            let str_rng = str_rng.as_bytes();
            

            socket.send_to(str_rng, "10.40.55.44:6000").expect("couldn't send data");
            println!("sent number to other server");

            let mut error_flag = true;
            while error_flag {
                let case = socket.recv_from(&mut buf);
            match case {
                Ok((_,src_addr)) => {
                    println!("Recieved successsfully from {}",src_addr);
                    error_flag = false;
                }
                Err(_) =>error_flag = true
            }
        }

            let first_digit = (buf[0] - 48)*10;
            println!("Second dig {}", buf[1]);
            let second_digit = buf[1]-48;
            let num = first_digit + second_digit;
            // let num = 50;
            println!("i recieved number : {}",num);

            
        
            println!(" recieved number from other server");
            
            if rng < num.into()
            {
                println!("I will not sleep bitches");
            }
            else {
                println!("Goodnight ###################################");
                for i in 0..agent_list.len() {
                    let sleep_message= String::from("sleep");
                    
                    socket.send_to(sleep_message.as_bytes(), &agent_list[i]).expect("Didnt recieve data"); // tell agents that you have fallen
                }

                let sec = time::Duration::from_secs(10);
                thread::sleep(sec);
                println!("Goodmorning ###############################");
                for i in 0..agent_list.len() {
                
                    let awake_message= String::from("awake");
                    socket.send_to(awake_message.as_bytes(), &agent_list[i]).expect("Didnt recieve data"); // tell agents that you have fallen
                }
            }
        }

    };
}   

fn gahzeen(socket : &UdpSocket, gahez : &Arc<Mutex<i32>>){
    loop{
        let mut bufg = [0;1000];
        let mut gahez_1 = {
                    
                    let mut gahez_1 = gahez.lock().unwrap();
                    *gahez_1 = 0;
                    0
                    
            };
                    
            while gahez_1 != 1 
            {
                println!("ana mestanni fel gahez");
                let ana_gahez:i32 = 1;

                let eb3at_gahez = format!("{}",ana_gahez);
                
                let eb3at_gahez = eb3at_gahez.as_bytes();

                socket.send_to(eb3at_gahez, "10.40.55.44:2155").expect("gahez bayez");
                // let timer = time::Duration::from_secs(5);
                // thread::sleep(timer);
                println!("ana ba3at kosom el gahez");
                let message = socket.recv_from(&mut bufg);
                match message {
                    Ok((_,_src_addr)) => {
                        println!("recieved gahez message");
                    }
                    Err(_) => println!("error when received gahez message")
                }
                let first_digit = 1;        //bufg[1];
                let enta_gahez = first_digit;
                //let enta_gahez =  i32::from_be_bytes(buf);

                println!("enta_gahez is = {}", enta_gahez);

                let enta_gahez: i32 = enta_gahez.into();
                
                gahez_1 = ana_gahez * enta_gahez;

                
            }
            
            
            {
                        
                let mut gahez_1 = gahez.lock().unwrap();
                *gahez_1 = 1;
                        
            }
            let timer = time::Duration::from_secs(15);
        
            thread::sleep(timer);
    }
}