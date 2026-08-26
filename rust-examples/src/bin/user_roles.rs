/*User Roles
enum Role {
    Admin,
    User,
    Guest,
}

Give each role different permissions. */

enum Role {
    Admin,
    User,
    Guest,
}

fn permissions(role: Role) -> &'static str {
    match role {
        Role::Admin => "Full access",
        Role::User => "Read and write",
        Role::Guest => "Read only",
    }
}

// alternative approach
fn can_delete(role: &Role) -> bool {
    match role {
        Role::Admin => true,
        Role::User => false,
        Role::Guest => false,
    }
}

fn can_write(role: &Role) -> bool {
    match role {
        Role::Admin => true,
        Role::User => true,
        Role::Guest => false,
    }
}

fn can_read(role: &Role) -> bool {
    match role {
        Role::Admin => true,
        Role::User => true,
        Role::Guest => true,
    }
}

fn main() {
    let admin_role = Role::Admin;
    let user_role = Role::User;
    let guest_role = Role::Guest;

    let role = Role::User;

    println!("Can read: {}", can_read(&role));
    println!("Can write: {}", can_write(&role));
    println!("Can delete: {}", can_delete(&role));
    println!("Admin permissions: {}", permissions(admin_role));
    println!("User permissions: {}", permissions(user_role));
    println!("Guest permissions: {}", permissions(guest_role));
}