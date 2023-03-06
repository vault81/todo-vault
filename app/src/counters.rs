use leptos::*;

use leptos_router::*;

use crate::functions::*;

#[component]
pub fn Counters(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <header>
            <h1>"Server-Side Counters"</h1>
            <p>"Each of these counters stores its data in the same variable on the server."</p>
            <p>"The value is shared across connections. Try opening this is another browser tab to see what I mean."</p>
        </header>
        <div>
            <h2>"Counter 1"</h2>
            <Counter/>
            <h2>"Counter 2"</h2>
            <FormCounter/>
        </div>
    }
}

#[component]
pub fn Counter(cx: Scope) -> impl IntoView {
    let dec = create_action(cx, |_| adjust_server_count(-1, "decing".into()));
    let inc = create_action(cx, |_| adjust_server_count(1, "incing".into()));
    let clear = create_action(cx, |_| clear_server_count());

    tracing::info!("Counter created");
    let counter = create_local_resource(
        cx,
        move || {
            (
                dec.version().get(),
                inc.version().get(),
                clear.version().get(),
            )
        },
        |_| async move { get_server_count().await },
    );

    tracing::info!("42 created");
    let value = move || {
        counter
            .read(cx)
            .map_or(0, |count: Result<i32, ServerFnError>| count.unwrap_or(0))
    };

    let error_msg = move || {
        counter
            .read(cx)
            .and_then(|res| match res {
                Ok(_) => None,
                Err(e) => Some(e),
            })
    };

    view! {
        cx,
        <div>
            <h2>"Simple Counter"</h2>
            <p>"This counter sets the value on the server and automatically reloads the new value."</p>
            <div>
                <button on:click=move |_| clear.dispatch(())>"Clear"</button>
                <button on:click=move |_| dec.dispatch(())>"-1"</button>
                <span>"Value: " {value} "!"</span>
                <button on:click=move |_| inc.dispatch(())>"+1"</button>
            </div>
            {move || error_msg().map(|msg| view! { cx, <p>"Error: " {msg.to_string()}</p>})}
        </div>
    }
}

// This is the <Form/> counter
// It uses the same invalidation pattern as the plain counter,
// but uses HTML forms to submit the actions
#[component]
pub fn FormCounter(cx: Scope) -> impl IntoView {
    let adjust = create_server_action::<AdjustServerCount>(cx);
    let clear = create_server_action::<ClearServerCount>(cx);

    let counter = create_local_resource(
        cx,
        move || (adjust.version().get(), clear.version().get()),
        |_| async {
            tracing::debug!("FormCounter running fetcher");
            get_server_count().await
        },
    );
    let value = move || {
        tracing::debug!("FormCounter looking for value");
        counter
            .read(cx)
            .and_then(std::result::Result::ok)
            .unwrap_or(0)
    };

    view! {
        cx,
        <div>
            <h2>"Form Counter"</h2>
            <p>"This counter uses forms to set the value on the server. When progressively enhanced, it should behave identically to the “Simple Counter.”"</p>
            <div>
                // calling a server function is the same as POSTing to its API URL
                // so we can just do that with a form and button
                <ActionForm action=clear>
                    <input type="submit" value="Clear"/>
                </ActionForm>
                // We can submit named arguments to the server functions
                // by including them as input values with the same name
                <ActionForm action=adjust>
                    <input type="hidden" name="delta" value="-1"/>
                    <input type="hidden" name="msg" value="form value down"/>
                    <input type="submit" value="-1"/>
                </ActionForm>
                <span>"Value: " {move || value().to_string()} "!"</span>
                <ActionForm action=adjust>
                    <input type="hidden" name="delta" value="1"/>
                    <input type="hidden" name="msg" value="form value up"/>
                    <input type="submit" value="+1"/>
                </ActionForm>
            </div>
        </div>
    }
}
