use chrono::{ Datelike, NaiveDate, NaiveDateTime, Utc };
use phf::phf_map;
use icondata as i;
use wasm_bindgen::prelude::*;
pub fn calculate_age(birth_date: &str) -> i64 {
    let parsed_date = NaiveDate::parse_from_str(birth_date, "%Y-%m-%d").unwrap();
    let now = Utc::now().naive_utc().date(); // Get the current date in UTC
    let birth_year = parsed_date.year();
    let birth_month = parsed_date.month();
    let birth_day = parsed_date.day();
    let current_year = now.year();
    let current_month = now.month();
    let current_day = now.day();

    let mut age = current_year - birth_year;
    if current_month < birth_month || (current_month == birth_month && current_day < birth_day) {
        age -= 1;
    }

    age as i64
}
pub fn convert_date_format(input: &String) -> String {
    if input == "Now" {
        "Now".to_string()
    } else {
        let formats = ["%Y-%m-%d", "%m/%d/%Y", "%d-%m-%Y"];
        for fmt in formats {
            if let Ok(date) = NaiveDate::parse_from_str(input, fmt) {
                return date.format("%m/%Y").to_string();
            }
        }
        // Fallback to default date
        NaiveDate::from_ymd_opt(2000, 1, 1).unwrap().format("%m/%Y").to_string()
    }
}
pub fn format_date_for_input(date_str: &str) -> String {
    // Parse the ISO 8601 date string
    if let Ok(datetime) = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%SZ") {
        // Format as YYYY-MM-DD
        datetime.format("%Y-%m-%d").to_string()
    } else {
        // Return a default date if parsing fails
        String::from("2000-01-01")
    }
}

pub fn get_icon_by_name(name: &str) -> Option<&'static icondata_core::IconData> {
    ICON_MAP.get(name).copied()
}

pub static ICON_MAP: phf::Map<
    &'static str,
    &'static icondata_core::IconData
> = phf_map! {
    "Mail" => i::AiMailOutlined,
    "Facebook" => i::AiFacebookOutlined,
    "Twitter" => i::FaSquareXTwitterBrands,
    "Linkedin" => i::AiLinkedinOutlined,
    "Home" => i::AiHomeOutlined,
    "Tiktok" => i::BiTiktok,
    "Phone" => i::BiPhoneCallSolid,
    "Instagram" => i::AiInstagramOutlined,
    "Whatsapp" => i::BsWhatsapp,
    "Wechat" => i::AiWechatOutlined,
    "Line" => i::FaLineBrands,
    "Discord" => i::BiDiscordAlt,
    "Telegram" => i::BiTelegram,
    "Snapchat" => i::IoLogoSnapchat,
    "Weibo" => i::AiWeiboOutlined,
    "Reddit" => i::BiReddit,
    "Github" => i::BiGithub,
    "Gitlab" => i::BiGitlab,
    "Bitbucket" => i::IoLogoBitbucket,
    "Viber" => i::FaViberBrands,
    "Slack" => i::AiSlackOutlined,
    "Kakaotalk" => i::SiKakaotalk,
    "Kik" => i::SiKik,
    "QQ" => i::AiQqCircleFilled,
    "Signal" => i::BsSignal,

};
pub static FONT_AWESOME_MAP: phf::Map<
    &'static str,
    &'static str
> = phf_map! {
    "Bitbucket" => "fab fa-bitbucket",
    "Discord" => "fab fa-discord",
    "Facebook" => "fab fa-facebook",
    "Github" => "fab fa-github",
    "Gitlab" => "fab fa-gitlab",
    "Home" => "fas fa-house", 
    "Instagram" => "fab fa-instagram",
    "Kakaotalk" => "fas fa-comment", 
    "Kik" => "fab fa-kik",
    "Line" => "fab fa-line",
    "Linkedin" => "fab fa-linkedin",
    "Mail" => "fas fa-envelope", 
    "Phone" => "fas fa-phone",
    "QQ" => "fab fa-qq",
    "Reddit" => "fab fa-reddit",
    "Signal" => "fab fa-signal-messenger", 
    "Slack" => "fab fa-slack",
    "Snapchat" => "fab fa-snapchat",
    "Telegram" => "fab fa-telegram",
    "Tiktok" => "fab fa-tiktok",
    "Twitter" => "fab fa-square-x-twitter",
    "Viber" => "fab fa-viber",
    "Wechat" => "fab fa-weixin", 
    "Weibo" => "fab fa-weibo",
    "Whatsapp" => "fab fa-whatsapp",
    "Website" => "fas fa-globe",
    "Portfolio" => "fas fa-briefcase",
    "Address" => "fas fa-map-marker-alt",
};
#[wasm_bindgen(module = "/assets/localstorage.js")]
extern "C" {
    pub fn setLocalStorage(key: &str, value: &str) -> JsValue;
    pub fn getLocalStorage(key: &str) -> JsValue;
    pub fn removeLocalStorage(key: &str);
}
