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
    let mut buffer = [0;1000];
    let (_, src_addr) = socket.recv_from(&mut buffer).expect("Didn't receive data");
    println!("Recieved successsfully from {}",src_addr);
    let client_reply = String::from_utf8(buffer.to_vec()).unwrap();
    println!("client sent : {}",client_reply);
    let reply = String::from("Ack");
    let reply =reply.as_bytes();
    socket.send_to(reply, src_addr).expect("couldn't send data");
}
//we can add token here as well to make it really sleep

fn election(socket : &UdpSocket,gahez : &Arc<Mutex<i32>>) {
    let my_local_ip = local_ip().unwrap();
    let agent_list = [my_local_ip.to_string()+":1234", "172.20.10.2:1234".to_string()]; 
    loop 
    {
        let gahez_1 = {
            let gahez_1 = gahez.lock().unwrap();
            *gahez_1
        };
        if gahez_1 == 1 
        {
            // let rng:i32 = rand::thread_rng().gen_range(10..99);
            let rng = 10;
            println!("generated rng = {}",rng);
            let mut buf = [0;1000];
            // let str_rng = String::from("goodmorning");
            // let str_rng = str_rng.as_bytes();
            // let mut max = 0;
            let str_rng = format!("{}",rng);
            println!("Sending {}", str_rng);
            let str_rng = str_rng.as_bytes();
            

            socket.send_to(str_rng, "172.20.10.2:6000").expect("couldn't send data");
            println!("sent number to other server");

            let(_,src_addr) = socket.recv_from(&mut buf).unwrap();
            println!("recieved from {}",src_addr);

            let first_digit = (buf[0] - 48)*10;
            println!("Second dig {}", buf[1]);
            let second_digit = buf[1]-48;
            let num = first_digit + second_digit;
            println!("i recieved number : {}",num);

            
        
            println!(" recieved number from other server");
            
            if rng < num.into()
            {
                println!("I will not sleep bitches");
            }
            else {
                println!("Goodnight");
                for i in 0..agent_list.len() {
                    let sleep_message= String::from("Goodnight");
                    
                    socket.send_to(sleep_message.as_bytes(), &agent_list[i]).expect("Didnt recieve data"); // tell agents that you have fallen
                }

                let sec = time::Duration::from_secs(10);
                thread::sleep(sec);
                println!("Goodmorning");
                for i in 0..agent_list.len() {
                
                    let awake_message= String::from("Goodmorning");
                    socket.send_to(awake_message.as_bytes(), &agent_list[i]).expect("Didnt recieve data"); // tell agents that you have fallen
                }
            }
        }
//             let timer = time::Duration::from_secs(10);
// //             for i in 0..agent_list.len() {
// //                 let sleep_message= String::from("sleeping");
// //                 socket.send_to(sleep_message.as_bytes(), &agent_list[i]).expect("Didnt recieve data"); // tell agents that you have fallen
// //             }
// //             println!("sleeping");
// //             thread::sleep(timer);
// //             for i in 0..agent_list.len() {
// //                 let awake_message= String::from("awake");
// //                 socket.send_to(awake_message.as_bytes(), &agent_list[i]).unwrap();   //need to put this in a loop to send to all agents
// //             }
// //         }
// //         else {
// //             println!("I will not sleep"); 
// //         }
//         }

        // let token_2 = {
        //     let token_2 = token_fn.lock().unwrap();
        //     *token_2
        // };
        // if !token_2{
        //     // let sec = time::Duration::from_secs(10);
        //     // thread::sleep(sec);
        //     let mut buf = [0;1000];
        //     let (_, src_addr) = socket.recv_from(&mut buf).expect("Didn't receive data");
        //     println!("Recieved successsfully from {}",src_addr);
        //     let client_reply = String::from_utf8(buf.to_vec()).unwrap();
        //     println!("client sent : {}",client_reply);
        //     let reply = String::from("Ack");
        //     let reply =reply.as_bytes();
        //     socket.send_to(reply, src_addr).expect("couldn't send data");
        // }
        
        

    };
}   


// fn election(socket : &UdpSocket, token_fn : &Arc<Mutex<bool>>, gahez : &Arc<Mutex<i32>>){

    
//     let my_local_ip = local_ip().unwrap();
    
//     //let load_list = ["10.40.62.218:21543"];

//     //const NUM_SERVERS : usize = 2;      //this is num servers - 1 for usage purposes
//     //let duration = time::Duration::from_secs(20);
//     ////socket.set_read_timeout(Some(duration)).unwrap();
//     loop{
//         let agent_list = [my_local_ip.to_string()+":7880"];//, "10.40.53.3:7880".to_string()

//         let gahez_1 = {
//             let gahez_1 = gahez.lock().unwrap();
//             *gahez_1
//         };
//         if gahez_1 == 1 
//         {

//         let rng:i32 = rand::thread_rng().gen_range(10..99);
//         println!("generated rng = {}",rng);
//         let mut buf = [0;4];
//         //let mut max = 0;
//         let str_rng = format!("{}",rng);
//         println!("Sending {}", str_rng);
//         let str_rng = str_rng.as_bytes();
        

//         let (_, src_addr) = socket.recv_from(&mut buf).unwrap();

//         println!(" recieved number from other server");
//         let timer = time::Duration::from_secs(5);
//         thread::sleep(timer);
//         socket.send_to(str_rng, "10.40.54.63:21543").unwrap();

//         println!("recieved from {}",src_addr);
//         //let _recieved = String::from_utf8(buf.to_vec()).unwrap();
        
//         let first_digit = (buf[0] - 48)*10;
//         println!("Second dig {}", buf[1]);
//         let second_digit = buf[1]-48;
//         let num = first_digit + second_digit;
        
        
//         println!("i recieved number : {}",num);

        

        
//             if rng < num.into()
//             {
                    
//                     let mut token_1 = token_fn.lock().unwrap();
//                     *token_1 = false;
                    
                    
//             }
//             else
//             {
                
//                     let mut token_1 = token_fn.lock().unwrap();
//                     *token_1 = true;
                    
//             }
        
//     }
    
//     let token_1 = {
//         let token_1 = token_fn.lock().unwrap();
//         *token_1
//     };

//         if token_1
//         {
            
//             let timer = time::Duration::from_secs(10);
//             for i in 0..agent_list.len() {
//                 let sleep_message= String::from("sleeping");
//                 socket.send_to(sleep_message.as_bytes(), &agent_list[i]).expect("Didnt recieve data"); // tell agents that you have fallen
//             }
//             println!("sleeping");
//             thread::sleep(timer);
//             for i in 0..agent_list.len() {
//                 let awake_message= String::from("awake");
//                 socket.send_to(awake_message.as_bytes(), &agent_list[i]).unwrap();   //need to put this in a loop to send to all agents
//             }
//         }
//         else {
//             println!("I will not sleep"); 
//         }

//         {
//             let mut token_1 = token_fn.lock().unwrap();
//             *token_1 = false;    
//         }

//     }

// }

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

                socket.send_to(eb3at_gahez, "172.20.10.2:2155").expect("gahez bayez");
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
                let first_digit = bufg[0] - 48;
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