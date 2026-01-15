//! Decoder registration and type identification macros

/// Trait for extracting type name from account enum variants
pub trait AccountTypeIdentifier {
    fn type_name(&self) -> &'static str;
}

/// Macro to implement AccountTypeIdentifier for an account enum
///
/// Usage:
/// ```ignore
/// impl_type_identifier!(CargoAccount => CargoPod, CargoStatsDefinition, CargoType);
/// ```
macro_rules! impl_type_identifier {
    ($account_enum:ty => $($variant:ident),* $(,)?) => {
        impl $crate::decoder::AccountTypeIdentifier for $account_enum {
            fn type_name(&self) -> &'static str {
                match self {
                    $(Self::$variant(_) => stringify!($variant),)*
                }
            }
        }
    };
}

pub(crate) use impl_type_identifier;

/// Macro to run analysis for a feature-gated decoder
macro_rules! run_decoder_analysis {
    ($ctx:expr, $feature:literal, $decoder:expr, $account_type:ty) => {
        #[cfg(feature = $feature)]
        {
            let decoder = $decoder;
            $crate::analysis::type_classification::analyze::<_, $account_type>($ctx, &decoder)?;

            if tracing::enabled!(tracing::Level::DEBUG) {
                $crate::analysis::sample_accounts::analyze::<_, $account_type>($ctx, &decoder)?;
            }
        }
    };
}

pub(crate) use run_decoder_analysis;
