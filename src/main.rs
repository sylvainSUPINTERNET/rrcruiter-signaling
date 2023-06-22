use warp::Filter;
use futures_util::{FutureExt, StreamExt};

mod utils;
mod services;



#[tokio::main]
async fn main() {

    let routes = warp::path("ws").and(warp::ws()).map( |ws: warp::ws::Ws | {

        ws.on_upgrade( |websocket| {
            
            let ( _tx, rx ) = websocket.split();

            rx.for_each( |message| async {

                match message {
                    Ok(msg) => {
                        let content = msg.to_str().unwrap_or("");
                        let event: services::event::Event  = serde_json::from_str(content).unwrap();

                        println!("Received message: {:?}", event.message);
                        println!("Received message: {:?}", event.topic);
                        println!("Received message: {:?}", event.user_uuid);

        
                    }
                    Err(e) => {
                        eprintln!("websocket error: {:?}", e);
                        return;
                    }
                }
            })
            // TODO user envoi un event join group X 

            // answer / offer 
            
            

            // rx.forward(tx).map( |result| {
            //     if let Err(e) = result {
            //         eprintln!("websocket error: {:?}", e);
            //     }
            // })
        })
    });

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}