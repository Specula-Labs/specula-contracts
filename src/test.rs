#![cfg(test)]
extern crate std;
use super::*;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::testutils::Events;
use soroban_sdk::{symbol_short, Symbol, TryFromVal};

#[test]
fn test_initialize_and_threshold() {
    let env = Env::default();
    let contract_id = env.register(StellarSentinel, ());
    let client = StellarSentinelClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    env.mock_all_auths();

    client.initialize(&admin, &75);
    assert_eq!(client.get_threshold(), Some(75));
    assert!(!client.is_agent(&admin));
}

#[test]
#[should_panic(expected = "already initialized")]
fn initialization_is_one_time() {
    let env = Env::default();
    let contract_id = env.register(StellarSentinel, ());
    let client = StellarSentinelClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    env.mock_all_auths();
    client.initialize(&admin, &50);
    client.initialize(&admin, &50);
}

#[test]
#[should_panic(expected = "threshold must be between 0 and 100")]
fn initialization_rejects_out_of_range_threshold() {
    let env = Env::default();
    let contract_id = env.register(StellarSentinel, ());
    let client = StellarSentinelClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    env.mock_all_auths();
    client.initialize(&admin, &101);
}

#[test]
fn authorized_agent_can_flag_at_or_above_threshold() {
    let env = Env::default();
    let contract_id = env.register(StellarSentinel, ());
    let client = StellarSentinelClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let agent = Address::generate(&env);
    let subject = Address::generate(&env);
    env.mock_all_auths();

    client.initialize(&admin, &75);
    client.authorize_agent(&admin, &agent);
    client.flag_anomaly(&agent, &subject, &75);
    client.flag_anomaly(&agent, &subject, &90);
    assert_eq!(
        client.get_latest_flag(&subject),
        Some(FlagRecord {
            agent,
            score: 90,
            ledger: env.ledger().sequence(),
            timestamp: env.ledger().timestamp(),
        })
    );
}

#[test]
fn admin_can_change_threshold_and_revoke_agents() {
    let env = Env::default();
    let contract_id = env.register(StellarSentinel, ());
    let client = StellarSentinelClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let agent = Address::generate(&env);
    env.mock_all_auths();

    client.initialize(&admin, &75);
    client.authorize_agent(&admin, &agent);
    client.set_threshold(&admin, &80);
    assert_eq!(client.get_threshold(), Some(80));
    client.revoke_agent(&admin, &agent);
    assert!(!client.is_agent(&agent));
}


#[test]
fn revoked_agent_can_be_reauthorized_and_flag_again() {
    let env = Env::default();
    let contract_id = env.register(StellarSentinel, ());
    let client = StellarSentinelClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let agent = Address::generate(&env);
    let subject = Address::generate(&env);
    env.mock_all_auths();

    client.initialize(&admin, &75);
    assert!(!client.is_agent(&agent));

    client.authorize_agent(&admin, &agent);
    assert!(client.is_agent(&agent));
    client.revoke_agent(&admin, &agent);
    assert!(!client.is_agent(&agent));

    client.authorize_agent(&admin, &agent);
    assert!(client.is_agent(&agent));
    client.flag_anomaly(&agent, &subject, &75);
    assert_eq!(client.get_latest_flag(&subject).unwrap().score, 75);
}


#[test]
fn latest_flag_is_empty_before_first_flag() {
    let env = Env::default();
    let contract_id = env.register(StellarSentinel, ());
    let client = StellarSentinelClient::new(&env, &contract_id);
    let subject = Address::generate(&env);
    assert_eq!(client.get_latest_flag(&subject), None);
}

#[test]
#[should_panic(expected = "score below risk threshold")]
fn agent_cannot_flag_below_threshold() {
    let env = Env::default();
    let contract_id = env.register(StellarSentinel, ());
    let client = StellarSentinelClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let agent = Address::generate(&env);
    let subject = Address::generate(&env);
    env.mock_all_auths();

    client.initialize(&admin, &75);
    client.authorize_agent(&admin, &agent);
    client.flag_anomaly(&agent, &subject, &74);
}

#[test]
#[should_panic(expected = "not an authorized agent")]
fn unauthorized_address_cannot_flag() {
    let env = Env::default();
    let contract_id = env.register(StellarSentinel, ());
    let client = StellarSentinelClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let agent = Address::generate(&env);
    let subject = Address::generate(&env);
    env.mock_all_auths();

    client.initialize(&admin, &75);
    client.flag_anomaly(&agent, &subject, &90);
}

#[test]
#[should_panic(expected = "not an authorized agent")]
fn revoked_agent_cannot_flag() {
    let env = Env::default();
    let contract_id = env.register(StellarSentinel, ());
    let client = StellarSentinelClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let agent = Address::generate(&env);
    let subject = Address::generate(&env);
    env.mock_all_auths();

    client.initialize(&admin, &75);
    client.authorize_agent(&admin, &agent);
    client.revoke_agent(&admin, &agent);
    client.flag_anomaly(&agent, &subject, &90);
}

#[test]
#[should_panic(expected = "score must be between 0 and 100")]
fn agent_cannot_submit_score_above_100() {
    let env = Env::default();
    let contract_id = env.register(StellarSentinel, ());
    let client = StellarSentinelClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let agent = Address::generate(&env);
    let subject = Address::generate(&env);
    env.mock_all_auths();

    client.initialize(&admin, &75);
    client.authorize_agent(&admin, &agent);
    client.flag_anomaly(&agent, &subject, &101);
}

#[test]
fn set_threshold_accepts_zero() {
#[should_panic(expected = "unauthorized")]
fn set_threshold_rejects_non_admin() {
fn flag_anomaly_emits_flagged_event() {
fn revoked_agent_can_be_re_authorized() {
    let env = Env::default();
    let contract_id = env.register(StellarSentinel, ());
    let client = StellarSentinelClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    env.mock_all_auths();

    client.initialize(&admin, &75);
    client.set_threshold(&admin, &0);
    assert_eq!(client.get_threshold(), 0);
}

#[test]
fn set_threshold_accepts_max_score() {
fn get_threshold_is_none_before_initialization() {
    let env = Env::default();
    let contract_id = env.register(StellarSentinel, ());
    let client = StellarSentinelClient::new(&env, &contract_id);
    assert_eq!(client.get_threshold(), None);
}

#[test]
fn zero_threshold_is_distinguishable_from_unset() {
    let env = Env::default();
    let contract_id = env.register(StellarSentinel, ());
    let client = StellarSentinelClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    env.mock_all_auths();

    client.initialize(&admin, &75);
    client.set_threshold(&admin, &100);
    assert_eq!(client.get_threshold(), 100);
}

#[test]
#[should_panic(expected = "threshold must be between 0 and 100")]
fn set_threshold_rejects_above_max_score() {
    let stranger = Address::generate(&env);
    env.mock_all_auths();

    client.initialize(&admin, &75);
    // Positive control: the real admin succeeds.
    client.set_threshold(&admin, &80);
    assert_eq!(client.get_threshold(), 80);
    // Non-admin is rejected by require_admin.
    client.set_threshold(&stranger, &90);
}

#[test]
#[should_panic(expected = "unauthorized")]
fn authorize_agent_rejects_non_admin() {
    let env = Env::default();
    let contract_id = env.register(StellarSentinel, ());
    let client = StellarSentinelClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    env.mock_all_auths();

    client.initialize(&admin, &75);
    client.set_threshold(&admin, &101);
}

#[test]
fn zero_threshold_accepts_zero_score() {
    let agent = Address::generate(&env);
    let stranger = Address::generate(&env);
    env.mock_all_auths();

    client.initialize(&admin, &75);
    // Positive control: the real admin succeeds.
    client.authorize_agent(&admin, &agent);
    assert!(client.is_agent(&agent));
    // Non-admin is rejected by require_admin.
    client.authorize_agent(&stranger, &agent);
}

#[test]
#[should_panic(expected = "unauthorized")]
fn revoke_agent_rejects_non_admin() {
    let agent = Address::generate(&env);
    let subject = Address::generate(&env);
    env.mock_all_auths();

    client.initialize(&admin, &75);
    client.authorize_agent(&admin, &agent);
    client.flag_anomaly(&agent, &subject, &90);

    let events = env.events().all();
    assert_eq!(events.len(), 1);
    let (_contract, topics, data) = events.get(0).unwrap();
    assert_eq!(topics.len(), 3);
    assert_eq!(
        Symbol::try_from_val(&env, &topics.get(0).unwrap()).unwrap(),
        symbol_short!("flagged")
    );
    assert_eq!(
        Address::try_from_val(&env, &topics.get(1).unwrap()).unwrap(),
        agent
    );
    assert_eq!(
        Address::try_from_val(&env, &topics.get(2).unwrap()).unwrap(),
        subject
    );
    assert_eq!(u32::try_from_val(&env, &data).unwrap(), 90);
    assert_eq!(client.get_latest_flag(&subject).unwrap().score, 90);
}

#[test]
fn rejected_flag_anomaly_emits_no_event() {
    let env = Env::default();
    let contract_id = env.register(StellarSentinel, ());
    let client = StellarSentinelClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let agent = Address::generate(&env);
    let subject = Address::generate(&env);
    env.mock_all_auths();

    client.initialize(&admin, &0);
    client.authorize_agent(&admin, &agent);
    client.flag_anomaly(&agent, &subject, &0);
    assert_eq!(client.get_latest_flag(&subject).unwrap().score, 0);
    let stranger = Address::generate(&env);
    env.mock_all_auths();

    client.initialize(&admin, &75);
    client.authorize_agent(&admin, &agent);
    // Positive control: the real admin succeeds.
    client.revoke_agent(&admin, &agent);
    assert!(!client.is_agent(&agent));
    client.authorize_agent(&admin, &agent);
    // Non-admin is rejected by require_admin.
    client.revoke_agent(&stranger, &agent);
    client.initialize(&admin, &75);
    assert_eq!(env.events().all().len(), 0);

    // An address that was never authorized must panic before publishing.
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        client.flag_anomaly(&agent, &subject, &90);
    }));
    assert!(result.is_err());
    assert_eq!(env.events().all().len(), 0);
    client.initialize(&admin, &0);
    assert_eq!(client.get_threshold(), Some(0));
    let agent = Address::generate(&env);
    env.mock_all_auths();

    client.initialize(&admin, &75);
    client.authorize_agent(&admin, &agent);
    assert!(client.is_agent(&agent));

    client.revoke_agent(&admin, &agent);
    assert!(!client.is_agent(&agent));

    client.authorize_agent(&admin, &agent);
    assert!(client.is_agent(&agent));
}
