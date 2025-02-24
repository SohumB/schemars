use crate::gen::SchemaGenerator;
use crate::schema::*;
use crate::JsonSchema;
use std::borrow::Cow;

macro_rules! decimal_impl {
    ($type:ty) => {
        decimal_impl!($type => Number, "Number");
    };
    ($type:ty => $instance_type:ident, $name:expr) => {
        impl JsonSchema for $type {
            no_ref_schema!();

            fn schema_name() -> String {
                $name.to_owned()
            }

            fn schema_id() -> Cow<'static, str> {
                Cow::Borrowed("Decimal")
            }

            fn json_schema(_: &mut SchemaGenerator) -> Schema {
                SchemaObject {
                    instance_type: Some(InstanceType::$instance_type.into()),
                    ..Default::default()
                }
                .into()
            }
        }
    };
}

#[cfg(feature = "rust_decimal")]
decimal_impl!(rust_decimal::Decimal);
#[cfg(feature = "bigdecimal03")]
decimal_impl!(bigdecimal03::BigDecimal);
#[cfg(feature = "bigdecimal04")]
decimal_impl!(bigdecimal04::BigDecimal);
