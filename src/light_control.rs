use std::process::Command;
use std::sync::Arc;

use iron::middleware;
use iron::prelude::*;
use iron::status;
use router::Router;

use utils;

struct Handler {
    tdtool:  Arc<String>,
    command: Arc<String>,
    workdir: Arc<String>,
}

impl Handler {
    fn new(tdtool: Arc<String>, command: Arc<String>, workdir: Arc<String>) -> Handler {
        Handler {
            tdtool:  tdtool,
            command: command,
            workdir: workdir
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
        let router = req.extensions.get::<Router>().unwrap();
        let ref id = iexpect!(router.find("id"));

        match router.find("device") {
            Some(device) => {
                let file = utils::presence_file(self.workdir.as_ref(), device);
                let path = file.as_path();

                if !path.exists() {
                    self.exec(&id)
                }
            },
            None => self.exec(&id),
        }

        Ok(Response::with((status::Ok, "")))
    }
}

pub fn bind(router: &mut Router, tdtool: &str, workdir: &str) {
    let t   = Arc::new(String::from(tdtool));
    let w   = Arc::new(String::from(workdir));
    let on  = Arc::new(String::from("--on"));
    let off = Arc::new(String::from("--off"));

    {
        let on_handler  = Handler::new(t.clone(), on.clone(),  w.clone());
        let off_handler = Handler::new(t.clone(), off.clone(), w.clone());

        router.put("/lights/:id/on",  on_handler,  "lights-on");
        router.put("/lights/:id/off", off_handler, "lights-off");
    }

    {
        let on_handler  = Handler::new(t.clone(), on.clone(),  w.clone());
        let off_handler = Handler::new(t.clone(), off.clone(), w.clone());

        router.put("/lights/:id/on-unless/:device",  on_handler,  "lights-on-unless");
        router.put("/lights/:id/off-unless/:device", off_handler, "lights-off-unless");
    }
}
