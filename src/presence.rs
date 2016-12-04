use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::sync::Arc;

use iron::middleware;
use iron::prelude::*;
use iron::status;
use regex::Regex;
use router::Router;

use utils;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^[A-Za-z0-9_-]+$").unwrap();
}

struct Handler {
    workdir: Arc<String>
}

impl Handler {
    fn new(workdir: Arc<String>) -> Handler {
        Handler {
            workdir: workdir
        }
    }

    fn present(&self, name: &str) -> io::Result<()> {
        let file = utils::presence_file(self.workdir.as_ref(), name);
        match OpenOptions::new().create(true).write(true).open(file.as_path()) {
            Ok(_)  => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn absent(&self, name: &str) -> io::Result<()> {
        let file = utils::presence_file(self.workdir.as_ref(), name);
        let path = file.as_path();

        if !path.exists() {
            return Ok(());
        }

        fs::remove_file(path)
    }
}

impl middleware::Handler for Handler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let router = req.extensions.get::<Router>().unwrap();

        let ref name  = iexpect!(router.find("name"));
        let ref state = iexpect!(router.find("state"));

        if !RE.is_match(name) {
            return Ok(Response::with((status::BadRequest, "")))
        }

        match state.as_ref() {
            "present" => itry!(self.present(name), status::InternalServerError),
            "absent"  => itry!(self.absent(name),  status::InternalServerError),
            _         => return Ok(Response::with((status::NotFound, ""))),
        };


        Ok(Response::with((status::Ok, "")))
    }
}

pub fn bind(router: &mut Router, workdir: &str) {
    let w = Arc::new(String::from(workdir));
    let present_handler = Handler::new(w.clone());

    router.put("/presence/:name/:state", present_handler, "presence-present");
}
