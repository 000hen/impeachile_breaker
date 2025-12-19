use rand::{Rng, distr::Alphanumeric};

// const URL_A: &str = "https://impeachlie.doce.cc/api/trpc/petition.submitStage1?batch=1";
// const URL_B: &str = "https://impeachlie.doce.cc/api/trpc/petition.submitStage2?batch=1";
const URL_A: &str = "https://impeachlie.doce.cc/api/trpc/petition.submit?batch=1";
const URL_B: &str = "https://impeachlie.doce.cc/api/trpc/petition.updateSignature?batch=1";

fn random_string(len: usize) -> String {
    let rng = rand::rng();
    rng.sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

fn rand_num_as_string(len: usize) -> String {
    let mut rng = rand::rng();
    (0..len)
        .map(|_| rng.random_range(0..10).to_string())
        .collect()
}

fn send_a_type(client: &reqwest::blocking::Client, thread_id: usize) {
    let name = random_string(10);
    let email = format!(
        "{}@{}.com",
        random_string(rand::random_range(5..20)),
        random_string(rand::random_range(5..20))
    );
    let body = serde_json::json!({"0":{"json":{"name":name,"email":email,"source":"direct"}}});

    match client
        .post(URL_A)
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .send()
    {
        Ok(response) => {
            if response.status().is_success() {
                println!(
                    "[thread:{}] Successfully submitted petition for {}",
                    thread_id, email
                );
            } else {
                eprintln!(
                    "[thread:{}] Failed to submit petition: {}",
                    thread_id,
                    response.status()
                );
            }
        }
        Err(e) => {
            eprintln!("[thread:{}] Error sending request: {}", thread_id, e);
        }
    }
}

fn send_b_type(client: &reqwest::blocking::Client, thread_id: usize) {
    let body = serde_json::json!({
        "0": {
            "json": {
                "signatureId": rand::random_range(10000..99999),
                "phone": rand_num_as_string(11),
                "gender": "other",
                "age": rand::random_range(18..3000),
                "location": "null"
            }
        }
    });

    match client
        .post(URL_B)
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .send()
    {
        Ok(response) => {
            if response.status().is_success() {
                println!("[thread:{}] Successfully submitted petition", thread_id);
            } else {
                eprintln!(
                    "[thread:{}] Failed to submit petition: {}",
                    thread_id,
                    response.status()
                );
            }
        }
        Err(e) => {
            eprintln!("[thread:{}] Error sending request: {}", thread_id, e);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <threads>", args[0]);
        std::process::exit(1);
    }

    let threads: usize = args[1].parse().expect("Invalid number of threads");
    let mut handles = vec![];

    for i in 0..threads {
        let handle = std::thread::spawn(move || {
            let client = reqwest::blocking::Client::new();
            loop {
                // randomly choose between A and B type requests
                send_a_type(&client, i);
                // if rand::random_bool(0.5) {
                //     send_a_type(&client, i);
                // } else {
                //     send_b_type(&client, i);
                // }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
