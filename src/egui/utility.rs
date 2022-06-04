use egui::Ui;
use english_quotes::quote::{Quote, ALL_PERMS};

pub fn vertical_category_checkbox(ui: &mut Ui, cc: &mut [bool]) {
    ui.vertical(|ui| {
        for (i, cat) in ALL_PERMS.clone().into_iter().enumerate() {
            ui.checkbox(cc.get_mut(i).unwrap(), cat);
        }
    });
}

pub fn get_chosen_types(cc: Vec<bool>) -> Vec<String> {
    cc.into_iter()
        .enumerate()
        .filter_map(|(i, b)| if b { Some(i) } else { None })
        .map(|i| ALL_PERMS[i].clone())
        .collect()
}

pub fn reverse_chosen_types(cats: Vec<String>) -> Vec<bool> {
    let mut res = vec![false; ALL_PERMS.len()];
    cats.into_iter()
        .filter_map(|cat| ALL_PERMS.iter().position(|perm| &cat == perm))
        .for_each(|index| res[index] = true);
    res
}

pub fn display_quotes_list(
    v: impl Iterator<Item = Quote>,
    ui: &mut Ui,
    mut on_click: Option<impl FnMut(Quote)>,
) {
    for quote in v {
        let Quote(txt, cats) = quote.clone();
        if ui.small_button(format!("{cats:?} | {txt}")).clicked() {
            if let Some(on_click) = &mut on_click {
                on_click(quote);
            }
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum QuoteSelectionFilter {
    And,
    Or,
}
impl Default for QuoteSelectionFilter {
    fn default() -> Self {
        Self::Or
    }
}
