//! 输入源身份不能覆盖上游，输入模式与连接名必须一致。
#[test]
fn identity_and_modes_are_consistent() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("Info.plist");
    let value = plist::Value::from_file(path).unwrap();
    let dict = value.as_dictionary().unwrap();
    let id = dict["CFBundleIdentifier"].as_string().unwrap();
    assert_eq!(id, "local.localgloss.inputmethod");
    assert_eq!(dict["TISInputSourceID"].as_string().unwrap(), id);
    assert_eq!(
        dict["InputMethodConnectionName"].as_string().unwrap(),
        format!("{id}_Connection")
    );
    assert_eq!(
        dict["InputMethodServerControllerClass"]
            .as_string()
            .unwrap(),
        "LocalGlossInputController"
    );
    let modes = dict["ComponentInputModeDict"].as_dictionary().unwrap();
    let mode = format!("{id}.Hans");
    assert!(
        modes["tsInputModeListKey"]
            .as_dictionary()
            .unwrap()
            .contains_key(&mode)
    );
    assert_eq!(
        modes["tsVisibleInputModeOrderedArrayKey"]
            .as_array()
            .unwrap()[0]
            .as_string(),
        Some(mode.as_str())
    );
}
