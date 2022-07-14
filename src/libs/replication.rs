
use crate::vector_utils;
use crate::format_utils;
use crate::scheme::Scheme;

pub fn put_replicas(scheme: Scheme) -> Scheme {
    let hosts_amount = scheme.get_hosts();
    let storages_amount = scheme.get_storages();

    let replicas = create_replicas(&scheme);

    if storages_amount % hosts_amount == 0
        || hosts_amount % storages_amount == 0
    { put_equal(scheme, replicas) }

    else if storages_amount < hosts_amount
    { put_less(scheme, replicas) }

    else { put_greater(scheme, replicas) }
}

fn create_replicas(scheme: &Scheme) -> Vec<Vec<String>> {
    let storages_amount = scheme.get_storages();
    let replicas_amount = scheme.get_replicas();

    let mut replicas = Vec::new();
    for i in 0..storages_amount {
        replicas.push(Vec::new());
        for j in 0..replicas_amount {
            let colored_alias = format_utils::colored_alias(format!("mr{}-{}", i+1, j+1));
            replicas[i].push(colored_alias);
        }
    }
    replicas
}

fn permutate(indexes: Vec<usize>, mut count: usize) -> Vec<usize> {
    let mut output = indexes.clone();

    while count > 0 {
        output = vector_utils::first_to_last(output);
        count -= 1;
    }
    output
}

fn put_equal(mut scheme: Scheme, replicas: Vec<Vec<String>>) -> Scheme {
    let storages_amount = scheme.get_storages();
    let replicas_amount = scheme.get_replicas();

    let mut i = 0;
    let mut count = 0;
    let mut indexes: Vec<usize> = Vec::new();
    for i in 0..storages_amount {
        indexes.push(i);
    }

    while count < replicas_amount {
        indexes = permutate(indexes, 1);
        if (i + 1) % storages_amount == 0 {
            indexes = permutate(indexes, 1);
        }
            for j in 0..indexes.len() {
                let scheme_index = scheme.get_next();
                // println!(
                //     "put {} on host-{} ",
                //     replicas[indexes[j]][i].clone(),
                //     scheme.get_genin_index() + 1
                // );
                scheme.values[scheme_index] = replicas[indexes[j]][i].clone();
            }
            i += 1;
            count += 1;
    }
    scheme
}

fn put_greater(mut scheme: Scheme, replicas: Vec<Vec<String>>) -> Scheme {
    let storages_amount = scheme.get_storages();
    let replicas_amount = scheme.get_replicas();

    let mut indexes: Vec<usize> = Vec::new();
    for k in 0..storages_amount {
        indexes.push(k);
    }

    indexes = vector_utils::reverb_by_pair(indexes);
    for i in 0..replicas_amount {

        for j in 0..indexes.len() {
            let scheme_index = scheme.get_next();
            // println!(
            //     "put {} on host-{} ",
            //     replicas[indexes[j]][i].clone(),
            //     scheme.get_genin_index() + 1
            // );
            scheme.values[scheme_index] = replicas[indexes[j]][i].clone();
        }
        indexes = permutate(indexes, 1);
    }
    scheme
}


fn put_less(mut scheme: Scheme, replicas: Vec<Vec<String>>) -> Scheme {
    let storages_amount = scheme.get_storages();
    let replicas_amount = scheme.get_replicas();

    let mut indexes: Vec<usize> = Vec::new();
    for k in 0..storages_amount {
        indexes.push(k);
    }

    for i in 0..replicas_amount {
        indexes = permutate(indexes, i);
        for j in 0..indexes.len() {
            let scheme_index = scheme.get_next();
            // println!(
            //     "put {} on host-{} ",
            //     replicas[indexes[j]][i].clone(),
            //     scheme.get_genin_index() + 1
            // );
            scheme.values[scheme_index] = replicas[indexes[j]][i].clone();
        }
    }
    scheme
}
