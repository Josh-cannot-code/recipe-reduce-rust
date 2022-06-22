use yew::{html, Component, Context, Html};

// Define the possible messages which can be sent to the component
pub enum Msg {
    ChangeToBlue,
    ChangeToRed,
}

pub struct Model {
    value: String, // This will store the current color
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: "color:blue".to_string() }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ChangeToBlue => {
                self.value = "color:blue".to_string();
                true // Return true to cause the displayed change to update
            }
            Msg::ChangeToRed => {
                self.value = "color:red".to_string();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        
        html! {
            <div>
                <div class="panel">
                    // A button to send the change to blue message
                    <button onclick={ctx.link().callback(|_| Msg::ChangeToBlue)}>
                        { "Change to Blue" }
                    </button>

                    // A button to send the change to red message
                    <button onclick={ctx.link().callback(|_| Msg::ChangeToRed)}>
                        { "Change to Red" }
                    </button> 
                </div>

                // Display text with current color
                <p style = {self.value.clone()}>
                    { "Some Text" }
                </p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
