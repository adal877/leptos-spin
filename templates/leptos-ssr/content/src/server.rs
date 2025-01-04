use crate::library::save_count;
use crate::library::home_page::SaveCount;

// Adicione este uso para pegar o tipo gerado pela macro `#[server]`
use leptos::ServerFn;
use leptos::prelude::ServerFnError;

struct LeptosServer;

impl Guest for LeptosServer {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        let executor = WasiExecutor::new(leptos_wasi::executor::Mode::Stalled);
        if let Err(e) = LeptosExecutor::init_local_custom_executor(executor.clone()) {
            eprintln!("Got error while initializing leptos_wasi executor: {e:?}");
            return;
        }
        executor.run_until(async {
            if let Err(e) = handle_request(request, response_out).await {
                eprintln!("Got error while handling request: {e:?}");
            }
        })
    }
}

async fn handle_request(
    request: IncomingRequest,
    response_out: ResponseOutparam,
) -> Result<(), HandlerError> {
    use leptos_wasi::prelude::Handler;

    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;

    Handler::build(request, response_out)?
        .with_server_fn::<save_count>() // O tipo é gerado pela macro #[server]
        .generate_routes(crate::app::App)
        .handle_with_context(move || crate::app::shell(leptos_options.clone()), || {})
        .await?;
    Ok(())
}

export!(LeptosServer with_types_in wasi);
