use crate::Squirrel;

pub async fn spending_summary(sq: &Squirrel) {
    let res = sq.all_dc_and_bl().await;
    println!("Spending summary");
    for (cat, bls) in res {
        println!("\n{} ({:.2})", cat.name, total_debit(&bls));
        // for bl in bls {
        //     println!("{:?}", bl);
        // }
    }
}

fn total_debit(bill_lines: &[db_entity::entity::bill_line::Model]) -> f64 {
    bill_lines.iter().filter_map(|bl| bl.debit).sum::<f64>()
}
