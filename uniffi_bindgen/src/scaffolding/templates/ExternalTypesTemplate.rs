// Support for external types.

// Types with an external `FfiConverter`...
{% for (name, crate_name, kind, tagged) in ci.iter_external_types() %}
// The FfiConverter for `{{ name }}` is defined in `{{ crate_name }}`
// If it has its existing FfiConverter defined with a UniFFITag, it needs forwarding.
{% if tagged %}
{% if ci.is_name_used_as_error(name) %}
::uniffi::ffi_converter_error_forward!(r#{{ name }}, ::{{ crate_name|crate_name_rs }}::UniFfiTag, crate::UniFfiTag);
{% else %}
{%- match kind %}
{%- when ExternalKind::DataClass %}
::uniffi::ffi_converter_forward!(r#{{ name }}, ::{{ crate_name|crate_name_rs }}::UniFfiTag, crate::UniFfiTag);
{%- when ExternalKind::Interface %}
::uniffi::ffi_converter_arc_forward!(r#{{ name }}, ::{{ crate_name|crate_name_rs }}::UniFfiTag, crate::UniFfiTag);
{%- when ExternalKind::Trait %}
::uniffi::ffi_converter_arc_forward!(dyn r#{{ name }}, ::{{ crate_name|crate_name_rs }}::UniFfiTag, crate::UniFfiTag);
{%- endmatch %}
{% endif %}
{% endif %}
{%- endfor %}
