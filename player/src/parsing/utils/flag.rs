pub(crate) fn check_flag(flag: &[u8; 3], target_flag: &[u8; 3]) -> bool {
    ((flag[0] & target_flag[0]) | (flag[1] & target_flag[1]) | (flag[2] & target_flag[2])) != 0
}
