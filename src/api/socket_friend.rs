use actix_web::{web, Error, HttpResponse,HttpRequest};
use std::time::{Instant,Duration};
use actix_web_actors::ws;
use actix::prelude::*;
use crate::common::connection;
use crate::model::user::{unseen_friend_requests,Users,mark_seen_request};

// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub fn ws(r: HttpRequest, stream: web::Payload,pool: web::Data<connection::PgPool>) -> Result<HttpResponse, Error> {
    println!("{:?}", r);
    let res = ws::start(MyWebSocket::new(&pool), &r, stream)?;
    Ok(res)
}

pub struct MyWebSocket {
    pub hb: Instant,
    pub pool :connection::PgPool,
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}

/// Handler for `ws::Message`
impl StreamHandler<ws::Message, ws::ProtocolError> for MyWebSocket {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        // process websocket messages
        println!("WS: {:?}", msg);
        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                let users = unseen_friend_requests(&self.pool,1);
                match users {
                    Ok(users) => {
                        let data = Users{users:users};
                        let json_data = serde_json::to_string(&data);
                        if json_data.is_ok(){
                            let mark = mark_seen_request(&self.pool,data.users,1);
                            match mark {
                                Ok(_) => ctx.text(json_data.unwrap()),
                                Err(_) => {}
                            }
                        }
                    }
                    Err(_) => {}
                }
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => ctx.text(text),
            ws::Message::Binary(bin) => ctx.binary(bin),
            ws::Message::Close(_) => {
                ctx.stop();
            }
            ws::Message::Nop => (),
        }
    }
}

impl MyWebSocket {
    pub fn new(pool:&connection::PgPool) -> Self {
        Self { hb: Instant::now(),pool:pool.to_owned() }
    }

    /// helper method that sends ping to client every second.
    ///
    /// also this method checks heartbeats from client
    pub fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping("");
        });
    }
}
