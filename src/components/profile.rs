use css_in_rust::Style;
use yew::prelude::*;
use yew::{html, Classes, Component, ComponentLink, Html, Properties, ShouldRender};

pub struct Profile {
    wrapper_style: Style,
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: String,
}

pub enum Msg {}

impl Component for Profile {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let wrapper_style = Style::create(
            String::from("wrapper"),
            String::from(
                r#"
                width: 940px;
                "#,
            ),
        )
        .expect("An error occured while creating the style.");

        Self {
            props,
            wrapper_style,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class=Classes::from("block sm:flex sm:items-center
                                      text-center sm:text-left
                                      max-w-full m-auto
                                      px-5 sm:px-3
                                      py-16 md:py-20 lg:py-24"
                                    ).extend(self.wrapper_style.clone())>
                <div class="w-40 md:w-56
                            h-40 md:h-56
                            p-4 md:p-6 border rounded-full
                            m-auto mb-5 sm:m-0">
                    <img
                      src="/assets/avatar.png"
                      alt="Avatar"
                      class="w-full h-full pb-full rounded-full object-cover"
                    />
                </div>

                <div class="sm:pl-8 sm:flex-1">
                    <h1 class="mb-3 text-2xl sm:text-2xl md:text-3xl">{"Hi! I'm "}<b>{"Soleil ????"}</b></h1>
                    <p class="mb-2">{"????"}<span class="italic">{"Bug is like a wind"}</span>{"????"}<span class="italic">{", always by my side."}</span></p>
                    <p>{"Ch??o m???ng b???n ???? ?????n v???i c??n c??? nh??? c???a t??i. T??i l?? Vi???t - Nguy???n V??n Vi???t. B???n c?? th??? g???i t??i l?? Soleil (ho???c Lielos c??ng ???????c nh?? ????)"}</p>
                    <p class="mb-4">{"???? ?????n ????y r???i th?? h??y ng?? qua m???t ch??t xem t??i c?? g?? nh??! ????"}</p>
                    <div class="flex justify-center sm:justify-start">
                        // Facebook
                        <a class="mr-5" href="https://www.facebook.com/lielos13/" target="_blank" rel="noopener noreferrer">
                            <svg stroke="currentColor" fill="currentColor" stroke-width="0" viewBox="0 0 512 512" height="1.5rem" width="1.5rem" xmlns="http://www.w3.org/2000/svg"><path d="M426.8 64H85.2C73.5 64 64 73.5 64 85.2v341.6c0 11.7 9.5 21.2 21.2 21.2H256V296h-45.9v-56H256v-41.4c0-49.6 34.4-76.6 78.7-76.6 21.2 0 44 1.6 49.3 2.3v51.8h-35.3c-24.1 0-28.7 11.4-28.7 28.2V240h57.4l-7.5 56H320v152h106.8c11.7 0 21.2-9.5 21.2-21.2V85.2c0-11.7-9.5-21.2-21.2-21.2z"></path></svg>
                        </a>

                        // Twitter
                        <a class="mr-5" href="https://twitter.com/SoleilVn" target="_blank" rel="noopener noreferrer">
                            <svg stroke="currentColor" fill="currentColor" stroke-width="0" viewBox="0 0 512 512" height="1.5rem" width="1.5rem" xmlns="http://www.w3.org/2000/svg"><path d="M492 109.5c-17.4 7.7-36 12.9-55.6 15.3 20-12 35.4-31 42.6-53.6-18.7 11.1-39.4 19.2-61.5 23.5C399.8 75.8 374.6 64 346.8 64c-53.5 0-96.8 43.4-96.8 96.9 0 7.6.8 15 2.5 22.1-80.5-4-151.9-42.6-199.6-101.3-8.3 14.3-13.1 31-13.1 48.7 0 33.6 17.2 63.3 43.2 80.7-16-.4-31-4.8-44-12.1v1.2c0 47 33.4 86.1 77.7 95-8.1 2.2-16.7 3.4-25.5 3.4-6.2 0-12.3-.6-18.2-1.8 12.3 38.5 48.1 66.5 90.5 67.3-33.1 26-74.9 41.5-120.3 41.5-7.8 0-15.5-.5-23.1-1.4C62.8 432 113.7 448 168.3 448 346.6 448 444 300.3 444 172.2c0-4.2-.1-8.4-.3-12.5C462.6 146 479 129 492 109.5z"></path></svg>
                        </a>

                        // LinkedIn
                        <a class="mr-5" href="https://www.linkedin.com/in/nguy%E1%BB%85n-v%C4%83n-vi%E1%BB%87t-491112165/" target="_blank" rel="noopener noreferrer">
                            <svg stroke="currentColor" fill="currentColor" stroke-width="0" viewBox="0 0 512 512" height="1.5rem" width="1.5rem" xmlns="http://www.w3.org/2000/svg"><path d="M417.2 64H96.8C79.3 64 64 76.6 64 93.9V415c0 17.4 15.3 32.9 32.8 32.9h320.3c17.6 0 30.8-15.6 30.8-32.9V93.9C448 76.6 434.7 64 417.2 64zM183 384h-55V213h55v171zm-25.6-197h-.4c-17.6 0-29-13.1-29-29.5 0-16.7 11.7-29.5 29.7-29.5s29 12.7 29.4 29.5c0 16.4-11.4 29.5-29.7 29.5zM384 384h-55v-93.5c0-22.4-8-37.7-27.9-37.7-15.2 0-24.2 10.3-28.2 20.3-1.5 3.6-1.9 8.5-1.9 13.5V384h-55V213h55v23.8c8-11.4 20.5-27.8 49.6-27.8 36.1 0 63.4 23.8 63.4 75.1V384z"></path></svg>
                        </a>

                        // Github
                        <a href="https://github.com/viet-nv" target="_blank" rel="noopener noreferrer">
                            <svg stroke="currentColor" fill="currentColor" stroke-width="0" viewBox="0 0 512 512" height="1.5rem" width="1.5rem" xmlns="http://www.w3.org/2000/svg"><path d="M256 32C132.3 32 32 134.9 32 261.7c0 101.5 64.2 187.5 153.2 217.9 1.4.3 2.6.4 3.8.4 8.3 0 11.5-6.1 11.5-11.4 0-5.5-.2-19.9-.3-39.1-8.4 1.9-15.9 2.7-22.6 2.7-43.1 0-52.9-33.5-52.9-33.5-10.2-26.5-24.9-33.6-24.9-33.6-19.5-13.7-.1-14.1 1.4-14.1h.1c22.5 2 34.3 23.8 34.3 23.8 11.2 19.6 26.2 25.1 39.6 25.1 10.5 0 20-3.4 25.6-6 2-14.8 7.8-24.9 14.2-30.7-49.7-5.8-102-25.5-102-113.5 0-25.1 8.7-45.6 23-61.6-2.3-5.8-10-29.2 2.2-60.8 0 0 1.6-.5 5-.5 8.1 0 26.4 3.1 56.6 24.1 17.9-5.1 37-7.6 56.1-7.7 19 .1 38.2 2.6 56.1 7.7 30.2-21 48.5-24.1 56.6-24.1 3.4 0 5 .5 5 .5 12.2 31.6 4.5 55 2.2 60.8 14.3 16.1 23 36.6 23 61.6 0 88.2-52.4 107.6-102.3 113.3 8 7.1 15.2 21.1 15.2 42.5 0 30.7-.3 55.5-.3 63 0 5.4 3.1 11.5 11.4 11.5 1.2 0 2.6-.1 4-.4C415.9 449.2 480 363.1 480 261.7 480 134.9 379.7 32 256 32z"></path></svg>
                        </a>
                    </div>
                </div>
            </div>
        }
    }
}
