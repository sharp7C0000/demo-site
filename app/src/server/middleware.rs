use iron::prelude::*;
use iron::status;
use iron::{AfterMiddleware};
use router::{Router, NoRoute};
use handlebars_iron::{Template, HandlebarsEngine, DirectorySource, MemorySource};

use std::collections::BTreeMap;

// custom 404 not found middleware
pub struct Custom404;

impl AfterMiddleware for Custom404 {
  fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
    if let Some(_) = err.error.downcast::<NoRoute>() {
      let mut resp = Response::new();
      let mut data = BTreeMap::new();
      data.insert("message".to_string(), "404 Page not found".to_string());
      resp.set_mut(Template::new("error", data)).set_mut(status::NotFound);
      Ok(resp)
    } else {
      Err(err)
    }
  }
}