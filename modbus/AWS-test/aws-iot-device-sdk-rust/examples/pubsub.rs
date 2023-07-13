use serde_json::json;
use rumqttc::{self, Packet, QoS};
use std::error::Error;
use aws_iot_device_sdk_rust::client::{AWSIoTSettings, AWSIoTAsyncClient, async_event_loop_listener};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let aws_settings = AWSIoTSettings::new(
        "Rust-Thing ".to_owned(),
        r"C:\Users\smart\OneDrive\ドキュメント\Rust\modbus\AWS-test\aws-iot-device-sdk-rust\cert\root-CA.crt".to_owned(),
        r"C:\Users\smart\OneDrive\ドキュメント\Rust\modbus\AWS-test\aws-iot-device-sdk-rust\cert\26c29384a2f03b7ff8818d47b68b661f5699617fb21fa9429cd2d106209b09a9-certificate.pem.crt".to_owned(),
        r"C:\Users\smart\OneDrive\ドキュメント\Rust\modbus\AWS-test\aws-iot-device-sdk-rust\cert\26c29384a2f03b7ff8818d47b68b661f5699617fb21fa9429cd2d106209b09a9-private.pem.key".to_owned(),
        "a1rz96s27v9dgn-ats.iot.ap-northeast-1.amazonaws.com".to_owned(),
        None,
    );

    let (iot_core_client, eventloop_stuff) = AWSIoTAsyncClient::new(aws_settings).await?;

    // iot_core_client.subscribe("test".to_string(), QoS::AtMostOnce).await.unwrap();
    iot_core_client.publish("topic".to_string(), QoS::AtMostOnce, "hey").await.unwrap();

    // let mut receiver1 = iot_core_client.get_receiver().await;
    // let mut receiver2 = iot_core_client.get_receiver().await;

    // let recv1_thread = tokio::spawn(async move {
    //     loop {
    //         match receiver1.recv().await {
    //             Ok(event) => {
    //                 match event {
    //                     Packet::Publish(p) => println!("Received message {:?} on topic: {}", p.payload, p.topic),
    //                     _ => println!("Got event on receiver1: {:?}", event),
    //                 }

    //             },
    //             Err(_) => (),
    //         }
    //     }
    // });

    // let recv2_thread = tokio::spawn(async move {
    //     loop {
    //         match receiver2.recv().await {
    //             Ok(event) => println!("Got event on receiver2: {:?}", event),
    //             Err(_) => (),
    //         }
    //     }
    // });
    let pub = tokio::spawn(async move {
        loop {
            iot_core_client.publish("topic".to_string(), QoS::AtMostOnce, "testing aws using raso\pberryv pi").await.unwrap();
            
        }
    });
    

    let listen_thread = tokio::spawn(async move {
            async_event_loop_listener(eventloop_stuff).await.unwrap();
            //iot_core_client.listen().await.unwrap();
    });

    //iot_core_client.publish("topic".to_string(), QoS::AtMostOnce, "hey").await.unwrap();
    tokio::join!(
        pub
    );
        // recv1_thread,
        // recv2_thread,
        // listen_thread);
        
    Ok(())

}
