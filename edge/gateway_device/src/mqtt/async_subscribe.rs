// paho-mqtt/examples/async_subscribe.rs
//
// This is a Paho MQTT Rust client, sample application.
//
//! This application is an MQTT subscriber using the asynchronous client
//! interface of the Paho Rust client library.
//! It also monitors for disconnects and performs manual re-connections.
//!
//! The sample demonstrates:
//!   - An async/await subscriber
//!   - Connecting to an MQTT server/broker.
//!   - Subscribing to topics
//!   - Receiving messages from an async stream.
//!   - Handling disconnects and attempting manual reconnects.
//!   - Using a "persistent" (non-clean) session so the broker keeps
//!     subscriptions and messages through reconnects.
//!   - Last will and testament
//!
//! Note that this example specifically does *not* handle a ^C, so breaking
//! out of the app will always result in an un-clean disconnect causing the
//! broker to emit the LWT message.

/*******************************************************************************
 * Copyright (c) 2017-2025 Frank Pagliughi <fpagliughi@mindspring.com>
 *
 * All rights reserved. This program and the accompanying materials
 * are made available under the terms of the Eclipse Public License v2.0
 * and Eclipse Distribution License v1.0 which accompany this distribution.
 *
 * The Eclipse Public License is available at
 *    http://www.eclipse.org/legal/epl-v20.html
 * and the Eclipse Distribution License is available at
 *   http://www.eclipse.org/org/documents/edl-v10.php.
 *
 * Contributors:
 *    Frank Pagliughi - initial implementation and documentation
 *******************************************************************************/

use crate::tpm::ecdsa::ecdsa;
use futures::executor::block_on;
use paho_mqtt as mqtt;
use reqwest::blocking::Client;
use std::{env, process, time::Duration};
use tss_esapi::handles::KeyHandle;
use tss_esapi::Context;

// The topics to which we subscribe.
const TOPICS: &[&str] = &["lwt", "temperature"];
const QOS: &[i32] = &[1, 1];
const NUM_OF_READINGS: i32 = 5;

/////////////////////////////////////////////////////////////////////////////

pub fn subscribe_example(client: &Client, context: &mut Context, ecdsa_handle: KeyHandle) {
    // Initialize the logger from the environment
    env_logger::init();

    let host = env::args()
        .nth(1)
        .unwrap_or_else(|| "mqtt://192.168.0.88:1883".to_string());

    println!("Connecting to the MQTT server at '{}'...", host);

    // Create the client. Use a Client ID for a persistent session.
    // A real system should try harder to use a unique ID.
    let create_opts = mqtt::CreateOptionsBuilder::new_v3()
        .server_uri(host)
        .client_id("rust_async_subscribe")
        .finalize();

    // Create the client connection
    let mut cli = mqtt::AsyncClient::new(create_opts).unwrap_or_else(|e| {
        println!("Error creating the client: {:?}", e);
        process::exit(1);
    });

    if let Err(err) = block_on(async {
        // Get message stream before connecting.
        let strm = cli.get_stream(25);

        // Define the set of options for the connection
        let lwt = mqtt::Message::new(
            "lwt",
            "[LWT] Async subscriber lost connection",
            mqtt::QOS_1,
        );

        // Create the connect options, explicitly requesting MQTT v3.x
        let conn_opts = mqtt::ConnectOptionsBuilder::new_v3()
            .keep_alive_interval(Duration::from_secs(30))
            .clean_session(false)
            .will_message(lwt)
            .finalize();

        // Make the connection to the broker
        cli.connect(conn_opts).await?;

        println!("Subscribing to topics: {:?}", TOPICS);
        cli.subscribe_many(TOPICS, QOS).await?;

        // Just loop on incoming messages.
        println!("Waiting for messages...");

        let mut rconn_attempt: usize = 0;

        // Note that we're not providing a way to cleanly shut down and
        // disconnect. Therefore, when you kill this app (with a ^C or
        // whatever) the server will get an unexpected drop and then
        // should emit the LWT message.

        let mut payload: Vec<f64> = vec![];
        let mut count = 0;
        let mut average: f64 = 0.0;

        while let Ok(msg_opt) = strm.recv().await {
            if let Some(msg) = msg_opt {
                if msg.topic() == TOPICS[0] {
                    println!("LWT Received: {}",msg);
                }else{
                    let payload_as_str = msg.payload_str();
                    let payload_as_int: f64 = payload_as_str.parse().expect("Failed to parse String into f64");
                    payload.push(payload_as_int);
                    count += 1;

                    if count == NUM_OF_READINGS{
                        println!("Received: {:?}",payload);
                        //aggregate data (IoT example)
                        for temperature in &mut payload{
                            average += *temperature;
                        }
                        average = average / 5.0;
                        //convert aggregate to byte array Vec<u8>
                        let average_as_bytes = average.to_be_bytes();
                        let mut payload_to_send: Vec<u8> = vec![];
                        payload_to_send.extend(average_as_bytes);
                        println!("Payload to send: {:?}",payload_to_send);
                        //ecdsa to sign and send to server
                        ecdsa(client,context,ecdsa_handle,payload_to_send);

                        payload.truncate(0);
                        count = 0;
                        average = 0.0;
                    }
                }
            }
            else {
                // A "None" means we were disconnected. Try to reconnect...
                println!("Lost connection. Attempting reconnect...");
                while let Err(err) = cli.reconnect().await {
                    rconn_attempt += 1;
                    println!("Error reconnecting #{}: {}", rconn_attempt, err);
                    // For tokio use: tokio::time::delay_for()
                    async_std::task::sleep(Duration::from_secs(1)).await;
                }
                println!("Reconnected.");
            }
        }

        // Explicit return type for the async block
        Ok::<(), mqtt::Error>(())
    }) {
        eprintln!("{}", err);
    }
}