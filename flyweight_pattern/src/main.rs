mod heavy_struct {
    use std::rc::Rc;

    struct MyHeavyStruct {
        super_heavy_data: String,
    }

    struct InstanceOfMyHeavyStruct {
        data: Rc<MyHeavyStruct>,
        id: usize,
    }

    pub fn example() {
        let mut instances = Vec::new();
        {
            let pointer_to_data = Rc::new(MyHeavyStruct {
                super_heavy_data: "MyHeavyStruct".to_string(),
            });
            for i in 0..10 {
                let instance = InstanceOfMyHeavyStruct {
                    data: pointer_to_data.clone(),
                    id: i,
                };
                instances.push(instance);
            }
        }
        for instance in instances {
            println!("{} {}", instance.id, instance.data.super_heavy_data);
        }
    }
}

mod shared_variable_state {
    use std::rc::Rc;

    struct CreatorsName {
        name: String,
    }

    struct Resources {
        owner: Rc<CreatorsName>,
    }

    pub fn example() {
        let mut resources = Vec::new();
        {
            let bob = Rc::new(CreatorsName {
                name: "Bob".to_string(),
            });

            let alice = Rc::new(CreatorsName {
                name: "Alice".to_string(),
            });

            for i in 0..10 {
                let resource = Resources {
                    owner: if i % 2 == 0 {
                        bob.clone()
                    } else {
                        alice.clone()
                    },
                };
                resources.push(resource);
            }
        }
        for resource in resources {
            println!("{}", resource.owner.name);
        }
    }
}

fn main() {
    heavy_struct::example();
    shared_variable_state::example();
}
