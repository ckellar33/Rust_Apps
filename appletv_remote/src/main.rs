use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::net::TcpStream;
use std::array;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
// use tokio::prelude::*;
use std::{error::Error, time::Duration};
use prost::Message;
use futures_util::{pin_mut, StreamExt};
use prost_types::Any;
use crate::rust_protobuf_protos::ProtocolMessage::ProtocolMessage;
use crate::rust_protobuf_protos::DeviceInfoMessage::DeviceInfoMessage;
use protobuf::reflect::MessageDescriptor;

use protobuf::ext::ExtFieldOptional;

pub mod protos {
    include!(concat!(env!("OUT_DIR"), "/_.rs"));
}
mod rust_protobuf_protos {
    include!(concat!(env!("OUT_DIR"), "/rust_protobuf_protos/mod.rs"));
}

const SERVICE_NAME: &'static str = "_airplay._tcp.local";

async fn send_and_receive_message(socket_addr: SocketAddr) -> Result<(), Box<dyn Error>> {

    // let message = ProtocolMessage::new();
    // let extension_field = MessageDescriptor::for_type::<DeviceInfoMessage>()
    //     .oneof_by_name("device_info")
    //     .unwrap();

    let device_info = protos::DeviceInfoMessage {
        unique_identifier: "B8D8678C-9DA9-4D29-9338-5D6B827B8063".to_string(),
        name: "Chris Kellar's iPhone".to_string(),
        localized_model_name: Some("iPhone16".to_string()),
        system_build_version: "13F69".to_string(),
        application_bundle_version: Some("".to_string()),
        application_bundle_identifier: "".to_string(),
        protocol_version: 1,
    };

    // Create the message
    let message = protos::ProtocolMessage {
        r#type: protos::protocol_message::Type::DeviceInfoMessage.into(),
        identifier: Some("96A3A0EB-3584-4648-9285-2F771397CFB8".to_string()),
        authentication_token: None,
        error_code: Some(0),
        timestamp: None,
        device_info_message: Some(device_info),
    };

    // Serialize it to bytes
    let mut buf = Vec::new();
    message.encode(&mut buf)?;

    // Connect to the server (localhost:5000)
    let (ip, port) = match socket_addr {
        SocketAddr::V4(socket) => {
            println!("IP: {}, Port: {}", socket.ip(), socket.port());
            (*socket.ip(), socket.port())    
        }
        SocketAddr::V6(socket) => {
            println!("IP: {}, Port: {}", socket.ip(), socket.port());
            return Err("IPv6 not supported".into());
        }
    };
    println!("Connecting to server at {}", socket_addr.ip().to_string());
    let mut stream = TcpStream::connect(socket_addr).await?;

    // Send the serialized message to the server
    stream.write_all(&buf).await?;
    println!("Sent message to server: {:?}: {:02x?}", message, buf);

    // Now, receive the response from the server
    let mut response_buf = Vec::new();
    stream.read_to_end(&mut response_buf).await?;

    // Deserialize the server's response into a protobuf message
    let response = protos::ProtocolMessage::decode(&*response_buf)?;

    // Print the server's response
    println!("Received response from server: type={}, id={:?}, timestamp={:?}", response.r#type, response.identifier, response.timestamp);

    Ok(())
}

#[tokio::main]
async fn main() {
    let service_name = SERVICE_NAME;
    let stream = match mdns::discover::all(service_name, Duration::from_secs(2)) {
        Ok(discovery) => discovery.listen(),
        Err(e) => {
            panic!("Error: {:?}", e);
        }
    };

    pin_mut!(stream);

    let mut family_room : SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5000);

    println!("Scanning for devices...");
    let mut found = false;
    while let Some(Ok(response)) = stream.next().await {
        let addr = response.socket_address();

        for record in response.records() {
            println!("Record: {:?}", record);
            match &record.kind {
                mdns::RecordKind::A(addr) => {
                    if record.name.contains("Family") {
                        family_room = SocketAddr::new(IpAddr::V4(*addr), 49152);
                        found = true;
                        break;
                    }
                },
                _ => {}
            }
        }

        if found {
            break;
        }
    }

    println!("Trying to connect to Family Room...");
    match send_and_receive_message(family_room).await {
        Ok(_) => println!("Successfully sent and received message"),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}