#[macro_use]
mod macros;

test!(
    preserves_named_color_case,
    "a {\n  color: OrAnGe;\n}\n",
    "a {\n  color: OrAnGe;\n}\n"
);
test!(
    named_color_casing_is_color,
    "a {\n  color: hue(RED);\n}\n",
    "a {\n  color: 0deg;\n}\n"
);
test!(
    preserves_hex_color_case,
    "a {\n  color: #FfFfFf;\n}\n",
    "a {\n  color: #FfFfFf;\n}\n"
);
test!(
    preserves_hex_8_val_10000000,
    "a {\n  color: #10000000;\n}\n",
    "a {\n  color: #10000000;\n}\n"
);
test!(
    preserves_hex_8_val_12312312,
    "a {\n  color: #12312312;\n}\n",
    "a {\n  color: #12312312;\n}\n"
);
test!(
    preserves_hex_8_val_ab234cff,
    "a {\n  color: #ab234cff;\n}\n",
    "a {\n  color: #ab234cff;\n}\n"
);
test!(
    preserves_hex_6_val_000000,
    "a {\n  color: #000000;\n}\n",
    "a {\n  color: #000000;\n}\n"
);
test!(
    preserves_hex_6_val_123123,
    "a {\n  color: #123123;\n}\n",
    "a {\n  color: #123123;\n}\n"
);
test!(
    preserves_hex_6_val_ab234c,
    "a {\n  color: #ab234c;\n}\n",
    "a {\n  color: #ab234c;\n}\n"
);
test!(
    preserves_hex_4_val_0000,
    "a {\n  color: #0000;\n}\n",
    "a {\n  color: #0000;\n}\n"
);
test!(
    preserves_hex_4_val_123a,
    "a {\n  color: #123a;\n}\n",
    "a {\n  color: #123a;\n}\n"
);
test!(
    preserves_hex_4_val_ab2f,
    "a {\n  color: #ab2f;\n}\n",
    "a {\n  color: #ab2f;\n}\n"
);
test!(
    preserves_hex_3_val_000,
    "a {\n  color: #000;\n}\n",
    "a {\n  color: #000;\n}\n"
);
test!(
    preserves_hex_3_val_123,
    "a {\n  color: #123;\n}\n",
    "a {\n  color: #123;\n}\n"
);
test!(
    preserves_hex_3_val_ab2,
    "a {\n  color: #ab2;\n}\n",
    "a {\n  color: #ab2;\n}\n"
);
test!(
    converts_rgb_to_named_color,
    "a {\n  color: rgb(0, 0, 0);\n}\n",
    "a {\n  color: black;\n}\n"
);
test!(
    converts_rgba_to_named_color_red,
    "a {\n  color: rgb(255, 0, 0, 255);\n}\n",
    "a {\n  color: red;\n}\n"
);
test!(
    rgb_negative,
    "a {\n  color: rgb(-1, 1, 1);\n}\n",
    "a {\n  color: #000101;\n}\n"
);
test!(
    rgb_binop,
    "a {\n  color: rgb(1, 2, 1+2);\n}\n",
    "a {\n  color: #010203;\n}\n"
);
test!(
    rgb_pads_0,
    "a {\n  color: rgb(1, 2, 3);\n}\n",
    "a {\n  color: #010203;\n}\n"
);
test!(
    rgba_percent,
    "a {\n  color: rgba(159%, 169, 169%, 50%);\n}\n",
    "a {\n  color: rgba(255, 169, 255, 0.5);\n}\n"
);
test!(
    rgba_percent_round_up,
    "a {\n  color: rgba(59%, 169, 69%, 50%);\n}\n",
    "a {\n  color: rgba(150, 169, 176, 0.5);\n}\n"
);
test!(
    rgb_double_digits,
    "a {\n  color: rgb(254, 255, 255);\n}\n",
    "a {\n  color: #feffff;\n}\n"
);
test!(
    rgb_double_digits_white,
    "a {\n  color: rgb(255, 255, 255);\n}\n",
    "a {\n  color: white;\n}\n"
);
test!(
    alpha_function_4_hex,
    "a {\n  color: alpha(#0123);\n}\n",
    "a {\n  color: 0.2;\n}\n"
);
test!(
    alpha_function_named_color,
    "a {\n  color: alpha(red);\n}\n",
    "a {\n  color: 1;\n}\n"
);
test!(
    opacity_function_number,
    "a {\n  color: opacity(1);\n}\n",
    "a {\n  color: opacity(1);\n}\n"
);
test!(
    opacity_function_number_unit,
    "a {\n  color: opacity(1px);\n}\n",
    "a {\n  color: opacity(1px);\n}\n"
);
test!(
    rgba_one_arg,
    "a {\n  color: rgba(1 2 3);\n}\n",
    "a {\n  color: #010203;\n}\n"
);
test!(
    rgb_two_args,
    "a {\n  color: rgb(#123, 0);\n}\n",
    "a {\n  color: rgba(17, 34, 51, 0);\n}\n"
);
test!(
    rgba_two_args,
    "a {\n  color: rgba(red, 0.5);\n}\n",
    "a {\n  color: rgba(255, 0, 0, 0.5);\n}\n"
);
test!(
    rgba_opacity_over_1,
    "a {\n  color: rgba(1, 2, 3, 3);\n}\n",
    "a {\n  color: #010203;\n}\n"
);
test!(
    rgba_negative_alpha,
    "a {\n  color: rgba(1, 2, 3, -10%);\n}\n",
    "a {\n  color: rgba(1, 2, 3, 0);\n}\n"
);
test!(
    rgba_opacity_decimal,
    "a {\n  color: rgba(1, 2, 3, .6);\n}\n",
    "a {\n  color: rgba(1, 2, 3, 0.6);\n}\n"
);
test!(
    rgba_opacity_percent,
    "a {\n  color: rgba(1, 2, 3, 50%);\n}\n",
    "a {\n  color: rgba(1, 2, 3, 0.5);\n}\n"
);
test!(
    rgba_3_args,
    "a {\n  color: rgba(7.1%, 20.4%, 33.9%);\n}\n",
    "a {\n  color: #123456;\n}\n"
);
error!(
    rgb_no_args,
    "a {\n  color: rgb();\n}\n", "Error: Missing argument $channels."
);
error!(
    rgba_no_args,
    "a {\n  color: rgba();\n}\n", "Error: Missing argument $channels."
);
test!(
    invert_no_weight,
    "a {\n  color: invert(white);\n}\n",
    "a {\n  color: black;\n}\n"
);
test!(
    plain_invert_no_unit,
    "a {\n  color: invert(1);\n}\n",
    "a {\n  color: invert(1);\n}\n"
);
test!(
    plain_invert_unit_percent,
    "a {\n  color: invert(10%);\n}\n",
    "a {\n  color: invert(10%);\n}\n"
);
test!(
    plain_invert_unit_deg,
    "a {\n  color: invert(1deg);\n}\n",
    "a {\n  color: invert(1deg);\n}\n"
);
test!(
    plain_invert_negative,
    "a {\n  color: invert(-1);\n}\n",
    "a {\n  color: invert(-1);\n}\n"
);
test!(
    plain_invert_float,
    "a {\n  color: invert(1.5);\n}\n",
    "a {\n  color: invert(1.5);\n}\n"
);
test!(
    plain_invert_arithmetic,
    "a {\n  color: invert(1 + 1);\n}\n",
    "a {\n  color: invert(2);\n}\n"
);
test!(
    plain_invert_nan,
    "a {\n  color: invert((0 / 0));\n}\n",
    "a {\n  color: invert(NaN);\n}\n"
);
error!(
    plain_invert_two_args,
    "a {\n  color: invert(1, 50%);\n}\n",
    "Error: Only one argument may be passed to the plain-CSS invert() function."
);
test!(
    invert_weight_percent,
    "a {\n  color: invert(white, 20%);\n}\n",
    "a {\n  color: #cccccc;\n}\n"
);
test!(
    invert_weight_percent_turquoise,
    "a {\n  color: invert(turquoise, 23%);\n}\n",
    "a {\n  color: #5db4ab;\n}\n"
);
test!(
    invert_weight_no_unit,
    "a {\n  color: invert(white, 20);\n}\n",
    "a {\n  color: #cccccc;\n}\n"
);

test!(
    transparentize,
    "a {\n  color: transparentize(rgba(0, 0, 0, 0.5), 0.1);\n}\n",
    "a {\n  color: rgba(0, 0, 0, 0.4);\n}\n"
);
test!(
    fade_out,
    "a {\n  color: fade-out(rgba(0, 0, 0, 0.8), 0.2);\n}\n",
    "a {\n  color: rgba(0, 0, 0, 0.6);\n}\n"
);
test!(
    opacify,
    "a {\n  color: opacify(rgba(0, 0, 0, 0.5), 0.1);\n}\n",
    "a {\n  color: rgba(0, 0, 0, 0.6);\n}\n"
);
test!(
    fade_in,
    "a {\n  color: opacify(rgba(0, 0, 17, 0.8), 0.2);\n}\n",
    "a {\n  color: #000011;\n}\n"
);
test!(
    grayscale_1,
    "a {\n  color: grayscale(plum);\n}\n",
    "a {\n  color: #bfbfbf;\n}\n"
);
test!(
    grayscale_2,
    "a {\n  color: grayscale(red);\n}\n",
    "a {\n  color: gray;\n}\n"
);
test!(
    grayscale_number,
    "a {\n  color: grayscale(15%);\n}\n",
    "a {\n  color: grayscale(15%);\n}\n"
);
test!(
    complement,
    "a {\n  color: complement(red);\n}\n",
    "a {\n  color: aqua;\n}\n"
);
test!(
    complement_hue_under_180,
    "a {\n  color: complement(#abcdef);\n}\n",
    "a {\n  color: #efcdab;\n}\n"
);
test!(
    mix_no_weight,
    "a {\n  color: mix(#f00, #00f);\n}\n",
    "a {\n  color: purple;\n}\n"
);
test!(
    mix_weight_25,
    "a {\n  color: mix(#f00, #00f, 25%);\n}\n",
    "a {\n  color: #4000bf;\n}\n"
);
test!(
    mix_opacity,
    "a {\n  color: mix(rgba(255, 0, 0, 0.5), #00f);\n}\n",
    "a {\n  color: rgba(64, 0, 191, 0.75);\n}\n"
);
test!(
    mix_sanity_check,
    "a {\n  color: mix(black, white);\n}\n",
    "a {\n  color: gray;\n}\n"
);
test!(
    change_color_blue,
    "a {\n  color: change-color(#102030, $blue: 5);\n}\n",
    "a {\n  color: #102005;\n}\n"
);
test!(
    change_color_red_blue,
    "a {\n  color: change-color(#102030, $red: 120, $blue: 5);\n}\n",
    "a {\n  color: #782005;\n}\n"
);
test!(
    change_color_lum_alpha,
    "a {\n  color: change-color(hsl(25, 100%, 80%), $lightness: 40%, $alpha: 0.8);\n}\n",
    "a {\n  color: rgba(204, 85, 0, 0.8);\n}\n"
);
test!(
    adjust_color_blue,
    "a {\n  color: adjust-color(#102030, $blue: 5);\n}\n",
    "a {\n  color: #102035;\n}\n"
);
test!(
    adjust_color_negative,
    "a {\n  color: adjust-color(#102030, $red: -5, $blue: 5);\n}\n",
    "a {\n  color: #0b2035;\n}\n"
);
test!(
    adjust_color_lum_alpha,
    "a {\n  color: adjust-color(hsl(25, 100%, 80%), $lightness: -30%, $alpha: -0.4);\n}\n",
    "a {\n  color: rgba(255, 106, 0, 0.6);\n}\n"
);
test!(
    scale_color_lightness,
    "a {\n  color: scale-color(hsl(120, 70%, 80%), $lightness: 50%);\n}\n",
    "a {\n  color: #d4f7d4;\n}\n"
);
test!(
    scale_color_negative,
    "a {\n  color: scale-color(rgb(200, 150%, 170%), $green: -40%, $blue: 70%);\n}\n",
    "a {\n  color: #c899ff;\n}\n"
);
test!(
    scale_color_alpha,
    "a {\n  color: scale-color(hsl(200, 70%, 80%), $saturation: -90%, $alpha: -30%);\n}\n",
    "a {\n  color: rgba(200, 205, 208, 0.7);\n}\n"
);
test!(
    scale_color_alpha_over_1,
    "a {\n  color: scale-color(sienna, $alpha: -70%);\n}\n",
    "a {\n  color: rgba(160, 82, 45, 0.3);\n}\n"
);
test!(
    ie_hex_str_hex_3,
    "a {\n  color: ie-hex-str(#abc);\n}\n",
    "a {\n  color: #FFAABBCC;\n}\n"
);
test!(
    ie_hex_str_hex_6,
    "a {\n  color: ie-hex-str(#3322BB);\n}\n",
    "a {\n  color: #FF3322BB;\n}\n"
);
test!(
    ie_hex_str_rgb,
    "a {\n  color: ie-hex-str(rgba(0, 255, 0, 0.5));\n}\n",
    "a {\n  color: #8000FF00;\n}\n"
);
test!(
    rgba_1_arg,
    "a {\n  color: rgba(74.7% 173 93%);\n}\n",
    "a {\n  color: #beaded;\n}\n"
);
test!(
    hsla_1_arg,
    "a {\n  color: hsla(60 60% 50%);\n}\n",
    "a {\n  color: #cccc33;\n}\n"
);
test!(
    hsla_1_arg_weird_units,
    "a {\n  color: hsla(60foo 60foo 50foo);\n}\n",
    "a {\n  color: #cccc33;\n}\n"
);
test!(
    sass_spec__spec_colors_basic,
    "p {
  color: rgb(255, 128, 0);
  color: red green blue;
  color: (red) (green) (blue);
  color: red + hux;
  color: unquote(\"red\") + green;
  foo: rgb(200, 150%, 170%);
}
",
    "p {\n  color: #ff8000;\n  color: red green blue;\n  color: red green blue;\n  color: redhux;\n  color: redgreen;\n  foo: #c8ffff;\n}\n"
);
test!(
    sass_spec__spec_colors_change_color,
    "p {
  color: change-color(#102030, $blue: 5);
  color: change-color(#102030, $alpha: .325);
  color: change-color(#102030, $red: 120, $blue: 5);
  color: change-color(hsl(25, 100%, 80%), $lightness: 40%, $alpha: 0.8);
}
",
    "p {\n  color: #102005;\n  color: rgba(16, 32, 48, 0.325);\n  color: #782005;\n  color: rgba(204, 85, 0, 0.8);\n}\n"
);
test!(
    transparent_from_function,
    "a {\n  color: rgb(transparent, 0);\n}\n",
    "a {\n  color: rgba(0, 0, 0, 0);\n}\n"
);
test!(
    named_color_transparent_opacity,
    "a {\n  color: opacity(transparent);\n}\n",
    "a {\n  color: 0;\n}\n"
);
test!(
    negative_values_in_rgb,
    "a {\n  color: rgb(-1 -1 -1);\n}\n",
    "a {\n  color: black;\n}\n"
);
test!(
    interpolation_after_hash_containing_only_hex_chars,
    "a {\n  color: ##{123};\n  color: type-of(##{123});\n}\n",
    "a {\n  color: #123;\n  color: string;\n}\n"
);
test!(
    non_hex_chars_after_hash_are_still_touching_hash,
    "a {\n  color: #ooobar;\n}\n",
    "a {\n  color: #ooobar;\n}\n"
);
test!(
    more_than_8_hex_chars_after_hash_starts_with_letter,
    "a {\n  color: #ffffffffff;\n}\n",
    "a {\n  color: #ffffffffff;\n}\n"
);
test!(
    more_than_8_hex_chars_after_hash_starts_with_number,
    "a {\n  color: #0000000000;\n}\n",
    "a {\n  color: #00000000 0;\n}\n"
);
test!(
    more_than_8_hex_chars_after_hash_starts_with_number_contains_hex_char,
    "a {\n  color: #00000000f00;\n}\n",
    "a {\n  color: #00000000 f00;\n}\n"
);
test!(
    all_three_rgb_channels_have_decimal,
    "a {\n  color: rgba(1.5, 1.5, 1.5, 1);\n}\n",
    "a {\n  color: #020202;\n}\n"
);
test!(
    builtin_fn_red_rounds_channel,
    "a {\n  color: red(rgba(1.5, 1.5, 1.5, 1));\n}\n",
    "a {\n  color: 2;\n}\n"
);
test!(
    builtin_fn_green_rounds_channel,
    "a {\n  color: green(rgba(1.5, 1.5, 1.5, 1));\n}\n",
    "a {\n  color: 2;\n}\n"
);
test!(
    builtin_fn_blue_rounds_channel,
    "a {\n  color: blue(rgba(1.5, 1.5, 1.5, 1));\n}\n",
    "a {\n  color: 2;\n}\n"
);
test!(
    color_equality_named_and_hex,
    "a {\n  color: red==#ff0000;\n}\n",
    "a {\n  color: true;\n}\n"
);
test!(
    color_equality_named_and_hsla,
    "a {\n  color: hsla(0deg, 100%, 50%)==red;\n}\n",
    "a {\n  color: true;\n}\n"
);
test!(
    alpha_filter_one_arg,
    "a {\n  color: alpha(a=a);\n}\n",
    "a {\n  color: alpha(a=a);\n}\n"
);
test!(
    alpha_filter_multiple_args,
    "a {\n  color: alpha(a=a, b=b, c=d, d=d);\n}\n",
    "a {\n  color: alpha(a=a, b=b, c=d, d=d);\n}\n"
);
test!(
    alpha_filter_whitespace,
    "a {\n  color: alpha(a   =    a);\n}\n",
    "a {\n  color: alpha(a=a);\n}\n"
);
test!(
    alpha_filter_named,
    "a {\n  color: alpha($color: a=a);\n}\n",
    "a {\n  color: alpha(a=a);\n}\n"
);
error!(
    alpha_filter_both_null,
    "a {\n  color: alpha(null=null);\n}\n", "Error: $color: = is not a color."
);
error!(
    alpha_filter_multiple_args_one_not_valid_filter,
    "a {\n  color: alpha(a=a, b);\n}\n", "Error: Only 1 argument allowed, but 2 were passed."
);
error!(
    alpha_filter_invalid_from_whitespace,
    "a {\n  color: alpha( A a   =    a  );\n}\n", "Error: $color: A a=a is not a color."
);
error!(
    alpha_filter_invalid_non_alphabetic_start,
    "a {\n  color: alpha(1=a);\n}\n", "Error: $color: 1=a is not a color."
);
// todo: we need many more of these tests
test!(
    rgba_special_fn_4th_arg_max,
    "a {\n  color: rgba(1 2 max(3, 3));\n}\n",
    "a {\n  color: rgba(1, 2, max(3, 3));\n}\n"
);
test!(
    rgb_special_fn_4_arg_maintains_units,
    "a {\n  color: rgb(1, 0.02, 3%, max(0.4));\n}\n",
    "a {\n  color: rgb(1, 0.02, 3%, max(0.4));\n}\n"
);
test!(
    rgb_special_fn_3_arg_maintains_units,
    "a {\n  color: rgb(1, 0.02, max(0.4));\n}\n",
    "a {\n  color: rgb(1, 0.02, max(0.4));\n}\n"
);
test!(
    rgb_special_fn_2_arg_first_non_color,
    "a {\n  color: rgb(1, var(--foo));\n}\n",
    "a {\n  color: rgb(1, var(--foo));\n}\n"
);
test!(
    rgb_special_fn_2_arg_first_is_color,
    "a {\n  color: rgb(rgb(1%, 1, 1), var(--foo));;\n}\n",
    "a {\n  color: rgb(3, 1, 1, var(--foo));\n}\n"
);
test!(
    #[ignore = "we do not check if interpolation occurred"]
    interpolated_named_color_is_not_color,
    "a {\n  color: type-of(r#{e}d);\n}\n",
    "a {\n  color: string;\n}\n"
);
test!(
    color_equality_differ_in_green_channel,
    "a {\n  color: rgb(1, 1, 1) == rgb(1, 2, 1);\n}\n",
    "a {\n  color: false;\n}\n"
);
test!(
    color_equality_differ_in_blue_channel,
    "a {\n  color: rgb(1, 1, 1) == rgb(1, 1, 2);\n}\n",
    "a {\n  color: false;\n}\n"
);
test!(
    color_equality_differ_in_alpha_channel,
    "a {\n  color: rgba(1, 1, 1, 1.0) == rgba(1, 1, 1, 0.5);\n}\n",
    "a {\n  color: false;\n}\n"
);
test!(
    invert_weight_zero_is_nop,
    "a {\n  color: invert(#0f0f0f, 0);\n}\n",
    "a {\n  color: #0f0f0f;\n}\n"
);
test!(
    mix_combined_weight_is_normalized_weight,
    "a {\n  color: mix(rgba(255, 20, 0, 0), rgba(0, 20, 255, 1), 100);\n}\n",
    "a {\n  color: rgba(255, 20, 0, 0);\n}\n"
);
test!(
    hue_largest_channel_is_blue,
    "a {\n  color: hue(rgb(1, 2, 5));\n}\n",
    "a {\n  color: 225deg;\n}\n"
);
