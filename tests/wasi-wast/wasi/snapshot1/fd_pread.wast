(wasi_test "fd_pread.wasm"
  (preopens "test_fs")
  (assert_return (i64.const 0))
  (assert_stdout " POLONIUS\n\n    He will come straight. Look you lay home to him:\n\n POLONIUS\n\n    He will come straight. Look you lay home to him:\n\nRead the same data? true\n")
)