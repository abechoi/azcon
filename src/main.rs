use azcon::azcon::*;
use futures::executor::block_on;

// AGENDA:
// loop components_report to utilize get_status(), get_history(), and get_event() multiple times
// format output to look pretty
// delete_and_apply() - drilling into erroneous components, delete hr and apply the yaml

// kubectl get event --namespace servicetitan-halo --field-selector involvedObject.name=servicetitan-halo-worker-message-consumer-95f5f4586-f65bc 

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
