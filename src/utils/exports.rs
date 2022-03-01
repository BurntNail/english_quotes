use crate::{ALL_PERMS, db::read_db};
use std::{fs::File, io::Write};

pub fn export() {
    let list = read_db().expect("a database needs to exist");
    let mut f = File::create("export.md").expect("need to be able to open the file");
    writeln!(f, "# Jack's WIB Quotes\n").unwrap();

    for perm in ALL_PERMS {
        write!(f, "## {}\n", perm).unwrap();
        list.clone()
            .into_iter()
            .filter(|quote| quote.1.contains(perm))
            .for_each(|quote| {
                let index = quote.1.iter().position(|x| x == perm);
                let mut new_list = quote.1;
                new_list.remove(index.unwrap());

                writeln!(f, " - *{}*, related to **{:?}**", quote.0, new_list).unwrap();
            });
        writeln!(f).unwrap();
    }
}
