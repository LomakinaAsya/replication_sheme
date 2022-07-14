use crate::replication;
use crate::format_utils;

#[derive(Debug, Clone)]
pub struct Scheme {
    hosts_amount: usize,
    ports_amount: usize,
    routers_amount: usize,
    storages_amount: usize,
    replicas_amount: usize,
    customs_amount: usize,
    next_index: usize,
    genin_index: usize,
    pub values: Vec<String>,
}

impl Scheme {
    pub fn new_schema(
        hosts_amount: usize,
        ports_amount: usize,
        routers_amount: usize,
        storages_amount: usize,
        replicas_amount: usize,
        customs_amount: usize
    ) -> Option<Scheme> {

        let required_space = routers_amount + storages_amount * (replicas_amount + 1) + customs_amount;
        let actual_space = hosts_amount * ports_amount;

        if actual_space <= required_space {
            println!("not enough server space");
            return None
        }

        let mut values: Vec<String> = Vec::new();
        for _i in 0..ports_amount {
            for _j in 0..hosts_amount {
                values.push(String::from(""));
            }
        }

        Some(
            Scheme {
                hosts_amount: hosts_amount,
                ports_amount: ports_amount,
                routers_amount: routers_amount,
                storages_amount: storages_amount,
                replicas_amount: replicas_amount,
                customs_amount: customs_amount,
                next_index: 0,
                genin_index: 0,
                values: values,
            }
        )
    }

    pub fn get_hosts(&self) -> usize {
        self.hosts_amount.clone()
    }
    pub fn get_ports(&self) -> usize {
        self.ports_amount.clone()
    }
    pub fn get_routers(&self) -> usize {
        self.routers_amount.clone()
    }
    pub fn get_storages(&self) -> usize {
        self.storages_amount.clone()
    }
    pub fn get_replicas(&self) -> usize {
        self.replicas_amount.clone()
    }
    pub fn get_customs(&self) -> usize {
        self.customs_amount.clone()
    }
    pub fn get_genin_index(&self) -> usize {
        self.genin_index
    }

    pub fn get_next(&mut self) -> usize {
        let index = self.next_index;

        self.genin_index = (self.next_index + 1) % self.hosts_amount;
        self.next_index += 1;

        index.clone()
    }

    pub fn print_schema(self) {
        format_utils::print_vec_as2d(self.values, self.hosts_amount);
    }

    fn put_routers(mut self) -> Self {
        let mut count = 1;

        while count < self.routers_amount + 1 {
            let index = self.get_next();
            let colored_alias = format_utils::colored_alias(format!("r{}", count));
            self.values[index] = colored_alias;
            count += 1;
        }

        self
    }

    fn put_storages(mut self) -> Self {
        let mut count = 1;
        while count < self.storages_amount + 1 {
            let index = self.get_next();
            let colored_alias = format_utils::colored_alias(format!("m{}", count));
            self.values[index] = colored_alias;
            count += 1;
        }

        self
    }

    fn put_replicas(mut self) -> Self {
        self = replication::put_replicas(self);
        self
    }

    fn put_customs(mut self) -> Self {
        let mut count = 1;

        while count < self.customs_amount + 1 {
            let index = self.get_next();
            let colored_alias = format_utils::colored_alias(format!("c{}", count));
            self.values[index] = colored_alias;
            count += 1;
        }

        self
    }

    pub fn put_instances(self) -> Self {
        self.put_routers()
            .put_storages()
            .put_replicas()
            .put_customs()
    }
}