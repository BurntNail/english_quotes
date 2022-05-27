use egui::Ui;
use english_quotes::quote::ALL_PERMS;

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
