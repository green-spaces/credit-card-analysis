use crate::Squirrel;

pub async fn display_categories(sq: &Squirrel) {
    let categories = sq.all_dc().await;
    for cate in categories {
        println!("{} ({})", cate.name, cate.id);
    }
}
