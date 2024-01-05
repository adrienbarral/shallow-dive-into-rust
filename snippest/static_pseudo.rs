fn whats_your_name_identity(identity: &Identity) -> String {
    identity.name.clone()
}

fn whats_your_name_teacher(teacher: &Teacher) -> String {
    teacher.identity.name.clone()
}

fn does_its_name_is_nice_identity(identity: &Identity) -> bool {
    if identity.whats_your_name().contains("Adrien") {
        return true;
    }
    return false;
}

fn does_its_name_is_nice_teacher(teacher: &Teacher) -> bool {
    if teacher.whats_your_name().contains("Adrien") {
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
    assert_eq!(does_its_name_is_nice_teacher(&adrien), true);
    assert_eq!(does_its_name_is_nice_identity(&john.identity), false);
}
