
//! ## About this library
//! 
//! 
//! This library is designed to facilitate communication between the [Critter & Guitarri Organelle](https://www.critterandguitari.com/organelle)
//! and Rust.
//! 
//! This library can be used for:
//! 
//! - Receiving OSC messages from the Organelle's keyboard and knob
//! - Toggling the Organelle's patchLoaded flag
//! - Sending OSC commands to the Organelle's screen
//! 
//! The main goal is to make it easier to interface with the Organelle's hardware inputs and outputs.


extern crate rosc;

use rosc::{OscMessage, OscPacket, OscType};
use rosc::encoder;

use std::net::{UdpSocket};




enum OledMsg {CLEAR, UPDATE}


/// The HwInterface struct can be used for received OSC signals from the Organelle harware 
/// and sending OSC messages back to the Organelle in order to control it's OLED screen.
pub struct HWInterface{
    receiver : UdpSocket,
    sender : UdpSocket
}


 impl HWInterface{


    fn constrain (input : &i32, min : &i32, max : &i32) -> i32 {
        let mut result = input.clone();
        let min_value = min.clone();
        let max_value = max.clone();
        if &result < &min_value {
            result = min_value;
        }
        else if &result > &max_value {
            result = max_value
        }

        result
    }


    /// Creates new instance of class
    pub fn new () -> Self {
        
        Self {
            // addr : SocketAddrV4::from_str(&receive_addr).unwrap(),
            receiver: UdpSocket::bind("127.0.0.1:4000".to_string()).unwrap(),
            sender: UdpSocket::bind("0.0.0.0:0").unwrap()
            
        }
    }

    /// Connects the sender UDP socket to `localhost:4001` (port used for Organelle's OLED messages)
    pub fn connect(&mut self) {
        self.sender.connect("127.0.0.1:4001".to_string()).expect("Failed to connect socket to port.");
        // connect
    }


    fn oled(&mut self, msg_type: OledMsg) {
        // let send_sock = UdpSocket::bind(addr).unwrap();
        let msg_str = match &msg_type {
            OledMsg::CLEAR => "/oled/gClear".to_string(),
            OledMsg::UPDATE => "/oled/gFlip".to_string()
        };
        

        let msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
            addr: msg_str,
            args: vec![OscType::Int(3), OscType::Int(1)],
        }))
        .unwrap();
        self.sender.send(&msg_buf).expect("couldn't send message");
    //     println!("{:?}", msg_buf);
    }

    /// Clears the OLED screen
    pub fn oled_clear(&mut self) {
        self.oled(OledMsg::CLEAR);
    }

    /// Updates the OLED screen
    pub fn oled_update(&mut self) {
        self.oled(OledMsg::UPDATE);
    }


    /// Toggles the display of the infobar on the Organelle's OLED
    pub fn oled_showinfobar(&mut self, status : bool) {
        let msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
                addr: "/oled/gShowInfoBar".to_string(),
                args: vec![OscType::Int(3), OscType::Int(status as i32)],
            })).unwrap();
        self.sender.send(&msg_buf).expect("couldn't send message");
    }


    /// Set the Organelle to behave as if there's an open patch.
    pub fn patchloaded(&mut self, status : i32) {
        let msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
                addr: "/patchLoaded".to_string(),
                args: vec![OscType::Int(status)],
            })).unwrap();
        self.sender.send(&msg_buf).expect("couldn't send message");
    }


    /// Initialzies the Organelle OLED by enabling the `patchLoaded` flag, 
    /// clearing the screen and disabling the info bar.
    pub fn oled_initialize(&mut self) {
        self.patchloaded(1);
        self.oled_clear();
        self.oled_showinfobar(false);
        // self.oled(_)
    }


    /// Replicates the funcionality of the `/oled/gFillArea` OSC message
    pub fn oled_fillarea(&mut self, pos_x : i32, pos_y : i32, width : i32, heigth : i32, color: i32 ) {
        let msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
                addr: "/oled/gFillArea".to_string(),
                args: vec![OscType::Int(3), OscType::Int(pos_x), OscType::Int(pos_y), OscType::Int(width), OscType::Int(heigth), OscType::Int(color)],
            })).unwrap();
        self.sender.send(&msg_buf).expect("couldn't send message");
    }

    /// Replicates the funcionality of the `/oled/gFillArea` OSC message
    pub fn oled_box(&mut self, pos_x : i32, pos_y : i32, width : i32, heigth : i32, color: i32 ) {
        let msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
                addr: "/oled/gBox".to_string(),
                args: vec![OscType::Int(3), OscType::Int(pos_x), OscType::Int(pos_y), OscType::Int(width), OscType::Int(heigth), OscType::Int(color)],
            })).unwrap();
        self.sender.send(&msg_buf).expect("couldn't send message");
    }

    /// Replicates the funcionality of the `/oled/gLine` OSC message
    pub fn oled_line(&mut self, x_start : i32, y_start : i32, x_stop : i32, y_stop : i32 ) {
        let msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
                addr: "/oled/gLine".to_string(),
                args: vec![OscType::Int(3), OscType::Int(x_start), OscType::Int(y_start), OscType::Int(x_stop), OscType::Int(y_stop), OscType::Int(1)],
            })).unwrap();
        self.sender.send(&msg_buf).expect("couldn't send message");
    }

    /// Replicates the funcionality of the `/oled/gPrintln` OSC message
    pub fn oled_println(&mut self, pos_x : i32, pos_y : i32, size : i32, text : String ) {
        let msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
                addr: "/oled/gPrintln".to_string(),
                args: vec![OscType::Int(3), OscType::Int(pos_x), OscType::Int(pos_y), OscType::Int(size), OscType::Int(1), OscType::String(text)],
            })).unwrap();
        self.sender.send(&msg_buf).expect("couldn't send message");
    }


    /// Enables the use of the encoder
    pub fn enable_patch_submenu (&mut self, status : i32) {
        let msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
            addr: "/enablepatchsub".to_string(),
            args: vec![OscType::Int(status)],
        })).unwrap();
        self.sender.send(&msg_buf).expect("couldn't send message");
    }

    pub fn go_home (&mut self) {
        let msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
            addr: "/gohome".to_string(),
            args: vec![OscType::Int(1)],
        })).unwrap();
        self.sender.send(&msg_buf).expect("couldn't send message");
    }

    pub fn reload (&mut self) {
        let msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
            addr: "/reload".to_string(),
            args: vec![OscType::Int(1)],
        })).unwrap();
        self.sender.send(&msg_buf).expect("couldn't send message");
    }

    pub fn led(&mut self, color: i32) {
        let msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
            addr: "/led".to_string(),
            args: vec![OscType::Int(color)],
        })).unwrap();
        self.sender.send(&msg_buf).expect("couldn't send message");
    }

    /// Receives a message from receiver UDP port and returns the incoming OSC message
    pub fn receive(&mut self) -> (String, Vec<i32>) {
        let mut buf = [0u8; rosc::decoder::MTU];
        
        let mut received_msg = "".to_string();
        let mut msg_vec = vec![];
        match self.receiver.recv_from(&mut buf) {
            Ok((size, _)) => {
                // println!("Received packet with size {} from: {}", size, addr);
                let osc_result = rosc::decoder::decode(&buf[..size]).unwrap();
                match &osc_result {
                    OscPacket::Message(msg) => {
                        received_msg = msg.addr.clone();
                        for x in msg.args.iter() {
                            msg_vec.push(x.clone().int().unwrap())
                        }
                    },
                    OscPacket::Bundle(_) => {}
                }
                
                
                // handle_packet(packet, &send_sock);
            },
            Err(_) => {
            }
        }
        (received_msg, msg_vec)

    }


}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
       let mut newOSC = super::HWInterface::new();
       newOSC.connect();
       newOSC.oled_initialize();
       newOSC.oled_line(0,10, 127, 10);
       newOSC.oled_line(0, 24, 127, 24);
       newOSC.oled_println(0, 14, 8, "This is a RUST test".to_string());
    //    let result = newOSC.receive();
    //     println!("{:?}", result);
    //    loop {
        
        // assert!(result);
        // println!("{:?}", result);
            // match result {
            //         super::OscPacket::Message(msg) => { 
            //             println!("{:?}", msg.args)
            //             // let knobvalue1 = msg.args[0].clone().int().unwrap();
            //             // let knobvalue2 = msg.args[1].clone().int().unwrap();
            //             // let knobvalue3 = msg.args[2].clone().int().unwrap();
            //             // let knobvalue4 = msg.args[3].clone().int().unwrap();
            //             // newOSC.oled_println(&15, &40, &16, knobvalue1.to_string());
            //         }
            //         super::OscPacket::Bundle(msg) => {
            //             println!("{:?}", msg)
            //         }
            // }
    //         newOSC.oled_update();
    //    }
        assert_eq!(result, Some);
    }
}
