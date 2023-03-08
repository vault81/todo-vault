use leptos::*;
use leptos_router::*;

use crate::functions::*;

// buttons styled with tailwind css
#[component]
fn Button(
    cx: Scope,
    children: Children,
    #[prop(optional, into)] b_type: String,
) -> impl IntoView {
    view! {
        cx,
        <button
            type={b_type}
            class="py-2.5 px-5 mr-2 mb-2 text-sm font-medium text-gray-900 bg-white rounded-lg border border-gray-200 dark:text-gray-400 dark:bg-gray-800 dark:border-gray-600 hover:text-blue-700 hover:bg-gray-100 focus:z-10 focus:ring-4 focus:ring-gray-200 focus:outline-none dark:focus:ring-gray-700 dark:hover:text-white dark:hover:bg-gray-700"
        >
            {children(cx)}
        </button>
    }
}

#[component]
pub fn Counters(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div class="format dark:format-invert">
            <h1>"Server-Side Counters"</h1>
            <p> "Each of these counters stores its data in the same variable on the server."</p>
            <p>"The value is shared across connections. Try opening this is another browser tab to see what I mean."</p>
            <div>
                <h2>"Counter 1"</h2>
                <Counter/>
                <h2>"Counter 2"</h2>
                <FormCounter/>
            </div>
        </div>
    }
}

#[component]
pub fn Counter(cx: Scope) -> impl IntoView {
    let dec = create_action(cx, |_| adjust_server_count(-1, "decing".into()));
    let inc = create_action(cx, |_| adjust_server_count(1, "incing".into()));
    let clear = create_action(cx, |_| clear_server_count());

    let counter = create_resource(
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

    let value = move || {
        counter
            .read(cx)
            .and_then(std::result::Result::ok)
            .unwrap_or(0)
    };

    let error_msg = move || {
        counter.read(cx).and_then(|res| match res {
            Ok(_) => None,
            Err(e) => Some(e),
        })
    };

    view! {
        cx,
        <div>
            <h3>"Simple Counter"</h3>
            <p>"This counter sets the value on the server and automatically reloads the new value."</p>
            <div class="inline-grid grid-cols-5 justify-items-center">
                <Button on:click=move |_| clear.dispatch(())>"Clear"</Button>
                <Button on:click=move |_| dec.dispatch(())>"-1"</Button>
                <span class="py-2.5 px-5 mr-2 mb-2 w-32">
                    "Value: " {move || value().to_string()} "!"
                </span>
                <Button on:click=move |_| inc.dispatch(())>"+1"</Button>
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

    let counter = create_resource(
        cx,
        move || (adjust.version().get(), clear.version().get()),
        |_| async { get_server_count().await },
    );

    let value = move || {
        counter
            .read(cx)
            .and_then(std::result::Result::ok)
            .unwrap_or(0)
    };

    view! {
        cx,
        <div>
            <h3>"Form Counter"</h3>
            <p>"This counter uses forms to set the value on the server. When progressively enhanced, it should behave identically to the “Simple Counter.”"</p>
            <div class="inline-grid grid-cols-5 justify-items-center">
                // calling a server function is the same as POSTing to its API URL
                // so we can just do that with a form and button
                <ActionForm action=clear>
                    <Button b_type="submit">
                        "Clear"
                    </Button>
                </ActionForm>
                // We can submit named arguments to the server functions
                // by including them as input values with the same name
                <ActionForm action=adjust>
                    <input type="hidden" name="delta" value="-1"/>
                    <input type="hidden" name="msg" value="form value down"/>
                    <Button b_type="submit">
                        "-1"
                    </Button>
                </ActionForm>
                <span class="py-2.5 px-5 mr-2 mb-2 w-32">
                    "Value: " {move || value().to_string()} "!"
                </span>
                <ActionForm action=adjust>
                    <input type="hidden" name="delta" value="1"/>
                    <input type="hidden" name="msg" value="form value up"/>
                    <Button b_type="submit">
                        "+1"
                    </Button>
                </ActionForm>
            </div>
        </div>
    }
}
