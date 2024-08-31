use yew::prelude::*;
use std::time::Duration;
use gloo_timers::callback::Interval;

struct CountdownTimer {
    target: chrono::DateTime<chrono::Utc>,
    remaining: Duration,
    _interval: Interval,
}

#[derive(Properties, PartialEq)]
struct CountdownProps {
    target: chrono::DateTime<chrono::Utc>,
}

enum Msg {
    UpdateRemaining,
}

impl Component for CountdownTimer {
    type Message = Msg;
    type Properties = CountdownProps;

    fn create(ctx: &Context<Self>) -> Self {
        let target = ctx.props().target;
        let remaining = target.signed_duration_since(chrono::Utc::now()).to_std().unwrap_or(Duration::from_secs(0));

        let link = ctx.link().clone();
        let interval = Interval::new(1000, move || link.send_message(Msg::UpdateRemaining));

        Self {
            target,
            remaining,
            _interval: interval,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateRemaining => {
                let now = chrono::Utc::now();
                if now < self.target {
                    self.remaining = self.target.signed_duration_since(now).to_std().unwrap();
                } else {
                    self.remaining = Duration::from_secs(0);
                }
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let days = self.remaining.as_secs() / 86400;
        let hours = (self.remaining.as_secs() % 86400) / 3600;
        let minutes = (self.remaining.as_secs() % 3600) / 60;
        let seconds = self.remaining.as_secs() % 60;

        html! {
            <div>
                <h2>{"Countdown Timer"}</h2>
                <p>
                    {format!("{:02}:{:02}:{:02}:{:02}", days, hours, minutes, seconds)}
                </p>
            </div>
        }
    }
}

// Main app component
#[function_component(App)]
fn app() -> Html {
    let target = chrono::Utc::now() + chrono::Duration::days(1);

    html! {
        <div>
            <h1>{"Yew Countdown Website"}</h1>
            <CountdownTimer target={target} />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}