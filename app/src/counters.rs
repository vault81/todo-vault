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
            <p> "This counter stores its data in a variable on the server."</p>
            <p>"The value is shared across connections. Try opening this is another browser tab to see what I mean."</p>
            <div>
                <h2>"Form Counter"</h2>
                <FormCounter/>
            </div>
        </div>
    }
}

#[component]
pub fn FormCounter(cx: Scope) -> impl IntoView {
    let adjust = create_server_action::<AdjustServerCount>(cx);
    let clear = create_server_action::<ClearServerCount>(cx);

    let counter = create_resource(
        cx,
        move || (adjust.version().get(), clear.version().get()),
        |_| async { get_server_count().await },
    );

    view! {
        cx,
        <div>
            <h3>"Form Counter"</h3>
            <p>
                "This counter uses forms to set the value on the server. " <br/>
                "When progressively enhanced, it should behave like any JS app coutner" <br/>
                "When the site is loaded without client-side JS/wasm, it works just as well just slower by using form submits." <br/>
                "This allows us to write websites that \"just work\" on any device, without much coding effort."
            </p>
            <div class="inline-grid grid-cols-5 justify-items-center">
                <ActionForm action=clear>
                    <Button b_type="submit">
                        "Clear"
                    </Button>
                </ActionForm>
                <ActionForm action=adjust>
                    <input type="hidden" name="delta" value="-1"/>
                    <input type="hidden" name="msg" value="form value down"/>
                    <Button b_type="submit">
                        "-1"
                    </Button>
                </ActionForm>
                <Transition fallback=move || view!{cx, <span class="py-2.5 px-5 mr-2 mb-2 w-32"> "Value: "</span>}>
                    <span class="py-2.5 px-5 mr-2 mb-2 w-32">
                        "Value: " {
                            move || counter.read(cx).and_then(std::result::Result::ok).unwrap_or(0).to_string()
                        } "!"
                    </span>
                </Transition>
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
