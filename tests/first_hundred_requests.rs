use std::process::Command; // Run programs

#[test]
#[cfg(feature = "nyc")]
fn nyc_query1() -> Result<(), Box<dyn std::error::Error>> {
    let binary_path = env!("CARGO_BIN_EXE_tpe_pi_rust");
    let mut cmd = Command::new(binary_path);
    cmd.status().expect("Termination failed");

    let fixture = std::fs::read_to_string("fixtures/first_hundred_requests/query1.csv")
        .expect("Failed to read query1.csv fixture");

    let output = std::fs::read_to_string("query1.csv").expect("Failed to read query1.csv");

    assert_eq!(fixture, output, "Output does not match expected fixture");

    Ok(())
}

#[test]
#[cfg(feature = "nyc")]
fn nyc_query2() -> Result<(), Box<dyn std::error::Error>> {
    let binary_path = env!("CARGO_BIN_EXE_tpe_pi_rust");
    let mut cmd = Command::new(binary_path);
    cmd.status().expect("Termination failed");

    let fixture = std::fs::read_to_string("fixtures/first_hundred_requests/query2.csv")
        .expect("Failed to read query2.csv fixture");

    let output = std::fs::read_to_string("query2.csv").expect("Failed to read query2.csv");

    assert_eq!(fixture, output, "Output does not match expected fixture");

    Ok(())
}

#[test]
#[cfg(feature = "nyc")]
fn nyc_query3() -> Result<(), Box<dyn std::error::Error>> {
    let binary_path = env!("CARGO_BIN_EXE_tpe_pi_rust");
    let mut cmd = Command::new(binary_path);
    cmd.status().expect("Termination failed");

    let fixture = std::fs::read_to_string("fixtures/first_hundred_requests/query3.csv")
        .expect("Failed to read query3.csv fixture");

    let output = std::fs::read_to_string("query3.csv").expect("Failed to read query3.csv");

    assert_eq!(fixture, output, "Output does not match expected fixture");

    Ok(())
}
