use warp::Filter;
use futures_util::{FutureExt, StreamExt};

#[tokio::main]
async fn main() {

    let routes = warp::path("ws").and(warp::ws()).map( |ws: warp::ws::Ws | {

        ws.on_upgrade( |websocket| {
            
            let ( _tx, rx ) = websocket.split();

            rx.for_each( |message| async {

                match message {
                    Ok(msg) => {
                        let content = msg.to_str().unwrap_or("");

                        println!("Received message: {:?}", content);
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