use crate::app::components::forms::{ CheckBox, InputArrayField, InputField, TextEditor };
use crate::app::components::records::PortfolioRecords;
use crate::app::components::utils::{ show_error_toast, show_success_toast };
use crate::app::components::layouts::TabRender;
use crate::app::models::Portfolio;
use leptos::*;
use uuid::Uuid;

#[component]
pub fn EditPortfolioTab(
    portfolios: ReadSignal<Vec<Portfolio>>,
    set_portfolios: WriteSignal<Vec<Portfolio>>,
    set_is_update_portfolio: WriteSignal<bool>,
    select_tab: ReadSignal<i32>
) -> impl IntoView {
    let (portfolio_name, set_portfolio_name) = create_signal(String::new());
    let (portfolio_link, set_portfolio_link) = create_signal(String::new());
    let (is_opensource, set_is_opensource) = create_signal(false);
    let (portfolio_icon_url, set_portfolio_icon_url) = create_signal(String::new());
    let (portfolio_detail, set_portfolio_detail) = create_signal(String::new());
    let (screenshots_url, set_screenshots_url) = create_signal(vec!["".to_string()]);
    let (stacks, set_stacks) = create_signal(vec!["".to_string()]);
    let (use_portfolio_detail_pdf_version, set_use_portfolio_detail_pdf_version) = create_signal(
        bool::from(false)
    );
    let (portfolio_detail_pdf_data, set_portfolio_detail_pdf_data) = create_signal(String::new());

    let (validate_portfolio, set_validate_portfolio) = create_signal(false);

    let add_portfolio = move |_| {
        set_validate_portfolio.update(|v| {
            *v = !*v;
        });
        let form_valid =
            !portfolio_name.get().trim().is_empty() && !portfolio_detail.get().trim().is_empty();
        if form_valid {
            let new_portfolio = Portfolio {
                uuid: Uuid::new_v4().to_string(),
                index: (portfolios.get().len() + 1) as u8,
                portfolio_name: portfolio_name.get(),
                portfolio_detail: portfolio_detail.get(),
                portfolio_icon_url: portfolio_icon_url.get(),
                portfolio_link: portfolio_link.get(),
                is_opensource: is_opensource.get(),
                screenshots_url: screenshots_url.get(),
                stacks: stacks.get(),
                use_portfolio_detail_pdf_version: use_portfolio_detail_pdf_version.get(),
                portfolio_detail_pdf_data: Some(portfolio_detail_pdf_data.get()),
            };
            set_portfolios.update(|portfolio| portfolio.push(new_portfolio));
            set_validate_portfolio.set(false);
            set_portfolio_name.set(String::new());
            set_portfolio_detail.set(String::new());
            set_portfolio_icon_url.set(String::new());
            set_portfolio_detail_pdf_data.set(String::new());
            set_use_portfolio_detail_pdf_version.set(bool::from(false));
            set_portfolio_link.set(String::new());
            set_is_opensource.set(false);
            set_screenshots_url.set(vec!["".to_string()]);
            set_stacks.set(vec!["".to_string()]);
            set_is_update_portfolio(true);
            show_success_toast("Add Portfolio Success", "Portfolio Added.");
        } else {
            show_error_toast("Add Portfolio Failed", "Missing required field.");
        }
    };
    let delete_portfolio = move |index: usize| {
        set_portfolios.update(|portfolios| {
            portfolios.remove(index);
            for i in index..portfolios.len() {
                portfolios[i].index = (i + 1) as u8;
            }
        });
        set_is_update_portfolio(true)
    };
    let edit_portfolio = move |index: usize| {
        let list = portfolios.get();
        if
            let Some(portfolio) = list
                .iter()
                .enumerate()
                .find(|(i, _)| *i == index)
        {
            let portfolio = portfolio.1.clone();
            set_portfolio_name.set(portfolio.portfolio_name);
            set_portfolio_link.set(portfolio.portfolio_link);
            set_is_opensource.set(portfolio.is_opensource);
            set_portfolio_detail.set(portfolio.portfolio_detail);
            set_portfolio_icon_url.set(portfolio.portfolio_icon_url);
            set_portfolio_detail_pdf_data.set(
                portfolio.portfolio_detail_pdf_data.unwrap_or(String::from(""))
            );
            set_use_portfolio_detail_pdf_version.set(portfolio.use_portfolio_detail_pdf_version);
            set_stacks.set(portfolio.stacks);
            set_screenshots_url.set(portfolio.screenshots_url);
            delete_portfolio(index);
        }
    };

    view! {
        <TabRender  no=4 active_page=select_tab>
        <Show when=move || select_tab() == 4>
        <Suspense fallback=move || view! { <p>"LoadingIntro..."</p> }>
        <div class="editContainer">
        <h1>"Edit Portfolio"</h1>              
        <InputField input_type="text" id="portfolio_name" label="Project Name" validation=validate_portfolio set_value=set_portfolio_name  get_value=portfolio_name require=true />
        {move ||view! { <CheckBox id="is_opensource"  label= "Opensource" set_value=set_is_opensource  get_value=is_opensource />}}
        <InputField input_type="text" id="portfolio_link" label="Project Link Url" set_value=set_portfolio_link  get_value=portfolio_link require=false />
        <InputField input_type="text" id="portfolio_icon_url" label="Project Icon Url" set_value=set_portfolio_icon_url  get_value=portfolio_icon_url require=false />       
        { move ||
            if select_tab() == 4  {
          view!{
            <div>         
            <TextEditor
            label="Project Detail"
            id="portfolio_detail"
            validation=validate_portfolio
            disabled=false
            require=true
            get_value=portfolio_detail
            set_value=set_portfolio_detail
            />
        </div>
            }
            }else{
                view!{ <div></div> }
            }
        }
        <InputArrayField id="screenshots_url" label="Screenshots url" set_fields=set_screenshots_url  get_values=screenshots_url require=false />
        <InputArrayField  id="stacks" label="Project Stack" set_fields=set_stacks  get_values=stacks require=false />
        <CheckBox id="use_portfolio_detail_pdf_version"  label= "Use Portfolio Detail PDF version" set_value=set_use_portfolio_detail_pdf_version get_value=use_portfolio_detail_pdf_version />
        { move ||
          if select_tab() == 4  && use_portfolio_detail_pdf_version.get() {
        view!{
          <div>  <TextEditor
          label="Portfolio Detail (PDF Version)"
          id="portfolio_detail_pdf_data"
          validation=validate_portfolio
          disabled=false
          require=true
          get_value=portfolio_detail_pdf_data
          set_value=set_portfolio_detail_pdf_data
           />
           </div>
          }
          }else{
              view!{ <div></div> }
          }
          }  
               <button
                type="button"
                class="addButton"
                on:click=add_portfolio >
                "Add Portfolio Project"
            </button>
          <PortfolioRecords
          portfolios=portfolios
          is_edit=true
          set_is_update=set_is_update_portfolio
          set_portfolios=set_portfolios
          on_delete=Callback::new(move |index| delete_portfolio(index))
          on_edit=Callback::new(move |index| edit_portfolio(index))
          />
          </div>
          </Suspense>
          </Show>
        </TabRender>
    }
}
