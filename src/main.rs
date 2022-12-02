// use std::fmt::format;
use std::net::UdpSocket;
use std::thread;
use std::time;
//use std::time::Instant;
use rand::Rng;
use std::sync::{Arc, Mutex};
use local_ip_address::local_ip;

use std::fs::File;
use std::io::prelude::*;

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
let mut file = File::create("server_statistics.txt").expect("Error encountered while creating file!");
let thread_join_handle = thread::spawn(move || {
    handle_request(&my_server_socket, &mut file);
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

fn handle_request(socket : &UdpSocket, file : &mut File)
{
    // let sec = time::Duration::from_secs(10);
    // thread::sleep(sec);
    let mut count_requests = 0;
    loop {
        let mut buffer = [0;1000];
        let (_, src_addr) = socket.recv_from(&mut buffer).expect("Didn't receive data");
        //println!("Recieved successsfully from {}",src_addr);
        count_requests = count_requests + 1;
        let client_reply = String::from_utf8(buffer.to_vec()).unwrap();
        println!("client sent : {}",client_reply);
        let reply = String::from("Ack");
        let reply =reply.as_bytes();
        let reply_address = src_addr.to_string();
        // let location = (reply_address.len()-1) as i32;
        // reply_address[location] = "4";
        let use_addr = reply_address.replace("7880", "7885");
        println!("{}", use_addr);
        socket.send_to(reply, &use_addr).expect("couldn't send data");

        if count_requests % 1000 == 0
        {
            let count_str = format!("Number of requests = {}\n\n", count_requests);
            file.write_all(count_str.as_bytes()).unwrap()
        }
        // println!("Sent ack");
    }
}
//we can add token here as well to make it really sleep

fn election(socket : &UdpSocket,gahez : &Arc<Mutex<i32>>) {
    // let my_local_ip = local_ip().unwrap();
    let agent_list = ["10.40.35.100:7882", "10.40.55.187:7882"]; //my_local_ip.to_string()+":7882", 
    let server_list = ["10.40.47.17:6000", "10.40.37.119:6000"];
    //let numservers = server_list.len()+1;
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
            //let mut rng_list = [i32;numservers+1];
            let mut rng_list = [0,0,0];
            rng_list[0] = rng;
            for i in 0..server_list.len()
            {
                socket.send_to(str_rng, &server_list[i]).expect("couldn't send data");
                println!("sent number to server {}", server_list[i]);
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
                let second_digit = buf[1]-48;
                let num = first_digit + second_digit;
                // let num = 50;
                rng_list[i+1] = num.into();
                println!("i recieved number : {}",num);
            }
            

        
            let mut max = 0;
            for i in 0..rng_list.len()
            {
                if rng_list[i] > max{
                    max = rng_list[i];
                }
            }
            

            
        
            println!(" recieved number from other server");
            
            if max != rng_list[0]
            {
                println!("I will not sleep ###############################");
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
        let timer = time::Duration::from_secs(15);
        let mut bufg = [0;1000];
        let server_list = ["10.40.47.17:2155", "10.40.37.119:2155"];
        let mut gahez_1 = {
                    
                    let mut gahez_1 = gahez.lock().unwrap();
                    *gahez_1 = 0;
                    0
                    
            };
            let mut gahez_list = [0,0,0];

                
            while gahez_1 != 1 
            {
                println!("ana mestanni fel gahez /////////////////////////////////////////");
                let ana_gahez:i32 = 1;
                gahez_list[0] = 1;
                let eb3at_gahez = format!("{}",ana_gahez);
                
                let eb3at_gahez = eb3at_gahez.as_bytes();
                for i in 0..server_list.len()
                {
                    socket.send_to(eb3at_gahez, server_list[i]).expect("gahez bayez");

                    let message = socket.recv_from(&mut bufg);
                    match message {
                        Ok((_,_src_addr)) => {
                            println!("recieved gahez message");
                        }
                        Err(_) =>()
                    }
                    

                    //let first_digit = bufg[0] - 48;
                    let first_digit= 1;
                    let enta_gahez = first_digit;
                    //let enta_gahez =  i32::from_be_bytes(buf);
                    gahez_list[i+1] = enta_gahez;
                    println!("enta_gahez is = {}", enta_gahez);

                    //let enta_gahez: i32 = enta_gahez.into();
                }
                
                gahez_1 = 1;
                for i in 0..gahez_list.len()
                {
                    gahez_1 = gahez_1 & gahez_list[i];
                }
                
                //gahez_1 = ana_gahez * enta_gahez;

                
            }
            
            {
                        
                let mut gahez_1 = gahez.lock().unwrap();
                *gahez_1 = 1;
                        
            }
            thread::sleep(timer);
    }
}