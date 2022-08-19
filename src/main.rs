use azcon::azcon::*;
use futures::executor::block_on;

// AGENDA:
// namespace_report() - handle non-existing clusters
// delete_and_apply() - drilling into erroneous components, delete hr and apply the yaml

fn main() {

    loop {
        print_menu();

        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input).expect("Invalid number!");

        match input.trim().parse::<u8>() {
            Ok(1) => connect_to_cluster(),
            Ok(2) => block_on(namespace_report()),
            Ok(3) => remove_cluster(),
            _ => {
                println!("Exiting >>>>>");
                break;}
        };
    }
}
