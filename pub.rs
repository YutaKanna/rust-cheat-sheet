// どこからでも見える
pub fn public_function() {}

// 属するcrateから見える
pub(crate) fn crate_wide_public_function() {}

// 親モジュールから見える
pub(super) fn parent_module_wide_public_function() {}
