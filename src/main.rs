fn main() {
    let result: Result<(), ()> = Ok(());

    if let Err(e) = result {
        eprintln!("error: {e:?}");
    }
}
