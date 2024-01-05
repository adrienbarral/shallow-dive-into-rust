impl Named for Identity {
    fn whats_your_name(&self) -> String {
        self.name.clone()
    }
}

impl Named for Teacher {
    fn whats_your_name(&self) -> String {
        self.identity.name.clone()
    }
}

fn does_its_name_is_nice(named: &dyn Named) -> bool {
    if named.whats_your_name().contains("Adrien") {
        return true;
    }
    return false;
}

fn main() {
    let adrien = Teacher{
        identity: Identity{name: String::from("Adrien BARRAL")},
        salary: 40_000
    };
    
    let john = Student {
        identity: Identity(name: String::from("John SMITH")),
        class: String::from("MIR Master")
    };
    assert_eq!(does_its_name_is_nice(&adrien), true);
    assert_eq!(does_its_name_is_nice(&john.identity), false);
}
