#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use web_sys::window;
use crate::styles::checkbox_style::STYLE;

const _ICON_FILTER: &str = manganis::mg!(file("src/assets/filter-icon.svg"));
const _ICON_ARROW: &str = manganis::mg!(file("src/assets/chevron-down.svg"));

#[component]
pub fn CheckBox() -> Element {
    let mut key_word_list = use_signal(Vec::<String>::new);
    let mut is_dropdown = use_signal(|| false);
    let screen_size = use_signal(|| (0u32, 0u32));

    // ดึงขนาดหน้าจอเมื่อ component mount
    use_effect({
        let mut screen_size = screen_size.clone();
        move || {
            let window = window().unwrap();
            let width = window.inner_width().unwrap().as_f64().unwrap() as u32;
            let height = window.inner_height().unwrap().as_f64().unwrap() as u32;
            screen_size.set((width, height));
            //info!("Screen size: {}x{}", width, height);
            ()
        }
    });

    info!("Current dropdown status: {is_dropdown}");

    // ตรวจสอบขนาดหน้าจอและตั้งค่า dropdown
    use_effect({
        let mut is_dropdown = is_dropdown.clone();
        let screen_size = screen_size.clone();
        move || {
            let (width, _) = *screen_size.read();
            info!("Current Width size: {width}");
            if width > 430 {
                is_dropdown.set(true);
            } else {
                is_dropdown.set(false);
            }
            ()
        }
    });

    info!("Current dropdown state: {is_dropdown}");

    let (width, height) = screen_size();
    info!("Current key word list: {:?}", key_word_list.read());
    info!("Current Screen size: {width} x {height}");

    // ฟังก์ชันสำหรับจัดการการคลิก checkbox
    let mut handle_checkbox_click = move |label: &str| {
        let mut list = key_word_list.write();
        if list.contains(&label.to_string()) {
            list.retain(|x| x != label);
        } else {
            list.push(label.to_string());
        }
        info!("{label} checkbox clicked");
    };

    rsx! {
        style { {STYLE} }

        div { class: "checkbox-container col-xs-12 col-sm-4 col-lg-1",
            div { class: "checkbox-sidebar col-xs-11",
                //h3 { style: "color: white; text-align: center", "Current dropdown status: {is_dropdown}" }
                //h3 { style: "color: white", "Current Screen Size: Width:{width} x Height:{height}" }
                div {
                    button {
                        class: "icon-container",
                        onclick: move |_| {
                            let mut dropdown = is_dropdown.write();
                            *dropdown = !*dropdown;
                        },
                        img { src: "{_ICON_FILTER}" }
                        img { class: "col-lg-hidden col-ms-hidden", src: "{_ICON_ARROW}" }
                    }
                }

                if *is_dropdown.read() {
                    div { class: "checkbox-pt",
                        h3 { class: "header", "General" }
                        ul { class: "detail ",
                            // Checkbox สำหรับตัวเลือกต่างๆ
                            li {
                                input {
                                    class: "filter-checkbox",
                                    r#type: "checkbox",
                                    id: "chill",
                                    onclick: move |_| handle_checkbox_click("Chill")
                                }
                                label { class: "filter-label", "Chill" }
                            }
                            li {
                                input {
                                    class: "filter-checkbox",
                                    r#type: "checkbox",
                                    id: "dramatic",
                                    onclick: move |_| handle_checkbox_click("Dramatic")
                                }
                                label { class: "filter-label", "Dramatic" }
                            }
                            li {
                                input {
                                    class: "filter-checkbox",
                                    r#type: "checkbox",
                                    id: "happy",
                                    onclick: move |_| handle_checkbox_click("Happy")
                                }
                                label { class: "filter-label", "Happy" }
                            }
                            li {
                                input {
                                    class: "filter-checkbox",
                                    r#type: "checkbox",
                                    id: "sad",
                                    onclick: move |_| handle_checkbox_click("Sad")
                                }
                                label { class: "filter-label", "Sad" }
                            }
                            li {
                                input {
                                    class: "filter-checkbox",
                                    r#type: "checkbox",
                                    id: "hopeful",
                                    onclick: move |_| handle_checkbox_click("Hopeful")
                                }
                                label { class: "filter-label", "Hopeful" }
                            }
                            li {
                                input {
                                    class: "filter-checkbox",
                                    r#type: "checkbox",
                                    id: "fantasy",
                                    onclick: move |_| handle_checkbox_click("Fantasy")
                                }
                                label { class: "filter-label", "Fantasy" }
                            }
                            li {
                                input {
                                    class: "filter-checkbox",
                                    r#type: "checkbox",
                                    id: "romantic",
                                    onclick: move |_| handle_checkbox_click("Romantic")
                                }
                                label { class: "filter-label", "Romantic" }
                            }
                            li {
                                input {
                                    class: "filter-checkbox",
                                    r#type: "checkbox",
                                    id: "relaxing",
                                    onclick: move |_| handle_checkbox_click("Relaxing")
                                }
                                label { class: "filter-label", "Relaxing" }
                            }
                        }
                    }
                }
            }
        }
    }
}
