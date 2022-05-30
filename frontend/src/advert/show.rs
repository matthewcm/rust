use yew::{prelude::*, services::{ConsoleService}};

pub struct Advert {
    url: Option<String>
}

impl Advert{
    fn render_advert (&self) -> Html {
      match &self.url {
        Some(url) => 
          html! {
              <img class=classes!("w-96") src={url.to_string()}/>
          },
        None => 
          html! {
              <p> {"No Image"}</p>
          }
      }
    }
}

pub enum Msg {
    MakeReq,
}

impl Component for Advert {
    type Properties = ();
    type Message = Msg;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::MakeReq);
        Self {
            url: None
        }
    }

    fn view(&self) -> Html {
        html! {
            <div>
              { self.render_advert() }
            </div>
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MakeReq => {
              ConsoleService::info( self.url.as_deref().unwrap_or("hi"));
              self.url = Some("https://upload.wikimedia.org/wikipedia/commons/9/9a/Gull_portrait_ca_usa.jpg".to_owned());
              ()
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }
}