#[macro_use] extern crate nickel;

mod server;

fn main() {
    let mut server = server::DemoServer::new();

    server.init();
    server.register();
    server.server.listen("127.0.0.1:6767");

    //server.utilize(StaticFilesHandler::new("public/"));

    // router::register(&server);
    //
    // server.get("/", middleware! { |_, response|
    //     let mut data = HashMap::new();
    //     data.insert("name", "user");
    //     return response.render("views/index.tpl", &data);
    // });
    //
    // server.listen("127.0.0.1:6767");


}
