pub mod azcon {

    use std::process::Command;
    use std::time::Duration;
    use std::process::Output;
    use std::thread;

    pub fn print_menu() {

        let current_context:String = if get_current_context() == "" {
            "NONE".to_string()
        } else { get_current_context() };
    
        println!("\n\
        <<<<<<<<<<< AZCON MENU >>>>>>>>>>>\n\n\
        Currently connected to {}\n\
        1. Connect to cluster\n\
        2. Namespace report\n\
        3. Delete a cluster\n\n\
        any key to exit\n", current_context);
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

    pub async fn namespace_report() {

        println!("Enter namespace: ");
    
        let mut raw_input: String = String::new();
        std::io::stdin().read_line(&mut raw_input).expect("Error: check namespace or try a different cluster.");
        
        let namespace = raw_input.trim().to_string();

        if check_namespace(&namespace) {

            get_deployments(&namespace).await;

            let pods: Vec<String> = get_pods(&namespace).await;
    
            let hr: Vec<String> = get_hr(&namespace).await;

            // components_report(hr, pods);
            components_report(namespace, hr, pods);
        }else {
            println!("Namespace not found, verify cluster.");
        }
    }

    fn components_report(namespace: String, hr: Vec<String>, pods: Vec<String>) {

        let ref ns = namespace;

        let mut i: usize = 1;
        let mut report: String = String::new();

        println!("\n\n<<<<<<<<<< HR COMPONENTS | PODS COMPONENTS >>>>>>>>>>\n");

        for (i, line) in hr.iter().enumerate() {
            println!("{}. {}", i + 1, line);
        }
        for (i, line) in pods.iter().enumerate() {
            println!("{}. {}", i + 1 + hr.len(), line);
        }
        // helm status and helm history for hr
        // describe pods for pods
        let mut raw_input: String = String::new();
        std::io::stdin().read_line(&mut raw_input).expect("Error: invalid choice...");

        let input = raw_input.trim();
        let range1 = 1..=hr.len();
        let range2 = 1 + hr.len()..= pods.len() + hr.len();

        if input.parse::<usize>().is_ok() && range1.contains(&input.parse::<usize>().unwrap()) {

            let ref component_name = &hr[input.parse::<usize>().unwrap()-1];
            get_status(ns.to_string(), component_name.to_string());
            get_history(ns.to_string(), component_name.to_string());
            
        } else if input.parse::<usize>().is_ok() && range2.contains(&input.parse::<usize>().unwrap()) {

            let ref pod_name = &pods[input.parse::<usize>().unwrap()-1];
            get_event(ns.to_string(), pod_name.to_string());
        
        } else {
            println!("\nError: enter a number from the list\n\n<<<<< Go back");
        }
    
    }

    fn get_status(namespace: String, component_name: String) {
        let output = Command::new("helm").arg("status").arg(component_name).arg("-n").arg(namespace).output().expect("ERROR");
        let status = String::from_utf8(output.stdout).unwrap();

        println!("{}", status);
    }

    fn get_history(namespace: String, component_name: String) {
        let output = Command::new("helm").arg("history").arg(component_name).arg("-n").arg(namespace).output().expect("ERROR");
        let history = String::from_utf8(output.stdout).unwrap();

        println!("{}", history);
    }

    fn get_event(namespace: String, pod_name: String) {

        let field_selector: String = format!("involvedObject.name={}", pod_name);
        let output: Output = Command::new("kubectl").arg("get").arg("event").arg("--namespace").arg(namespace).arg("--field-selector").arg(field_selector).output().expect("Cannot be found");
        let event: String = String::from_utf8(output.stdout).unwrap();

        println!("{}", event);
    }

    fn check_namespace(input: &str) -> bool {

        println!("Searching cluster for namespace...");

        let output = Command::new("kubectl").arg("get").arg("namespaces").output().expect("Error: namespace does not exist.");
        wait(1000);
        let content = String::from_utf8(output.stdout).unwrap();

        let mut is_true = false;

        for line in content.lines().skip(1) {

            let namespace = line.split_whitespace().nth(0).unwrap();

            if namespace == input {
                is_true = true;
                println!("Found {}", namespace);
                println!("Fetching data...");
            }
        }
        is_true
    }

    fn get_current_context() -> String {

        let output = Command::new("kubectl").arg("config").arg("current-context").output().expect("Error: verify that you are connected to a cluster.");
        String::from_utf8(output.stdout).unwrap()
    }
    
    fn get_contexts() -> Vec<String> {
    
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

    fn use_context(context: String) {

        Command::new("kubectl").arg("config").arg("use-context").arg(context).spawn().expect("ERROR");
        wait(1000);
    }

    fn delete_context(context: String) {

        Command::new("kubectl").arg("config").arg("delete-context").arg(context).spawn().expect("ERROR");
        wait(1000);
    }
    
    fn wait(ms: u16) {
        
        let time: Duration = Duration::from_millis(ms.into());
        thread::sleep(time);
    }

    async fn get_deployments(input: &str) {

        let output: Output = Command::new("kubectl").arg("get").arg("deployments").arg("--namespace").arg(input).output().expect("ERROR");
        wait(1000);
    
        let content: String = String::from_utf8(output.stdout).unwrap();
    
        let mut fail_count: i32 = 0;
        let mut deployments_summary: String = String::new();
    
        for line in content.lines().skip(1) {            
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
    
    async fn get_pods(input: &str) -> Vec<String> {
    
        let output: Output = Command::new("kubectl").arg("get").arg("pods").arg("--namespace").arg(input).output().expect("ERROR");
        wait(1000);
    
        let content: String = String::from_utf8(output.stdout).unwrap();
    
        let mut fail_count: i32 = 0;
        let mut pods_summary: String = String::new();
        let mut pods: Vec<String> = Vec::<String>::new(); 
    
        for line in content.lines().skip(1) {
            if !line.contains("Running") {
                fail_count += 1;
                pods_summary += line.trim();
                pods_summary += "\n";
                pods.push(line.split_whitespace().nth(0).unwrap().to_string());
            }
        }
        if fail_count == 0 {
            pods_summary += "All pods in 'Running' state!";
        }
    
        println!("\n\
            <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<< PODS SUMMARY >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>\n\n\
            {}", pods_summary);

        pods
    }
    
    async fn get_hr(input: &str) -> Vec<String> {
    
        let output: Output = Command::new("kubectl").arg("get").arg("hr").arg("--namespace").arg(input).output().expect("ERROR");
        wait(1000);
    
        // content is the String version of Command
        let content: String = String::from_utf8(output.stdout).unwrap();
    
        let mut fail_count: i32 = 0;
        let mut hr_summary: String = String::new();
        let mut hr: Vec<String> = Vec::<String>::new();
        
        for line in content.lines().skip(1) {
            if !line.contains("Succeeded") {
                fail_count += 1;
                hr_summary += line.trim();
                hr_summary += "\n";
                hr.push(line.split_whitespace().nth(0).unwrap().to_string());
            }
        }
        if fail_count == 0 {
            hr_summary += "All hr in 'Succeeded' phase!";
        }
        
        println!("\n\
            <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<< HR SUMMARY >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>\n\n\
            {}", hr_summary);

        hr
    }
}