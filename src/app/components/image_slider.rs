use leptos::*;
use leptos::IntoView;
use leptos_icons::Icon;
use icondata as i;
use crate::app::components::Dialog;
#[component]
pub fn ImageSlider(images: Vec<String>) -> impl IntoView {
    // Signal to keep track of the current slide index.
    let (current_index, set_current_index) = create_signal(0);
    let images_preview = images.clone();

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
        view! { <img src=images_preview[current_index.get()].clone() class="imageSlideItem"  /> }
    };

    let (open_dialog, set_open_dialog) = create_signal(false);
    view! { 
       <div> 
       { move || { 
        if open_dialog.get() { 
            let clone_images = images.clone();
            let dialog_image =  clone_images[current_index.get()].clone();
            view!  {<div  on:click=move |_| {
            set_open_dialog.set(!open_dialog.get()); }>
            <Dialog children_only=true >
            <img alt="avatar" src={dialog_image.clone()} />
        </Dialog>
            </div>}}
        else {
            view! {<div></div>}
        } 
       }  }
       <div class="imageSlideContainer">
  
           <div 
               class="flex transition-transform duration-500 ease-in-out"
               style=move || format!("transform: translateX(-{}%);", current_index.get() * 100)
           >
               
           </div>
                  {preview_image}
          <div class="imageSlideButton"> 
          <button
          on:click=prev_image
          class="absolute left-0 top-1/2 transform -translate-y-1/2 bg-gray-800 text-white px-4 py-2 opacity-75 hover:opacity-100"
      >
      <Icon icon={i::BiLeftArrowSolid} />
      </button>
      <button
      on:click=move |_| {
        set_open_dialog.set(!open_dialog.get());   }
 
      class="absolute right-0 top-1/2 transform -translate-y-1/2 bg-gray-800 text-white px-4 py-2 opacity-75 hover:opacity-100"
  >
  <Icon icon={i::BsArrowsFullscreen} />
  </button>
      <button
          on:click=next_image
          class="absolute right-0 top-1/2 transform -translate-y-1/2 bg-gray-800 text-white px-4 py-2 opacity-75 hover:opacity-100"
      >
      <Icon icon={i::BiRightArrowSolid} />
      </button>
      </div>
       </div>
       </div>
    }
}
