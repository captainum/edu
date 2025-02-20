use std::collections::HashMap;

fn parser(s: &str, departments: &mut HashMap<&str, Vec<String>>) {
    let parsed: Vec<&str> = s.split_whitespace().collect();
    let first_word = parsed[0]; 

    if first_word == "Add" {
        assert_eq!(parsed.len(), 4);
        assert_eq!(parsed[2], "to");

        let name: &str = parsed[1];
        let department = parsed[3];

        assert_eq!(departments.contains_key(department), true);

        if let Some(vec) = departments.get_mut(department) {
            vec.push(String::from(name));
        }
    }
    else if first_word == "List" {
        for (department, employees) in departments {
            println!("Department: {department}");
            let mut st = String::from("");
            for empl in employees {
                match st.len() {
                    0 => st = format!("{}", empl),
                    _ => {
                        st.push_str(", ");
                        st.push_str(empl);
                    }
                }
            }
            println!("{}", st);
            println!("");
        }
    }
    else {
        println!("command not found!:(");
    };
}

fn main() {
    let mut departments: HashMap<&str, Vec<String>>= HashMap::new();

    departments.insert("Engineering", Vec::new());
    departments.insert("Sales", Vec::new());

    parser("Add Amir to Engineering", &mut departments);
    parser("Add Sally to Sales", &mut departments);
    parser("Add Amir to Sales", &mut departments);
    parser("Add Sally to Engineering", &mut departments);
    parser("List", &mut departments);

}
