pub mod message_keys {
    use paste::paste;
    macro_rules! define_message_keys {
        ($($name:ident),* $(,)?) => {
            pub mod externs {
                unsafe extern "C" {
                  $(
                    pub safe static $name: u32;
                  )*
                }
            }

            paste! {
            $(
                #[allow(non_snake_case)]
                #[unsafe(no_mangle)]
                pub extern "C" fn [<GET_ $name>]() -> u32 {
                    // SAFETY: Pebble message keys are resolved at link time and never change.
                        externs::$name
                }

                #[allow(non_snake_case)]
                pub fn $name() -> u32 { [<GET_ $name>]() }
            )*
            }
        };
    }

    define_message_keys! {
        MESSAGE_KEY_SETTING_ACCENT_COLOR,
        MESSAGE_KEY_WEATHER_CURRENT_TEMP,
        MESSAGE_KEY_WEATHER_DAY_NAME,
        MESSAGE_KEY_WEATHER_DAY_TEMP_HIGH,
        MESSAGE_KEY_WEATHER_DAY_TEMP_LOW,
        MESSAGE_KEY_WEATHER_DAY_UPDATE_IDX,
        MESSAGE_KEY_WEATHER_HOUR_CLOUD_COVER,
        MESSAGE_KEY_WEATHER_HOUR_RAIN_AMOUNT,
        MESSAGE_KEY_WEATHER_HOUR_RAIN_CHANCE,
        MESSAGE_KEY_WEATHER_HOUR_UPDATE_IDX,
        MESSAGE_KEY_WEATHER_LOCATION_NAME,
        MESSAGE_KEY_WEATHER_REPORT_START,
        MESSAGE_KEY_WEATHER_REQUEST,
        MESSAGE_KEY_WEATHER_SEG_IS_DAY,
        MESSAGE_KEY_WEATHER_SEG_SUN_LEVEL,
        MESSAGE_KEY_WEATHER_SEG_TEMPERATURE,
        MESSAGE_KEY_WEATHER_SEG_UPDATE_IDX,
    }
}

pub mod resource_ids {
    pub const RESOURCE_ID_LM_MONO_48: u32 = 1;
}
