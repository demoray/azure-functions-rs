use azure_functions::{
    bindings::{HttpRequest, HttpResponse},
    func, Context,
};

#[func]
#[binding(name = "req", auth_level = "anonymous")]
pub fn greet(context: &Context, req: &HttpRequest) -> HttpResponse {
    log::info!("Context: {:?}, Request: {:?}", context, req);

    format!(
        "Hello from Rust, {}!\n",
        req.query_params().get("name").map_or("stranger", |x| x)
    )
    .into()
}
