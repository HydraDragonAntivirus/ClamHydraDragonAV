use yew::prelude::*;
use web_sys::HtmlInputElement;

pub struct App {
    link: Callback<web_sys::Event>,
    input_ref: NodeRef,
}

pub enum Msg {
    FolderSelected,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            link: ctx.link().callback(|_| Msg::FolderSelected),
            input_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FolderSelected => {
                if let Some(input) = self.input_ref.cast::<HtmlInputElement>() {
                    if let Some(files) = input.files() {
                        for i in 0..files.length() {
                            let file = files.item(i).unwrap();
                            log::info!("File selected: {:?}", file.name());
                        }
                    }
                }
            }
        }
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <input type="file"
                   multiple=true
                   webkitdirectory={true.to_string()}
                   ref={self.input_ref.clone()}
                   onchange={self.link.clone()} />
        }
    }
}
