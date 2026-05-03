// 1. Struct definition
// 'a means: "The address held by this Broker has a specific lifetime (validity period)"
struct Broker<'a> {
    building_address: &'a str,
}

// 2. Implementation Block
// We write impl<'a> so that we can use the lifetime 'a in the methods below
impl<'a> Broker<'a> {
    // CASE A: Broker-dependent Access ('b)
    // 'b is tied to &self. Meaning: The key is valid only as long as the Broker 'exists'.
    fn get_temporary_access<'b>(&'b self) -> &'b str {
        println!("Obtained a temporary key from the Broker...");
        self.building_address
    }

    // CASE B: Data-dependent Access ('a)
    // 'a is tied to the actual data. Meaning: The address remains valid as long as the building exists.
    // Even if the Broker (self) is dropped in the meantime!
    fn get_permanent_address(&self) -> &'a str {
        println!("Recorded the permanent address from the Broker...");
        self.building_address
    }
}

fn main() {
    // First, we create the 'Building' (Owned Data)
    // Its lifetime is the longest (lasting until the end of the main function)
    let building_data = String::from("Antilia, Mumbai");

    // Create variables to store results
    let address_perm: &str;
    // let address_temp: &str; // If this is uncommented, it will cause an error below
    let mut _address_temp = "";

    {
        // Create a small scope where the 'Broker' will be created
        println!("--- Inside the scope ---");
        let broker = Broker {
            building_address: &building_data,
        };

        // 1. Get the permanent address (which returns lifetime 'a)
        address_perm = broker.get_permanent_address();

        // 2. Get a temporary key (which returns lifetime 'b)
        _address_temp = broker.get_temporary_access();
        println!("Inside scope: Temporary access valid: {}", _address_temp);

        println!("Scope is about to end, the Broker is about to leave...");
    } // <--- Here the 'broker' (self) is DROPPED!

    println!("--- Outside the scope ---");

    // TEST 1: Will the permanent address still work?
    // YES! Because 'address_perm' had lifetime 'a', which was tied to 'building_data'.
    // The Broker is gone, but the address is still valid.
    println!("Outside scope: Permanent address: {}", address_perm);

    /*
    // TEST 2: Will the temporary access (key) still work?
    // NO! If you uncomment the line below, Rust will give an error.
    // Error: "borrowed value does not live long enough"
    // Because the lifetime 'b' of 'address_temp' ended with the 'broker'.

    // println!("Outside scope: Temporary access: {}", _address_temp);
    */

    println!("The building still exists: {}", building_data);
}
