// extern crate calcver;

// use calcver::error::{CalcverErrorReason};

// #[test]
// fn return_library_error_if_repo_not_found () {
//     let repo =  calcver::project::Project::from("NONEXISTENT").finalize();
//     match calcver::get_version(&repo, calcver::VersionBumpBehavior::Auto, false).err().unwrap().reason {
//         CalcverErrorReason::Library(reason) => assert!(true, "Should throw CalcverError with reason: Library"),
//         _ => assert!(false,"Should throw CalcverError with reason: Library")
//     }
// }