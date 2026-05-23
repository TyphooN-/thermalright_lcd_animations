pub const NUMBER_OF_LEDS: usize = 84;

pub const CPU_ALL: &[usize] = &[
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
    40, 41,
];

pub const GPU_ALL: &[usize] = &[
    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60,
    61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79,
    80, 81, 82, 83,
];

pub const CPU_LED: &[usize] = &[0, 1];
pub const CPU_TEMP: &[usize] = &[
    2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
];
pub const CPU_CELSIUS: &[usize] = &[23];
pub const CPU_FAHRENHEIT: &[usize] = &[24];
pub const CPU_USAGE_1_INDICATORS: &[usize] = &[25, 26];
pub const CPU_USAGE: &[usize] = &[27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40];
pub const CPU_PERCENT_LED: &[usize] = &[41];

pub const GPU_PERCENT_LED: &[usize] = &[42];
pub const GPU_USAGE: &[usize] = &[43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56];
pub const GPU_USAGE_1_INDICATORS: &[usize] = &[57, 58];
pub const GPU_CELSIUS: &[usize] = &[59];
pub const GPU_FAHRENHEIT: &[usize] = &[60];
pub const GPU_TEMP: &[usize] = &[
    61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79,
    80, 81,
];
pub const GPU_LED: &[usize] = &[82, 83];
