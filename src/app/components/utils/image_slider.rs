use leptos::*;
use leptos::IntoView;
use leptos_icons::Icon;
use icondata as i;
#[component]
pub fn ImageSlider(images: Vec<String>) -> impl IntoView {
    // Signal to keep track of the current slide index.
    let (current_index, set_current_index) = create_signal(0);
    let images2 = images.clone();
    // Function to go to the next slide.
    let next_image = {
        let images = images.clone();
        move |_| {
            let current = current_index.get();
            let new_index = if current >= images.len() - 1 { 0 } else { current + 1 };
            set_current_index.set(new_index);
        }
    };

    // Function to go to the previous slide.
    let prev_image = {
        let images = images.clone();
        move |_| {
            let current = current_index.get();
            let new_index = if current == 0 { images.len() - 1 } else { current - 1 };
            set_current_index.set(new_index);
        }
    };
    let preview_image = move || {
        view! { <img src=images[current_index.get()].clone() class="imageSlideItem"  /> }
    };
    let open_image = move |_| {
        if let Some(window) = web_sys::window() {
            let _ = window.open_with_url_and_target(&images2[current_index.get()], "_blank");
        }
    };
    view! { 
        <div class="imageSlideContainer">
            <div 
                class="flex transition-transform duration-500 ease-in-out"
                style=move || format!("transform: translateX(-{}%);", current_index.get() * 100)
            >    
            </div>
                   {preview_image}
           <div class="imageSlideButton"> 
           <button
           type="button"
           on:click=prev_image >
       <Icon icon={i::BiLeftArrowSolid} />
       </button>
       <button
         type="button"
       on:click=open_image >
    
   <Icon icon={i::FiExternalLink} />
   </button>
       <button
       type="button"
           on:click=next_image
          >
       <Icon icon={i::BiRightArrowSolid} />
       </button>
       </div>
        </div>
    }
}
