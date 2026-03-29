//! Integration tests for [`nefaxer::pipeline::explicit_lib_tuning`] (lib tuning skip path).

use nefaxer::Opts;
use nefaxer::disk_detect::DriveType;
use nefaxer::pipeline::explicit_lib_tuning;
use nefaxer::utils::fd_limit::determine_threads_given_fd_limit;

#[test]
fn all_three_fields_returns_fd_capped_threads_and_supplied_drive_walk() {
    let mut opts = Opts::default();
    opts.num_threads = Some(64);
    opts.drive_type = Some(DriveType::HDD);
    opts.use_parallel_walk = Some(false);
    let (n, dt, pw) = explicit_lib_tuning(&opts).expect("all three set");
    assert_eq!(dt, DriveType::HDD);
    assert!(!pw);
    assert_eq!(n, determine_threads_given_fd_limit(64));
}

#[test]
fn any_field_missing_returns_none() {
    let mut opts = Opts::default();
    opts.num_threads = Some(4);
    opts.drive_type = Some(DriveType::SSD);
    assert!(explicit_lib_tuning(&opts).is_none());

    let mut opts = Opts::default();
    opts.drive_type = Some(DriveType::SSD);
    opts.use_parallel_walk = Some(true);
    assert!(explicit_lib_tuning(&opts).is_none());
}
