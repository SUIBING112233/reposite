#[macro_export]
macro_rules! default_builder_fn {
    ($var_name:ident,$var_type:ty,$builder_type:ty) => {
        pub fn $var_name(mut self, $var_name: $var_type) -> $builder_type {
            self.$var_name = $var_name;
            self
        }
    };
}

/// The default Builder pattern code for structs
///
/// # Example
///
/// ```edition2021
/// use crate::default_builder;
/// use crate::default_builder_fn;
///
/// # fn main() {
/// default_builder!(
///     // Copy your struct to here
///     #[derive(PartialEq, Debug, Clone)]
///     pub struct Test {
///         pub test_field: String,
///     },
///     // Naming your struct builder
///     TestBuilder
/// );
/// # }
/// ```
///
/// After expansion, it is the following code
///
/// ```edition2021
/// use crate::default_builder;
/// use crate::default_builder_fn;
///
/// # fn main() {
/// #[derive(PartialEq, Debug, Clone)]
/// pub struct Test {
///     pub test_field: String,
/// }
///
/// pub struct TestBuilder {
///     // private
///     test_field: String,
/// }
///
/// impl TestBuilder {
///     default_builder_fn!(test_field, String, TestBuilder);
///     pub fn build(self) -> Test {
///         Test {
///             test_field: self.test_field,
///         }
///     }
/// }
/// # }
/// ```
///
/// # Add extra logic to your Builder
/// If you need to implement additional logic, you need to implement your own Impl statement block.
///
/// This macro *does not implement the Builder constructor*, so you need to implement the Builder constructor **yourself**.
#[macro_export]
macro_rules! default_builder {
    (
        $(#[$meta:meta])*
        $target_struct_vis:vis struct $target_struct_name:ident
        {
            $($field_vis:vis $field_name:ident : $field_type:ty,)*
        },
        $builder_struct_name:ident
    ) => {
        // Target struct
        $(#[$meta])*
        $target_struct_vis struct $target_struct_name{$(pub $field_name : $field_type,)*}

        // Builder struct
        pub struct $builder_struct_name{$($field_name : $field_type,)*}

        // Builder function
        impl $builder_struct_name {
            $(default_builder_fn!($field_name, $field_type, $builder_struct_name);)*

            // build function
            pub fn build(self)-> $target_struct_name{
                $target_struct_name {
                    $(
                        $field_name : self.$field_name,
                    )*
                }
            }
        }
    };
}
