use crate::integrations;
pub mod types;

/// Show your current monthly progress
pub fn show_monthly_hours() {
    refresh()
}

pub fn refresh() {
    // 1. tarkista onko tarvetta synkata
    // 1.a synkkaa jos edellisestä synkista on yli 3 tuntia (tää vois olla asetuksissa?)
    // 2. hae kuukauden tunnit
    // 3. printtaile ne ruudulle
    println!("refresh");
    let time_entries = integrations::get_monthly_time_entries();

    println!("{:?}", time_entries)
}
