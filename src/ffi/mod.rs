// This file is Copyright its original authors, visible in version control history.
//
// This file is licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. You may not use this file except in

#[cfg(feature = "uniffi")]
mod types;

#[cfg(feature = "uniffi")]
pub use types::*;

#[cfg(feature = "uniffi")]
pub fn maybe_deref<T, R>(wrapped_type: &std::sync::Arc<T>) -> &R
where
	T: AsRef<R>,
{
	wrapped_type.as_ref().as_ref()
}

/// Extract inner type.
#[cfg(feature = "uniffi")]
#[macro_export]
macro_rules! maybe_extract_inner {
	($value:expr) => {
		TryFrom::try_from($value)?
	};
}

#[cfg(feature = "uniffi")]
pub fn maybe_try_convert_enum<T, R>(wrapped_type: &T) -> Result<R, crate::error::Error>
where
	for<'a> R: TryFrom<&'a T, Error = crate::error::Error>,
{
	R::try_from(wrapped_type)
}

#[cfg(feature = "uniffi")]
pub fn maybe_wrap_arc<T>(ldk_type: impl Into<T>) -> std::sync::Arc<T> {
	std::sync::Arc::new(ldk_type.into())
}

#[cfg(feature = "uniffi")]
pub fn maybe_wrap<T>(ldk_type: impl Into<T>) -> T {
	ldk_type.into()
}

#[cfg(not(feature = "uniffi"))]
pub fn maybe_deref<T>(value: &T) -> &T {
	value
}

#[cfg(not(feature = "uniffi"))]
pub fn maybe_try_convert_enum<T>(value: &T) -> Result<&T, crate::error::Error> {
	Ok(value)
}

#[cfg(not(feature = "uniffi"))]
pub fn maybe_wrap_arc<T>(value: T) -> T {
	value
}

#[cfg(not(feature = "uniffi"))]
pub fn maybe_wrap<T>(value: T) -> T {
	value
}

/// Extract inner type.
#[cfg(not(feature = "uniffi"))]
#[macro_export]
macro_rules! maybe_extract_inner {
	($value:expr) => {
		$value
	};
}

#[cfg(feature = "uniffi")]
pub fn maybe_unwrap_option<T, L>(wrapper_option: Option<T>) -> Option<L>
where
	T: TryInto<L>,
{
	wrapper_option.and_then(|t| t.try_into().ok())
}

#[cfg(not(feature = "uniffi"))]
pub fn maybe_unwrap_option<T>(value: Option<T>) -> Option<T> {
	value
}
