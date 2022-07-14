use libs::scheme::Scheme;


fn main() {
    let hosts_amount: usize = 4;
    let ports_amount: usize = 8;
    let routers_amount: usize = 1;
    let storages_amount: usize = 6;
    let replicas_amount: usize = 3;
    let customs_amount: usize = 2;

    match Scheme::new_schema(
        hosts_amount,
        ports_amount,
        routers_amount,
        storages_amount,
        replicas_amount,
        customs_amount,
    ) {
        Some(schema) => schema.put_instances().print_schema(),
        None => (),
    }
}
