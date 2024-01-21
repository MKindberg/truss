use skim::prelude::*;
use skim::SkimItem;

pub fn select<T: SkimItem + Clone>(items: &Vec<T>) -> T {
    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .multi(false)
        .select1(true)
        .exit0(true)
        .preview_window(Some("right:50%:wrap"))
        .preview(Some("")) // preview should be specified to enable preview window
        .build()
        .unwrap();

    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();
    items.iter().cloned().map(Arc::new).for_each(|x| {
        let _ = tx_item.send(x);
    });
    drop(tx_item); // so that skim could know when to stop waiting for more items.

    let res = &Skim::run_with(&options, Some(rx_item)).unwrap();
    if res.final_event == Event::EvActAbort {
        std::process::exit(1);
    }
    let i = &res.selected_items[0];
    return (**i)
        .as_any()
        .downcast_ref::<T>()
        .expect("Failed to downcast")
        .clone();
}
