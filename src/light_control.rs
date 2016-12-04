use std::process::Command;
use std::sync::Arc;

use iron::middleware;
use iron::prelude::*;
use iron::status;
use router::Router;

struct Handler {
    tdtool:  Arc<String>,
    command: Arc<String>
}

impl Handler {
    fn new(tdtool: Arc<String>, command: Arc<String>) -> Handler {
        Handler {
            tdtool:  tdtool,
            command: command
        }
    }

    fn exec(&self, id: &str) {
        let output = Command::new(self.tdtool.as_ref())
            .arg(self.command.as_ref())
            .arg(id)
            .output();

        match output {
            Err(e) => println!("Error running tdtool: {}", e),
            _      => {}
        }
    }

}

impl middleware::Handler for Handler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let ref id = iexpect!(req.extensions.get::<Router>().unwrap().find("id"));
        self.exec(&id);
        Ok(Response::with((status::Ok, "")))
    }
}

pub fn bind(router: &mut Router, tdtool: &str) {
    let tdt = Arc::new(String::from(tdtool));

    let on_handler  = Handler::new(tdt.clone(), Arc::new(String::from("--on")));
    let off_handler = Handler::new(tdt.clone(), Arc::new(String::from("--off")));

    router.put("/lights/:id/on",  on_handler,  "lights-on");
    router.put("/lights/:id/off", off_handler, "lights-off");
}
