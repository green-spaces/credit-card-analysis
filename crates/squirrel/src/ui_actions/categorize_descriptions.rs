use crate::Squirrel;

pub async fn categorize_descriptions(sq: &Squirrel) {
    let bld_to_categorize = sq.bld_not_categorized().await;
    for bld in bld_to_categorize {
        println!("{:#?}", bld);
        println!("Actions: [p]rint categories, [c]reate category, [m]ap category id, [n]ext");
        let mut resp = String::new();
        std::io::stdin().read_line(&mut resp).unwrap();
        match resp.get(..1).unwrap() {
            "p" => {}
            "c" => {
                println!("Enter Category Name:");
                let mut name = String::new();
                std::io::stdin().read_line(&mut name).unwrap();
                let trimmed_name = name.trim();
                let cate_id = sq.dc_create(trimmed_name).await;
                sq.map_dc_to_bld(cate_id, bld.id).await.unwrap();
            }
            "m" => {}
            "n" => continue,
            _ => todo!(),
        }
    }
}
