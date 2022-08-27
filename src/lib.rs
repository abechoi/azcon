pub mod azcon {

    use std::process::Command;
    use std::process::Output;

    pub fn print_menu() {

        let current_context:String = if get_current_context() == "" {
            "NONE".to_string()
        } else { get_current_context() };
    
        println!("\n\n\
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

            let _deployments: Vec<String> = get_deployments(&namespace).await;
            let pods: Vec<String> = get_pods(&namespace).await;
            let hr: Vec<String> = get_hr(&namespace).await;

            if pods.len() > 0 || hr.len() > 0 {
                components_report(&namespace, pods, hr).await;
            }            
        }else {
            println!("Namespace not found, verify cluster.");
        }
    }

    async fn components_report(namespace: &str, pods: Vec<String>, hr: Vec<String>) {

        let ref ns = namespace;

        loop {

            print_components_report(pods.clone(), hr.clone());
            
            // helm status and helm history for hr
            // describe pods for pods
            let mut raw_input: String = String::new();
            std::io::stdin().read_line(&mut raw_input).expect("Error: invalid choice...");

            let input = raw_input.trim();
            let hr_range = 1..= hr.len();
            let pods_range = 1 + hr.len()..= pods.len() + hr.len();
            
            if input.parse::<usize>().is_ok() {

                if hr_range.contains(&input.parse::<usize>().unwrap()) {

                    let ref component_name = &hr[input.parse::<usize>().unwrap()-1];
                    get_history(ns.to_string(), component_name.to_string()).await;

                } else if pods_range.contains(&input.parse::<usize>().unwrap()) {

                    let ref pod_name = &pods[input.parse::<usize>().unwrap() - hr.len() - 1];
                    get_events(ns.to_string(), pod_name.to_string()).await;

                } else {
                    println!("\nError: enter a number from the list\n\n<<<<< Go back");
                }
                
            } else {
                println!("\n\n<<<<< Go back");
                break;
            }
        }
    
    }

    fn print_components_report(pods: Vec<String>, hr: Vec<String>) {

        let mut range = if hr.len() > pods.len() { 0..hr.len() } else { 0..pods.len() };


        println!("\n\n<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<      COMPONENTS  REPORT      >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>\n");
        println!("|                                   |                                                                        |");
        println!("| For helm status and history:      | For pod events:                                                        |");
        println!("|                                   |                                                                        |");

        for i in &mut range {

            if i < pods.len() && i < hr.len() {

                println!("| {0: <2} {1: <30} | {2: <2} {3: <67} |", i + 1 , hr[i], i + 1 + hr.len(), pods[i]);

            } else if i >= pods.len() && i < hr.len() {

                println!("| {0: <2} {1: <30} |                                                                    |", i + 1 , hr[i]);

            } else if i < pods.len() && i >= hr.len() {

                println!("|                                   | {0: <2} {1: <67} |", i + 1 + hr.len(), pods[i]);
            }
        }
        println!("\nany key to exit");
    }

    async fn get_history(namespace: String, component_name: String) {

        // helm history admin-tenants --namespace servicetitan-halo -o=json
        // helm history admin- --namespace servicetitan-eclipse 
        let output: Output = Command::new("helm").arg("history").arg(&component_name).arg("--namespace").arg(&namespace).output().expect("ERROR");
        let history: String = String::from_utf8(output.stdout).unwrap();

        if history == "" {
            println!("No history for component: {}", component_name);
        } else {

            println!("\n<<<<<<<<<<  HISTORY  >>>>>>>>>>\n");
            println!("Namespace: {}", namespace);
            println!("Component: {}\n", component_name);


            for line in history.lines().skip(1) {

                let v: Vec<_> = line.split_whitespace().into_iter().collect();

                let revision: String = v[0].to_string();
                let status: String = v[6].to_string();

                let d: Vec<_> = v.into_iter().skip(9).collect();
                let description: String = d.join(" ");

                println!("Revision: {:<2} Status: {}\nDescription: {}\n", revision, status, description);

            }
        }
        println!("\n\n<<<<< Go back");
    }

    async fn get_events(namespace: String, pod_name: String) {

        let field_selector: String = format!("involvedObject.name={}", pod_name);

        // output event in stringified json format
        // kubectl get event --namespace <namespace> --field-selector involvedObject.name=<pod name>
        let output: Output = Command::new("kubectl").arg("get").arg("event").arg("--namespace").arg(namespace).arg("--field-selector").arg(field_selector).output().expect("Cannot be found");
        let events: String = String::from_utf8(output.stdout).unwrap();

        if events == "" {
            println!("No logs for pod: {}", pod_name);
        } else {

            println!("\n<<<<<<<<<<  EVENTS  >>>>>>>>>>\n");
            println!("Pod Name: {}\n", pod_name);

            for line in events.lines().skip(1) {

                let v: Vec<_> = line.split_whitespace().into_iter().skip(4).collect();
                let message: String = v.join(" ");

                println!("Message: {}\n", message);
            }
        }
        println!("\n\n<<<<< Go back");
        
    }

    fn check_namespace(input: &str) -> bool {

        println!("Searching cluster for namespace...");

        let output = Command::new("kubectl").arg("get").arg("namespaces").output().expect("Error: namespace does not exist.");
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
    }

    fn delete_context(context: String) {

        Command::new("kubectl").arg("config").arg("delete-context").arg(context).spawn().expect("ERROR");
    }

    async fn get_deployments(input: &str) -> Vec<String> {

        let output: Output = Command::new("kubectl").arg("get").arg("deployments").arg("--namespace").arg(input).output().expect("ERROR");
    
        let content: String = String::from_utf8(output.stdout).unwrap();
    
        let mut fail_count: i32 = 0;
        let mut deployments_summary: String = String::new();
        let mut deployments: Vec<String> = Vec::<String>::new();
    
        for line in content.lines().skip(1) {            
            if line.contains("0/") {
                fail_count += 1;
                deployments_summary += line;
                deployments_summary += "\n";
                deployments.push(line.to_string());
            }
        }
        if fail_count == 0 {
            deployments_summary += "All deployments are available!";
        }
        println!("\n\
            <<<<<<<<<<<<<<<<<<<<<<<<<<<<<< DEPLOYMENTS SUMMARY >>>>>>>>>>>>>>>>>>>>>>>>>>>>>\n\n\
            {}", deployments_summary);

        deployments
    }
    
    async fn get_pods(input: &str) -> Vec<String> {
    
        let output: Output = Command::new("kubectl").arg("get").arg("pods").arg("--namespace").arg(input).output().expect("ERROR");
    
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