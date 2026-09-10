use rust_extensions::StrOrString;
use serde::*;
use service_sdk::my_no_sql_sdk::abstractions::Timestamp;

service_sdk::macros::use_my_no_sql_entity!();

pub const ABUSE_KEY_TYPE_IP: &str = "ip";

const IPV4_KEY_PREFIX: &str = "v4_";
const IPV6_KEY_PREFIX: &str = "v6_";
const IPV6_PREFIX_BITS: usize = 64;
const IPV6_PREFIX_BYTES: usize = IPV6_PREFIX_BITS / 8;

/// Active abuse restriction, written by risk-groups and read by whoever gates the action.
/// Not related to `trader-blockers`: that one blocks an existing account, this one a key.
#[my_no_sql_entity("abuse-restrictions")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct AbuseRestrictionMyNoSqlEntity {
    // Server-side expiration is housekeeping only; gates decide on restricted_until.
    pub expires: Timestamp,
    pub restricted_until: i64,
    pub rule_id: String,
    pub created_at: i64,
    pub reason: String,
}

impl AbuseRestrictionMyNoSqlEntity {
    pub fn generate_partition_key<'s>(key_type: impl Into<StrOrString<'s>>) -> StrOrString<'s> {
        key_type.into()
    }

    pub fn generate_row_key<'s>(key_value: impl Into<StrOrString<'s>>) -> StrOrString<'s> {
        key_value.into()
    }

    pub fn is_active(&self, now_unix_microseconds: i64) -> bool {
        self.restricted_until > now_unix_microseconds
    }
}

/// Counter key of an IP, shared so that the gate and the rule evaluator cannot drift apart.
/// IPv6 collapses to its /64 — one mobile subscriber owns the whole /64 and rotates the low bits.
pub fn build_ip_key(ip: &str) -> Option<String> {
    use std::fmt::Write;

    let parsed: std::net::IpAddr = ip.trim().parse().ok()?;

    let v6 = match parsed {
        std::net::IpAddr::V4(v4) => return Some(format!("{}{}", IPV4_KEY_PREFIX, v4)),
        std::net::IpAddr::V6(v6) => v6,
    };

    if let Some(v4) = v6.to_ipv4_mapped() {
        return Some(format!("{}{}", IPV4_KEY_PREFIX, v4));
    }

    let mut key = String::with_capacity(IPV6_KEY_PREFIX.len() + IPV6_PREFIX_BYTES * 2);
    key.push_str(IPV6_KEY_PREFIX);

    for byte in &v6.octets()[..IPV6_PREFIX_BYTES] {
        write!(key, "{:02x}", byte).expect("writing to a String never fails");
    }

    Some(key)
}

#[cfg(test)]
mod tests {
    use super::build_ip_key;

    #[test]
    fn ipv4_keeps_the_full_address() {
        assert_eq!(build_ip_key("89.105.199.178").unwrap(), "v4_89.105.199.178");
        assert_eq!(build_ip_key(" 1.2.3.4 ").unwrap(), "v4_1.2.3.4");
    }

    #[test]
    fn ipv6_collapses_to_prefix() {
        let rotated_low_bits = build_ip_key("2a01:cb08:1234:5678:aaaa:bbbb:cccc:dddd").unwrap();

        assert_eq!(rotated_low_bits, "v6_2a01cb0812345678");
        assert_eq!(
            build_ip_key("2a01:cb08:1234:5678::1").unwrap(),
            rotated_low_bits
        );
    }

    #[test]
    fn different_prefixes_stay_apart() {
        assert_ne!(
            build_ip_key("2a01:cb08:1234:5678::1").unwrap(),
            build_ip_key("2a01:cb08:1234:5679::1").unwrap()
        );
    }

    #[test]
    fn ipv4_mapped_counts_as_ipv4() {
        assert_eq!(build_ip_key("::ffff:1.2.3.4").unwrap(), "v4_1.2.3.4");
    }

    #[test]
    fn unparsable_input_has_no_key() {
        assert!(build_ip_key("").is_none());
        assert!(build_ip_key("Unknown").is_none());
        // An X-Forwarded-For chain is not an address — the caller must resolve one hop first.
        assert!(build_ip_key("203.0.113.77, 89.105.199.178").is_none());
    }
}
