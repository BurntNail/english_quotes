use crate::{
    db::read_db,
    quote::{FileType, ALL_PERMS},
    utils::Error,
};
use std::{fs::File, io::Write};

#[allow(clippy::missing_panics_doc)]
pub fn export() -> Result<(), Error> {
    let list = read_db()?;
    let mut f =
        File::create(FileType::Export.get_location()).expect("need to be able to open the file");
    writeln!(f, "# Jack's WIB Quotes\n").map(|_| ())?;

    for perm in ALL_PERMS.iter() {
        let perm = perm.to_string();
        writeln!(f, "## {}", perm).map(|_| ())?;

        let new_list = list
            .clone()
            .into_iter()
            .filter(|quote| quote.1.contains(&perm));

        for quote in new_list {
            let index = quote.1.iter().position(|x| x == &perm);
            let mut new_list = quote.1;

            new_list.remove(index.unwrap()); //PANIC: can't panic - boom

            writeln!(f, " - *{}*, related to **{:?}**", quote.0, new_list).map(|_| ())?;
        }
        writeln!(f).map(|_| ())?;
    }

    Ok(())
}
