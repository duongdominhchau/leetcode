// https://leetcode.com/problems/defanging-an-ip-address/

pub fn defang_i_paddr(address: String) -> String {
    address.replace('.', "[.]")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            defang_i_paddr("1.1.1.1".to_string()),
            "1[.]1[.]1[.]1".to_string()
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(
            defang_i_paddr("255.100.50.0".to_string()),
            "255[.]100[.]50[.]0".to_string()
        );
    }
}
