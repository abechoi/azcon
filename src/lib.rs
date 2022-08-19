pub mod azcon {

    use std::process::Command;
    use std::time::Duration;
    use std::thread;

    pub fn print_menu() {

        let current_context = get_current_context();
    
        println!("\n\
        <<<<<<<<<<< AZCON MENU >>>>>>>>>>>\n\n\
        Currently connected to {}\n\
        1. Connect to cluster\n\
        2. Namespace report\n\
        3. Delete a cluster\n\n\
        any key to exit\n", current_context);
    }

    pub fn get_current_context() -> String {

        let output = Command::new("kubectl").arg("config").arg("current-context").output().expect("Error: verify that you are connected to a cluster.");
        String::from_utf8(output.stdout).unwrap()
    }
    
    pub fn connect_to_cluster() {
    
        let mut raw_input = String::new();
    
        println!("\nChoose a cluster to connect to:\n");
        let contexts: Vec<String> = get_contexts();
    
        std::io::stdin().read_line(&mut raw_input).expect("Invalid number!");
    
        let input = raw_input.trim();
        let range = 1..=contexts.len();

        if input.parse::<usize>().is_ok() && range.contains(&input.parse::<usize>().unwrap()) {

            let ref c = &contexts[input.parse::<usize>().unwrap()-1];
            use_context(c.to_string());

        } else {
            println!("\nError: enter a number from the list\n\n<<<<< Go back");
        }
    }
    
    pub fn remove_cluster() {
    
        let mut raw_input: String = String::new();
    
        println!("\nChoose a cluster to remove:\n");
        let contexts: Vec<String> = get_contexts();
    
        std::io::stdin().read_line(&mut raw_input).expect("Invalid number!");

        let input = raw_input.trim();
        let range = 1..=contexts.len();

        if input.parse::<usize>().is_ok() && range.contains(&input.parse::<usize>().unwrap()) {

            let ref c = &contexts[input.parse::<usize>().unwrap()-1];
            delete_context(c.to_string());
            
        } else {
            println!("\nError: enter a number from the list\n\n<<<<< Go back");
        }
    }

    
    pub fn get_contexts() -> Vec<String> {
    
        let output = Command::new("kubectl").arg("config").arg("get-contexts").output().expect("ERROR");
        let content = String::from_utf8(output.stdout).unwrap();
        let mut contexts: Vec<String> = Vec::new();
    
        for line in content.lines().skip(1) {
            if line.split_whitespace().nth(0).unwrap() == "*"{
    
                contexts.push(line.split_whitespace().nth(1).unwrap().to_string());
    
            } else {
                contexts.push(line.split_whitespace().nth(0).unwrap().to_string());
            }
        }
    
        for (i, context) in contexts.iter().enumerate() {
            println!("{}. {}", i+1, context);
        }
        println!("\nany key to go back\n");
    
        contexts
    }

    pub fn use_context(context: String) {

        Command::new("kubectl").arg("config").arg("use-context").arg(context).spawn().expect("ERROR");
        wait(1000);
    }

    pub fn delete_context(context: String) {

        Command::new("kubectl").arg("config").arg("delete-context").arg(context).spawn().expect("ERROR");
        wait(1000);
    }
    
    pub fn wait(ms: u16) {
        
        let time: Duration = Duration::from_millis(ms.into());
        thread::sleep(time);
    }

    pub async fn namespace_report() {

        println!("Enter namespace: ");
    
        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input).expect("Error: check namespace or try a different cluster.");
        
        let input = input.trim().to_string();
    
        get_pods(&input).await;
    
        get_deployments(&input).await;
    
        get_hr(&input).await;
    
    }

    pub async fn get_deployments(input: &str) {

        let output = Command::new("kubectl").arg("get").arg("deployments").arg("--namespace").arg(input).output().expect("ERROR");
        wait(1000);
    
        let content = String::from_utf8(output.stdout).unwrap();
    
        let mut fail_count = 0;
        let mut deployments_summary = String::new();
    
        for line in content.lines().skip(1) {

            println!("TESTING: {}", line);
            
            if line.contains("0/") {
                fail_count += 1;
                deployments_summary += line;
                deployments_summary += "\n";
            }
        }
        if fail_count == 0 {
            deployments_summary += "All deployments are available!";
        }
        println!("\n\
            <<<<<<<<<<<<<<<<<<<<<<<<<<<<<< DEPLOYMENTS SUMMARY >>>>>>>>>>>>>>>>>>>>>>>>>>>>>\n\n\
            {}", deployments_summary);
    }
    
    pub async fn get_pods(input: &str) {
    
        let output = Command::new("kubectl").arg("get").arg("pods").arg("--namespace").arg(input).output().expect("ERROR");
        wait(1000);
    
        let content = String::from_utf8(output.stdout).unwrap();
    
        let mut fail_count = 0;
        let mut pods_summary = String::new();
    
        for line in content.lines().skip(1) {
            if !line.contains("Running") {
                fail_count += 1;
                pods_summary += line.trim();
                pods_summary += "\n";
            }
        }
        if fail_count == 0 {
            pods_summary += "All pods in 'Running' state!";
        }
    
        println!("\n\
            <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<< PODS SUMMARY >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>\n\n\
            {}", pods_summary);
    }
    
    pub async fn get_hr(input: &str) {
    
        let output = Command::new("kubectl").arg("get").arg("hr").arg("--namespace").arg(input).output().expect("ERROR");
        wait(1000);
    
        // content is the String version of Command
        let content = String::from_utf8(output.stdout).unwrap();
    
        let mut fail_count: i32 = 0;
        let mut hr_summary = String::new();
        
        for line in content.lines().skip(1) {
            if !line.contains("Succeeded") {
                fail_count += 1;
                hr_summary += line.trim();
                hr_summary += "\n";
            }
        }
        if fail_count == 0 {
            hr_summary += "All hr in 'Succeeded' phase!";
        }
        
        println!("\n\
            <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<< HR SUMMARY >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>\n\n\
            {}", hr_summary);
    }
}