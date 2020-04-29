* by default test runs parallel
* if wanna run in single thread 
  * cargo test -- --test-threads=1
* we dont see println! code we only see line that indicates test passed
* can disable output capture using
  * cargo test -- --nocapture
* cargo test single_test_name
* cargo test identifier // this runs all tests containing identifier
* #[ignore] // to ignore a particular test
* cargo test -- --ignored  // to run ignored tests
* 