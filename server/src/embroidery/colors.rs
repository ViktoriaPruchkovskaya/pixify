use image::Rgb;
use lab::Lab;
use serde::{ser::SerializeSeq, Serialize, Serializer};
use std::f32::consts::PI;

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub struct RgbColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Serialize for RgbColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_seq(Some(3))?;
        state.serialize_element(&self.red)?;
        state.serialize_element(&self.green)?;
        state.serialize_element(&self.blue)?;
        state.end()
    }
}

impl From<Rgb<u8>> for RgbColor {
    fn from(value: Rgb<u8>) -> Self {
        RgbColor {
            red: value[0],
            green: value[1],
            blue: value[2],
        }
    }
}

impl From<RgbColor> for Rgb<u8> {
    fn from(value: RgbColor) -> Self {
        Rgb([value.red, value.green, value.blue])
    }
}

impl From<RgbColor> for [u8; 3] {
    fn from(val: RgbColor) -> Self {
        [val.red, val.green, val.blue]
    }
}

pub struct DmcColor {
    pub name: &'static str,
    pub rgb: RgbColor,
}

impl RgbColor {
    pub fn find_dmc(&self) -> DmcColor {
        let mut color_diffs: Vec<(i32, DmcColor)> = Vec::with_capacity(RGB_TO_DMC.len());
        let lab = Lab::from_rgb(&(*self).into());
        for &(rgb, lab_2, name) in RGB_TO_DMC.iter() {
            let dmc_color = DmcColor { name, rgb };
            if rgb == *self {
                return dmc_color;
            }
            let diff = Self::calculate_diff(lab, lab_2) as i32;

            color_diffs.push((diff, dmc_color));
        }

        let (.., closest_dmc_color) = color_diffs.iter().min_by_key(|x| x.0).unwrap();
        DmcColor {
            name: closest_dmc_color.name,
            rgb: closest_dmc_color.rgb,
        }
    }

    pub fn calculate_diff(lab: Lab, lab_2: Lab) -> f32 {
        let c1 = (lab.a.powi(2) + lab.b.powi(2)).sqrt();
        let c2 = (lab_2.a.powi(2) + lab_2.b.powi(2)).sqrt();
        let c_bar = (c1 + c2) / 2.0;

        let delta_l_prime = lab_2.l - lab.l;
        let l_bar = (lab.l + lab_2.l) / 2.0;
        let a_prime_1 = Self::get_a_prime(lab.a, c_bar);
        let a_prime_2 = Self::get_a_prime(lab_2.a, c_bar);
        let c_prime_1 = (a_prime_1.powf(2.0) + lab.b.powf(2.0)).sqrt();
        let c_prime_2 = (a_prime_2.powf(2.0) + lab_2.b.powf(2.0)).sqrt();
        let c_bar_prime = (c_prime_1 + c_prime_2) / 2.0;
        let delta_c_prime = c_prime_2 - c_prime_1;
        let s_sub_l =
            1.0 + (0.015 * (l_bar - 50.0).powf(2.0)) / (20.0 + (l_bar - 50.0).powf(2.0)).sqrt();
        let s_sub_c = 1.0 + 0.045 * c_bar_prime;

        let h_prime_1 = Self::get_h_prime_fn(lab.b, a_prime_1);
        let h_prime_2 = Self::get_h_prime_fn(lab_2.b, a_prime_2);

        let delta_h_prime = Self::get_delta_h_prime(c1, c2, h_prime_1, h_prime_2);

        let delta_uppercase_h_prime = 2.0
            * (c_prime_1 * c_prime_2).sqrt()
            * (Self::degrees_to_radians(delta_h_prime) / 2.0).sin();

        let uppercase_h_bar_prime = Self::get_uppercase_h_bar_prime(h_prime_1, h_prime_2);

        let uppercase_t = Self::get_uppercase_t(uppercase_h_bar_prime);

        let s_sub_uppercase_h = 1.0 + 0.015 * c_bar_prime * uppercase_t;

        let r_sub_t = Self::get_r_sub_t(c_bar_prime, uppercase_h_bar_prime);

        let lightness: f32 = delta_l_prime / (1.0 * s_sub_l);

        let chroma: f32 = delta_c_prime / (1.0 * s_sub_c);

        let hue: f32 = delta_uppercase_h_prime / (1.0 * s_sub_uppercase_h);

        (lightness.powi(2) + chroma.powi(2) + hue.powi(2) + r_sub_t * chroma * hue).sqrt()
    }

    fn get_h_prime_fn(x: f32, y: f32) -> f32 {
        if x == 0.0 && y == 0.0 {
            return 0.0;
        }

        let mut hue_angle = Self::radians_to_degrees(x.atan2(y));
        if hue_angle < 0.0 {
            hue_angle += 360.0;
        }
        hue_angle
    }

    fn get_a_prime(a: f32, c_bar: f32) -> f32 {
        a + a / 2.0 * (1.0 - (c_bar.powi(7) / (c_bar.powi(7) + 25_f32.powi(7))).sqrt())
    }

    fn get_delta_h_prime(c1: f32, c2: f32, h_prime_1: f32, h_prime_2: f32) -> f32 {
        if 0.0 == c1 || 0.0 == c2 {
            return 0.0;
        }

        if (h_prime_1 - h_prime_2).abs() <= 180.0 {
            return h_prime_2 - h_prime_1;
        }

        if h_prime_2 <= h_prime_1 {
            return h_prime_2 - h_prime_1 + 360.0;
        }
        h_prime_2 - h_prime_1 - 360.0
    }

    fn get_uppercase_h_bar_prime(h_prime_1: f32, h_prime_2: f32) -> f32 {
        if (h_prime_1 - h_prime_2).abs() > 180.0 {
            return (h_prime_1 + h_prime_2 + 360.0) / 2.0;
        }

        (h_prime_1 + h_prime_2) / 2.0
    }

    fn get_uppercase_t(uppercase_h_bar_prime: f32) -> f32 {
        1.0 - 0.17 * Self::degrees_to_radians(uppercase_h_bar_prime - 30.0).cos()
            + 0.24 * Self::degrees_to_radians(2.0 * uppercase_h_bar_prime).cos()
            + 0.32 * Self::degrees_to_radians(3.0 * uppercase_h_bar_prime + 6.0).cos()
            - 0.20 * Self::degrees_to_radians(4.0 * uppercase_h_bar_prime - 63.0).cos()
    }

    fn get_r_sub_t(c_bar_prime: f32, uppercase_h_bar_prime: f32) -> f32 {
        -2.0 * (c_bar_prime.powi(7) / (c_bar_prime.powi(7) + 25f32.powi(7))).sqrt()
            * Self::degrees_to_radians(
                60.0 * (-((uppercase_h_bar_prime - 275.0) / 25.0).powi(2)).exp(),
            )
            .sin()
    }

    fn radians_to_degrees(radians: f32) -> f32 {
        radians * (180.0 / PI)
    }

    fn degrees_to_radians(degrees: f32) -> f32 {
        degrees * (PI / 180.0)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_gets_dmc_color() {
        let color = super::RgbColor {
            red: 255,
            green: 29,
            blue: 30,
        };
        let super::DmcColor { rgb, .. } = color.find_dmc();
        assert_eq!(rgb.red, 204);
        assert_eq!(rgb.green, 63);
        assert_eq!(rgb.blue, 24);
    }

    #[test]
    fn it_gets_existing_color() {
        let color = super::RgbColor {
            red: 255,
            green: 255,
            blue: 255,
        };
        let super::DmcColor { rgb, .. } = color.find_dmc();
        assert_eq!(rgb.red, 255);
        assert_eq!(rgb.green, 255);
        assert_eq!(rgb.blue, 255);
    }
}

pub static RGB_TO_DMC: [(RgbColor, Lab, &str); 487] = [
    (
        RgbColor {
            red: 255,
            green: 255,
            blue: 255,
        },
        Lab {
            l: 100.0,
            a: 0.0,
            b: 0.0,
        },
        "B5200",
    ),
    (
        RgbColor {
            red: 243,
            green: 206,
            blue: 205,
        },
        Lab {
            l: 85.80929,
            a: 12.811243,
            b: 5.332482,
        },
        "3713",
    ),
    (
        RgbColor {
            red: 234,
            green: 184,
            blue: 185,
        },
        Lab {
            l: 79.20105,
            a: 18.15921,
            b: 6.4108014,
        },
        "761",
    ),
    (
        RgbColor {
            red: 210,
            green: 140,
            blue: 142,
        },
        Lab {
            l: 65.28881,
            a: 26.912748,
            b: 9.899664,
        },
        "760",
    ),
    (
        RgbColor {
            red: 189,
            green: 104,
            blue: 104,
        },
        Lab {
            l: 53.729446,
            a: 33.85103,
            b: 15.149439,
        },
        "3712",
    ),
    (
        RgbColor {
            red: 167,
            green: 74,
            blue: 74,
        },
        Lab {
            l: 43.66693,
            a: 38.32251,
            b: 18.860859,
        },
        "3328",
    ),
    (
        RgbColor {
            red: 146,
            green: 32,
            blue: 42,
        },
        Lab {
            l: 32.508022,
            a: 47.127308,
            b: 23.71466,
        },
        "347",
    ),
    (
        RgbColor {
            red: 242,
            green: 193,
            blue: 176,
        },
        Lab {
            l: 81.92165,
            a: 15.079737,
            b: 15.221989,
        },
        "353",
    ),
    (
        RgbColor {
            red: 217,
            green: 134,
            blue: 118,
        },
        Lab {
            l: 64.25171,
            a: 30.060232,
            b: 22.084427,
        },
        "352",
    ),
    (
        RgbColor {
            red: 181,
            green: 73,
            blue: 61,
        },
        Lab {
            l: 45.53508,
            a: 43.06185,
            b: 29.475414,
        },
        "351",
    ),
    (
        RgbColor {
            red: 166,
            green: 50,
            blue: 42,
        },
        Lab {
            l: 38.824272,
            a: 46.96721,
            b: 32.05775,
        },
        "350",
    ),
    (
        RgbColor {
            red: 149,
            green: 10,
            blue: 30,
        },
        Lab {
            l: 31.118248,
            a: 52.775993,
            b: 29.81348,
        },
        "349",
    ),
    (
        RgbColor {
            red: 143,
            green: 1,
            blue: 21,
        },
        Lab {
            l: 29.20166,
            a: 52.281185,
            b: 32.84972,
        },
        "817",
    ),
    (
        RgbColor {
            red: 234,
            green: 155,
            blue: 175,
        },
        Lab {
            l: 72.25022,
            a: 32.162876,
            b: 1.8284678,
        },
        "3708",
    ),
    (
        RgbColor {
            red: 224,
            green: 123,
            blue: 141,
        },
        Lab {
            l: 63.292336,
            a: 40.959507,
            b: 7.863438,
        },
        "3706",
    ),
    (
        RgbColor {
            red: 193,
            green: 68,
            blue: 80,
        },
        Lab {
            l: 47.044197,
            a: 50.772728,
            b: 20.482021,
        },
        "3705",
    ),
    (
        RgbColor {
            red: 182,
            green: 46,
            blue: 59,
        },
        Lab {
            l: 41.558918,
            a: 54.453133,
            b: 25.82187,
        },
        "3801",
    ),
    (
        RgbColor {
            red: 152,
            green: 0,
            blue: 21,
        },
        Lab {
            l: 31.186691,
            a: 54.783596,
            b: 35.4165,
        },
        "666",
    ),
    (
        RgbColor {
            red: 126,
            green: 0,
            blue: 24,
        },
        Lab {
            l: 25.267841,
            a: 48.05504,
            b: 25.778692,
        },
        "321",
    ),
    (
        RgbColor {
            red: 127,
            green: 0,
            blue: 45,
        },
        Lab {
            l: 25.869759,
            a: 49.3184,
            b: 12.4341545,
        },
        "304",
    ),
    (
        RgbColor {
            red: 103,
            green: 0,
            blue: 26,
        },
        Lab {
            l: 19.878117,
            a: 41.977646,
            b: 16.815388,
        },
        "498",
    ),
    (
        RgbColor {
            red: 110,
            green: 0,
            blue: 29,
        },
        Lab {
            l: 21.59589,
            a: 44.004204,
            b: 17.208284,
        },
        "816",
    ),
    (
        RgbColor {
            red: 81,
            green: 0,
            blue: 28,
        },
        Lab {
            l: 14.5883465,
            a: 36.116867,
            b: 7.469982,
        },
        "815",
    ),
    (
        RgbColor {
            red: 70,
            green: 4,
            blue: 27,
        },
        Lab {
            l: 12.405315,
            a: 31.378477,
            b: 4.785392,
        },
        "814",
    ),
    (
        RgbColor {
            red: 232,
            green: 152,
            blue: 173,
        },
        Lab {
            l: 71.304245,
            a: 32.756775,
            b: 1.5334606,
        },
        "894",
    ),
    (
        RgbColor {
            red: 210,
            green: 99,
            blue: 124,
        },
        Lab {
            l: 56.170776,
            a: 46.20099,
            b: 7.2742224,
        },
        "893",
    ),
    (
        RgbColor {
            red: 203,
            green: 78,
            blue: 93,
        },
        Lab {
            l: 50.614723,
            a: 50.744446,
            b: 17.741417,
        },
        "892",
    ),
    (
        RgbColor {
            red: 193,
            green: 57,
            blue: 85,
        },
        Lab {
            l: 45.527077,
            a: 55.56491,
            b: 15.393013,
        },
        "891",
    ),
    (
        RgbColor {
            red: 247,
            green: 212,
            blue: 211,
        },
        Lab {
            l: 87.75909,
            a: 12.027502,
            b: 5.017197,
        },
        "818",
    ),
    (
        RgbColor {
            red: 233,
            green: 153,
            blue: 178,
        },
        Lab {
            l: 71.77342,
            a: 33.28976,
            b: -0.5421877,
        },
        "957",
    ),
    (
        RgbColor {
            red: 211,
            green: 93,
            blue: 134,
        },
        Lab {
            l: 55.483253,
            a: 50.5431,
            b: 0.4221201,
        },
        "956",
    ),
    (
        RgbColor {
            red: 145,
            green: 50,
            blue: 68,
        },
        Lab {
            l: 35.43851,
            a: 41.46594,
            b: 10.840398,
        },
        "309",
    ),
    (
        RgbColor {
            red: 244,
            green: 203,
            blue: 208,
        },
        Lab {
            l: 85.25106,
            a: 15.209616,
            b: 2.9420137,
        },
        "963",
    ),
    (
        RgbColor {
            red: 228,
            green: 170,
            blue: 186,
        },
        Lab {
            l: 75.32067,
            a: 23.617178,
            b: 0.16936064,
        },
        "3716",
    ),
    (
        RgbColor {
            red: 199,
            green: 128,
            blue: 147,
        },
        Lab {
            l: 61.384644,
            a: 30.004025,
            b: 1.280415,
        },
        "962",
    ),
    (
        RgbColor {
            red: 191,
            green: 100,
            blue: 127,
        },
        Lab {
            l: 53.733788,
            a: 39.3779,
            b: 1.6490936,
        },
        "961",
    ),
    (
        RgbColor {
            red: 194,
            green: 111,
            blue: 128,
        },
        Lab {
            l: 56.48459,
            a: 34.7808,
            b: 5.088842,
        },
        "3833",
    ),
    (
        RgbColor {
            red: 177,
            green: 71,
            blue: 93,
        },
        Lab {
            l: 45.14502,
            a: 45.10832,
            b: 9.524685,
        },
        "3832",
    ),
    (
        RgbColor {
            red: 144,
            green: 37,
            blue: 59,
        },
        Lab {
            l: 33.069588,
            a: 45.7972,
            b: 13.387179,
        },
        "3831",
    ),
    (
        RgbColor {
            red: 95,
            green: 2,
            blue: 36,
        },
        Lab {
            l: 18.383526,
            a: 39.883972,
            b: 7.361281,
        },
        "777",
    ),
    (
        RgbColor {
            red: 252,
            green: 230,
            blue: 222,
        },
        Lab {
            l: 92.82322,
            a: 6.1942935,
            b: 6.608641,
        },
        "819",
    ),
    (
        RgbColor {
            red: 227,
            green: 165,
            blue: 175,
        },
        Lab {
            l: 73.76441,
            a: 24.293781,
            b: 3.9290667,
        },
        "3326",
    ),
    (
        RgbColor {
            red: 129,
            green: 74,
            blue: 126,
        },
        Lab {
            l: 39.700684,
            a: 31.708628,
            b: -19.641626,
        },
        "776",
    ),
    (
        RgbColor {
            red: 199,
            green: 113,
            blue: 138,
        },
        Lab {
            l: 57.836403,
            a: 36.834656,
            b: 1.3061523,
        },
        "899",
    ),
    (
        RgbColor {
            red: 183,
            green: 84,
            blue: 112,
        },
        Lab {
            l: 48.982117,
            a: 42.905598,
            b: 3.5683274,
        },
        "335",
    ),
    (
        RgbColor {
            red: 122,
            green: 0,
            blue: 46,
        },
        Lab {
            l: 24.751701,
            a: 48.112274,
            b: 10.039896,
        },
        "326",
    ),
    (
        RgbColor {
            red: 232,
            green: 187,
            blue: 196,
        },
        Lab {
            l: 80.00119,
            a: 17.51718,
            b: 1.604557,
        },
        "151",
    ),
    (
        RgbColor {
            red: 215,
            green: 159,
            blue: 172,
        },
        Lab {
            l: 71.02468,
            a: 22.701056,
            b: 1.4542103,
        },
        "3354",
    ),
    (
        RgbColor {
            red: 193,
            green: 122,
            blue: 141,
        },
        Lab {
            l: 59.141624,
            a: 30.210436,
            b: 1.3761044,
        },
        "3733",
    ),
    (
        RgbColor {
            red: 170,
            green: 78,
            blue: 107,
        },
        Lab {
            l: 45.713314,
            a: 40.78421,
            b: 1.6214132,
        },
        "3731",
    ),
    (
        RgbColor {
            red: 136,
            green: 44,
            blue: 74,
        },
        Lab {
            l: 32.986458,
            a: 41.69038,
            b: 3.271079,
        },
        "3350",
    ),
    (
        RgbColor {
            red: 117,
            green: 19,
            blue: 51,
        },
        Lab {
            l: 25.221615,
            a: 42.955116,
            b: 7.074189,
        },
        "150",
    ),
    (
        RgbColor {
            red: 231,
            green: 191,
            blue: 205,
        },
        Lab {
            l: 81.10408,
            a: 16.482979,
            b: -1.6441226,
        },
        "3689",
    ),
    (
        RgbColor {
            red: 186,
            green: 129,
            blue: 152,
        },
        Lab {
            l: 60.255363,
            a: 25.436283,
            b: -3.414619,
        },
        "3688",
    ),
    (
        RgbColor {
            red: 162,
            green: 90,
            blue: 118,
        },
        Lab {
            l: 47.369335,
            a: 33.041508,
            b: -2.7653456,
        },
        "3687",
    ),
    (
        RgbColor {
            red: 108,
            green: 42,
            blue: 71,
        },
        Lab {
            l: 27.571705,
            a: 32.671436,
            b: -3.2566667,
        },
        "3803",
    ),
    (
        RgbColor {
            red: 83,
            green: 13,
            blue: 43,
        },
        Lab {
            l: 16.996674,
            a: 33.80005,
            b: -0.0364542,
        },
        "3685",
    ),
    (
        RgbColor {
            red: 233,
            green: 189,
            blue: 206,
        },
        Lab {
            l: 80.83675,
            a: 18.414974,
            b: -2.5526881,
        },
        "605",
    ),
    (
        RgbColor {
            red: 226,
            green: 166,
            blue: 190,
        },
        Lab {
            l: 74.29233,
            a: 25.673777,
            b: -3.5547614,
        },
        "604",
    ),
    (
        RgbColor {
            red: 210,
            green: 132,
            blue: 165,
        },
        Lab {
            l: 64.098145,
            a: 34.43733,
            b: -4.8434973,
        },
        "603",
    ),
    (
        RgbColor {
            red: 187,
            green: 84,
            blue: 129,
        },
        Lab {
            l: 50.087723,
            a: 46.344967,
            b: -4.957795,
        },
        "602",
    ),
    (
        RgbColor {
            red: 165,
            green: 48,
            blue: 97,
        },
        Lab {
            l: 39.543137,
            a: 51.561832,
            b: -1.2819827,
        },
        "601",
    ),
    (
        RgbColor {
            red: 147,
            green: 25,
            blue: 80,
        },
        Lab {
            l: 32.872856,
            a: 52.3165,
            b: -0.5489111,
        },
        "600",
    ),
    (
        RgbColor {
            red: 212,
            green: 140,
            blue: 172,
        },
        Lab {
            l: 66.31575,
            a: 31.907648,
            b: -5.493486,
        },
        "3806",
    ),
    (
        RgbColor {
            red: 176,
            green: 73,
            blue: 126,
        },
        Lab {
            l: 46.31571,
            a: 47.659157,
            b: -8.950925,
        },
        "3805",
    ),
    (
        RgbColor {
            red: 167,
            green: 57,
            blue: 112,
        },
        Lab {
            l: 41.709442,
            a: 50.245674,
            b: -7.4136496,
        },
        "3804",
    ),
    (
        RgbColor {
            red: 221,
            green: 181,
            blue: 210,
        },
        Lab {
            l: 77.91866,
            a: 19.212812,
            b: -9.151686,
        },
        "3609",
    ),
    (
        RgbColor {
            red: 201,
            green: 148,
            blue: 183,
        },
        Lab {
            l: 67.29001,
            a: 25.491983,
            b: -10.345149,
        },
        "3608",
    ),
    (
        RgbColor {
            red: 156,
            green: 76,
            blue: 141,
        },
        Lab {
            l: 44.462677,
            a: 42.144657,
            b: -21.118773,
        },
        "3607",
    ),
    (
        RgbColor {
            red: 132,
            green: 27,
            blue: 97,
        },
        Lab {
            l: 30.767673,
            a: 49.701885,
            b: -15.086163,
        },
        "718",
    ),
    (
        RgbColor {
            red: 137,
            green: 40,
            blue: 103,
        },
        Lab {
            l: 33.597,
            a: 47.247112,
            b: -14.525914,
        },
        "917",
    ),
    (
        RgbColor {
            red: 86,
            green: 0,
            blue: 43,
        },
        Lab {
            l: 16.27185,
            a: 38.697006,
            b: -1.0181844,
        },
        "915",
    ),
    (
        RgbColor {
            red: 238,
            green: 207,
            blue: 201,
        },
        Lab {
            l: 85.51457,
            a: 9.878606,
            b: 6.973219,
        },
        "225",
    ),
    (
        RgbColor {
            red: 211,
            green: 169,
            blue: 163,
        },
        Lab {
            l: 72.80652,
            a: 14.449328,
            b: 8.934128,
        },
        "224",
    ),
    (
        RgbColor {
            red: 195,
            green: 149,
            blue: 148,
        },
        Lab {
            l: 65.93225,
            a: 17.108381,
            b: 7.219672,
        },
        "152",
    ),
    (
        RgbColor {
            red: 153,
            green: 94,
            blue: 101,
        },
        Lab {
            l: 46.602695,
            a: 24.998873,
            b: 6.234467,
        },
        "223",
    ),
    (
        RgbColor {
            red: 140,
            green: 78,
            blue: 82,
        },
        Lab {
            l: 40.628933,
            a: 26.487171,
            b: 9.138405,
        },
        "3722",
    ),
    (
        RgbColor {
            red: 121,
            green: 53,
            blue: 55,
        },
        Lab {
            l: 31.55,
            a: 29.88404,
            b: 13.223314,
        },
        "3721",
    ),
    (
        RgbColor {
            red: 99,
            green: 28,
            blue: 33,
        },
        Lab {
            l: 22.279339,
            a: 32.253014,
            b: 14.8164215,
        },
        "221",
    ),
    (
        RgbColor {
            red: 204,
            green: 168,
            blue: 170,
        },
        Lab {
            l: 72.05836,
            a: 13.454348,
            b: 3.9224148,
        },
        "778",
    ),
    (
        RgbColor {
            red: 196,
            green: 158,
            blue: 169,
        },
        Lab {
            l: 68.79108,
            a: 15.819311,
            b: -0.3682375,
        },
        "3727",
    ),
    (
        RgbColor {
            red: 172,
            green: 131,
            blue: 150,
        },
        Lab {
            l: 59.16455,
            a: 18.82854,
            b: -4.0475726,
        },
        "316",
    ),
    (
        RgbColor {
            red: 135,
            green: 86,
            blue: 94,
        },
        Lab {
            l: 42.177826,
            a: 21.459908,
            b: 3.849113,
        },
        "3726",
    ),
    (
        RgbColor {
            red: 106,
            green: 56,
            blue: 65,
        },
        Lab {
            l: 30.095104,
            a: 23.250938,
            b: 4.337496,
        },
        "315",
    ),
    (
        RgbColor {
            red: 85,
            green: 35,
            blue: 49,
        },
        Lab {
            l: 21.414635,
            a: 24.842083,
            b: 2.1852076,
        },
        "3802",
    ),
    (
        RgbColor {
            red: 94,
            green: 13,
            blue: 34,
        },
        Lab {
            l: 19.15525,
            a: 36.42474,
            b: 9.810466,
        },
        "902",
    ),
    (
        RgbColor {
            red: 195,
            green: 188,
            blue: 196,
        },
        Lab {
            l: 77.0135,
            a: 3.9171875,
            b: -3.1500697,
        },
        "3743",
    ),
    (
        RgbColor {
            red: 163,
            green: 153,
            blue: 165,
        },
        Lab {
            l: 64.3916,
            a: 5.9180856,
            b: -4.944694,
        },
        "3042",
    ),
    (
        RgbColor {
            red: 114,
            green: 92,
            blue: 106,
        },
        Lab {
            l: 41.646038,
            a: 11.547536,
            b: -4.5825124,
        },
        "3041",
    ),
    (
        RgbColor {
            red: 85,
            green: 60,
            blue: 72,
        },
        Lab {
            l: 28.461952,
            a: 13.155341,
            b: -2.9723465,
        },
        "3740",
    ),
    (
        RgbColor {
            red: 163,
            green: 138,
            blue: 164,
        },
        Lab {
            l: 60.466286,
            a: 14.179527,
            b: -10.217333,
        },
        "3836",
    ),
    (
        RgbColor {
            red: 119,
            green: 87,
            blue: 121,
        },
        Lab {
            l: 41.404736,
            a: 19.525797,
            b: -14.072943,
        },
        "3835",
    ),
    (
        RgbColor {
            red: 74,
            green: 39,
            blue: 72,
        },
        Lab {
            l: 21.486862,
            a: 22.189722,
            b: -13.663017,
        },
        "3834",
    ),
    (
        RgbColor {
            red: 37,
            green: 3,
            blue: 31,
        },
        Lab {
            l: 5.0352592,
            a: 20.433798,
            b: -10.00801,
        },
        "154",
    ),
    (
        RgbColor {
            red: 204,
            green: 194,
            blue: 222,
        },
        Lab {
            l: 80.007645,
            a: 8.746206,
            b: -12.666178,
        },
        "211",
    ),
    (
        RgbColor {
            red: 178,
            green: 165,
            blue: 205,
        },
        Lab {
            l: 70.023575,
            a: 12.547135,
            b: -18.658495,
        },
        "210",
    ),
    (
        RgbColor {
            red: 149,
            green: 131,
            blue: 180,
        },
        Lab {
            l: 57.95854,
            a: 16.840189,
            b: -23.269701,
        },
        "209",
    ),
    (
        RgbColor {
            red: 111,
            green: 84,
            blue: 151,
        },
        Lab {
            l: 41.14365,
            a: 25.745571,
            b: -32.726933,
        },
        "208",
    ),
    (
        RgbColor {
            red: 84,
            green: 58,
            blue: 130,
        },
        Lab {
            l: 30.69424,
            a: 28.284863,
            b: -36.654877,
        },
        "3837",
    ),
    (
        RgbColor {
            red: 89,
            green: 62,
            blue: 99,
        },
        Lab {
            l: 30.568703,
            a: 19.25227,
            b: -17.233826,
        },
        "327",
    ),
    (
        RgbColor {
            red: 208,
            green: 187,
            blue: 211,
        },
        Lab {
            l: 78.25958,
            a: 11.721134,
            b: -9.30971,
        },
        "153",
    ),
    (
        RgbColor {
            red: 182,
            green: 158,
            blue: 190,
        },
        Lab {
            l: 68.10661,
            a: 14.68131,
            b: -13.213099,
        },
        "554",
    ),
    (
        RgbColor {
            red: 122,
            green: 95,
            blue: 150,
        },
        Lab {
            l: 44.975285,
            a: 22.284687,
            b: -26.049948,
        },
        "553",
    ),
    (
        RgbColor {
            red: 101,
            green: 70,
            blue: 127,
        },
        Lab {
            l: 35.359524,
            a: 25.110006,
            b: -27.277267,
        },
        "552",
    ),
    (
        RgbColor {
            red: 53,
            green: 13,
            blue: 71,
        },
        Lab {
            l: 12.60622,
            a: 30.180983,
            b: -27.226582,
        },
        "550",
    ),
    (
        RgbColor {
            red: 200,
            green: 206,
            blue: 218,
        },
        Lab {
            l: 82.63802,
            a: 0.25457144,
            b: -6.642127,
        },
        "3747",
    ),
    (
        RgbColor {
            red: 156,
            green: 175,
            blue: 201,
        },
        Lab {
            l: 70.8324,
            a: -0.8968711,
            b: -15.395331,
        },
        "341",
    ),
    (
        RgbColor {
            red: 129,
            green: 151,
            blue: 186,
        },
        Lab {
            l: 61.95053,
            a: 0.53951144,
            b: -20.696104,
        },
        "156",
    ),
    (
        RgbColor {
            red: 119,
            green: 131,
            blue: 180,
        },
        Lab {
            l: 55.532036,
            a: 7.5284243,
            b: -27.211273,
        },
        "340",
    ),
    (
        RgbColor {
            red: 120,
            green: 127,
            blue: 178,
        },
        Lab {
            l: 54.43377,
            a: 9.64275,
            b: -27.758812,
        },
        "155",
    ),
    (
        RgbColor {
            red: 85,
            green: 95,
            blue: 154,
        },
        Lab {
            l: 41.919933,
            a: 12.312055,
            b: -33.452965,
        },
        "3746",
    ),
    (
        RgbColor {
            red: 60,
            green: 62,
            blue: 129,
        },
        Lab {
            l: 29.388622,
            a: 19.27185,
            b: -38.249332,
        },
        "333",
    ),
    (
        RgbColor {
            red: 165,
            green: 187,
            blue: 207,
        },
        Lab {
            l: 74.85276,
            a: -3.2191873,
            b: -12.572455,
        },
        "157",
    ),
    (
        RgbColor {
            red: 132,
            green: 160,
            blue: 186,
        },
        Lab {
            l: 64.63698,
            a: -3.5935938,
            b: -16.594994,
        },
        "794",
    ),
    (
        RgbColor {
            red: 102,
            green: 124,
            blue: 156,
        },
        Lab {
            l: 51.42749,
            a: 0.101447105,
            b: -19.76707,
        },
        "793",
    ),
    (
        RgbColor {
            red: 73,
            green: 93,
            blue: 137,
        },
        Lab {
            l: 39.665966,
            a: 5.0178766,
            b: -26.828348,
        },
        "3807",
    ),
    (
        RgbColor {
            red: 49,
            green: 77,
            blue: 130,
        },
        Lab {
            l: 33.075,
            a: 7.1241856,
            b: -33.02673,
        },
        "792",
    ),
    (
        RgbColor {
            red: 43,
            green: 64,
            blue: 108,
        },
        Lab {
            l: 27.470665,
            a: 6.440014,
            b: -28.134144,
        },
        "158",
    ),
    (
        RgbColor {
            red: 12,
            green: 37,
            blue: 87,
        },
        Lab {
            l: 15.948576,
            a: 11.641115,
            b: -32.946976,
        },
        "791",
    ),
    (
        RgbColor {
            red: 167,
            green: 193,
            blue: 217,
        },
        Lab {
            l: 76.89907,
            a: -3.563851,
            b: -14.908636,
        },
        "3840",
    ),
    (
        RgbColor {
            red: 116,
            green: 144,
            blue: 187,
        },
        Lab {
            l: 59.20176,
            a: 1.0249913,
            b: -25.54797,
        },
        "3839",
    ),
    (
        RgbColor {
            red: 91,
            green: 121,
            blue: 170,
        },
        Lab {
            l: 50.452805,
            a: 2.8391778,
            b: -29.474878,
        },
        "3838",
    ),
    (
        RgbColor {
            red: 167,
            green: 194,
            blue: 213,
        },
        Lab {
            l: 77.04234,
            a: -4.996419,
            b: -12.532091,
        },
        "800",
    ),
    (
        RgbColor {
            red: 119,
            green: 135,
            blue: 160,
        },
        Lab {
            l: 55.87436,
            a: 0.028818846,
            b: -15.2035475,
        },
        "809",
    ),
    (
        RgbColor {
            red: 102,
            green: 140,
            blue: 175,
        },
        Lab {
            l: 56.757523,
            a: -3.877908,
            b: -22.555483,
        },
        "799",
    ),
    (
        RgbColor {
            red: 48,
            green: 101,
            blue: 152,
        },
        Lab {
            l: 41.535225,
            a: -0.41988492,
            b: -33.002792,
        },
        "798",
    ),
    (
        RgbColor {
            red: 29,
            green: 65,
            blue: 121,
        },
        Lab {
            l: 27.90424,
            a: 8.140892,
            b: -35.718853,
        },
        "797",
    ),
    (
        RgbColor {
            red: 0,
            green: 42,
            blue: 101,
        },
        Lab {
            l: 18.3438,
            a: 13.018743,
            b: -38.35891,
        },
        "796",
    ),
    (
        RgbColor {
            red: 0,
            green: 26,
            blue: 83,
        },
        Lab {
            l: 11.710039,
            a: 16.977505,
            b: -37.17288,
        },
        "820",
    ),
    (
        RgbColor {
            red: 200,
            green: 219,
            blue: 223,
        },
        Lab {
            l: 86.14098,
            a: -5.543411,
            b: -4.127574,
        },
        "162",
    ),
    (
        RgbColor {
            red: 172,
            green: 200,
            blue: 213,
        },
        Lab {
            l: 78.973434,
            a: -6.6164436,
            b: -9.614778,
        },
        "827",
    ),
    (
        RgbColor {
            red: 124,
            green: 163,
            blue: 186,
        },
        Lab {
            l: 64.9813,
            a: -7.605344,
            b: -16.121614,
        },
        "813",
    ),
    (
        RgbColor {
            red: 78,
            green: 127,
            blue: 162,
        },
        Lab {
            l: 51.159004,
            a: -6.3841343,
            b: -23.789227,
        },
        "826",
    ),
    (
        RgbColor {
            red: 6,
            green: 83,
            blue: 123,
        },
        Lab {
            l: 33.253082,
            a: -5.315438,
            b: -28.499233,
        },
        "825",
    ),
    (
        RgbColor {
            red: 0,
            green: 60,
            blue: 103,
        },
        Lab {
            l: 24.3558,
            a: 0.88483095,
            b: -29.972326,
        },
        "824",
    ),
    (
        RgbColor {
            red: 58,
            green: 171,
            blue: 209,
        },
        Lab {
            l: 65.4569,
            a: -19.154846,
            b: -28.413368,
        },
        "996",
    ),
    (
        RgbColor {
            red: 0,
            green: 136,
            blue: 183,
        },
        Lab {
            l: 52.977478,
            a: -13.6878195,
            b: -33.21916,
        },
        "3843",
    ),
    (
        RgbColor {
            red: 0,
            green: 123,
            blue: 175,
        },
        Lab {
            l: 48.58555,
            a: -9.276658,
            b: -35.493683,
        },
        "995",
    ),
    (
        RgbColor {
            red: 56,
            green: 196,
            blue: 211,
        },
        Lab {
            l: 72.905754,
            a: -31.347544,
            b: -18.136047,
        },
        "3846",
    ),
    (
        RgbColor {
            red: 0,
            green: 171,
            blue: 189,
        },
        Lab {
            l: 63.99681,
            a: -29.904247,
            b: -19.611883,
        },
        "3845",
    ),
    (
        RgbColor {
            red: 0,
            green: 136,
            blue: 161,
        },
        Lab {
            l: 52.040237,
            a: -21.668821,
            b: -22.007061,
        },
        "3844",
    ),
    (
        RgbColor {
            red: 166,
            green: 173,
            blue: 190,
        },
        Lab {
            l: 70.67154,
            a: 0.99143386,
            b: -9.509838,
        },
        "159",
    ),
    (
        RgbColor {
            red: 119,
            green: 135,
            blue: 160,
        },
        Lab {
            l: 55.87436,
            a: 0.028818846,
            b: -15.2035475,
        },
        "160",
    ),
    (
        RgbColor {
            red: 81,
            green: 96,
            blue: 124,
        },
        Lab {
            l: 40.524345,
            a: 1.5892237,
            b: -17.558437,
        },
        "161",
    ),
    (
        RgbColor {
            red: 229,
            green: 232,
            blue: 226,
        },
        Lab {
            l: 91.62299,
            a: -2.0702183,
            b: 2.560532,
        },
        "3756",
    ),
    (
        RgbColor {
            red: 199,
            green: 214,
            blue: 217,
        },
        Lab {
            l: 84.60955,
            a: -4.4537187,
            b: -3.210616,
        },
        "775",
    ),
    (
        RgbColor {
            red: 191,
            green: 210,
            blue: 217,
        },
        Lab {
            l: 83.0083,
            a: -4.978895,
            b: -5.633199,
        },
        "3841",
    ),
    (
        RgbColor {
            red: 161,
            green: 189,
            blue: 205,
        },
        Lab {
            l: 75.06341,
            a: -5.981028,
            b: -11.197746,
        },
        "3325",
    ),
    (
        RgbColor {
            red: 134,
            green: 171,
            blue: 194,
        },
        Lab {
            l: 68.08551,
            a: -7.0499477,
            b: -15.818871,
        },
        "3755",
    ),
    (
        RgbColor {
            red: 91,
            green: 136,
            blue: 164,
        },
        Lab {
            l: 54.56594,
            a: -7.674098,
            b: -19.657766,
        },
        "334",
    ),
    (
        RgbColor {
            red: 87,
            green: 126,
            blue: 162,
        },
        Lab {
            l: 51.331146,
            a: -3.560394,
            b: -23.485052,
        },
        "322",
    ),
    (
        RgbColor {
            red: 14,
            green: 68,
            blue: 105,
        },
        Lab {
            l: 27.427483,
            a: -2.405703,
            b: -26.36243,
        },
        "312",
    ),
    (
        RgbColor {
            red: 0,
            green: 53,
            blue: 88,
        },
        Lab {
            l: 21.020409,
            a: -0.95927715,
            b: -25.489933,
        },
        "803",
    ),
    (
        RgbColor {
            red: 15,
            green: 42,
            blue: 71,
        },
        Lab {
            l: 16.563896,
            a: 1.308158,
            b: -21.100498,
        },
        "336",
    ),
    (
        RgbColor {
            red: 9,
            green: 13,
            blue: 37,
        },
        Lab {
            l: 4.330971,
            a: 5.5186453,
            b: -16.0023,
        },
        "823",
    ),
    (
        RgbColor {
            red: 11,
            green: 12,
            blue: 28,
        },
        Lab {
            l: 3.775013,
            a: 3.3518076,
            b: -9.83457,
        },
        "939",
    ),
    (
        RgbColor {
            red: 199,
            green: 211,
            blue: 215,
        },
        Lab {
            l: 83.776146,
            a: -3.2806098,
            b: -3.37317,
        },
        "3753",
    ),
    (
        RgbColor {
            red: 173,
            green: 191,
            blue: 199,
        },
        Lab {
            l: 76.23393,
            a: -4.510254,
            b: -6.113136,
        },
        "3752",
    ),
    (
        RgbColor {
            red: 139,
            green: 161,
            blue: 175,
        },
        Lab {
            l: 65.02065,
            a: -4.5778456,
            b: -9.8187685,
        },
        "932",
    ),
    (
        RgbColor {
            red: 91,
            green: 122,
            blue: 142,
        },
        Lab {
            l: 49.611877,
            a: -5.8421793,
            b: -14.402782,
        },
        "931",
    ),
    (
        RgbColor {
            red: 42,
            green: 69,
            blue: 81,
        },
        Lab {
            l: 27.688648,
            a: -6.527215,
            b: -10.397434,
        },
        "930",
    ),
    (
        RgbColor {
            red: 13,
            green: 51,
            blue: 70,
        },
        Lab {
            l: 19.619278,
            a: -5.730972,
            b: -15.624839,
        },
        "3750",
    ),
    (
        RgbColor {
            red: 195,
            green: 217,
            blue: 217,
        },
        Lab {
            l: 85.11708,
            a: -7.3117914,
            b: -2.4972796,
        },
        "828",
    ),
    (
        RgbColor {
            red: 177,
            green: 209,
            blue: 213,
        },
        Lab {
            l: 81.70271,
            a: -9.744853,
            b: -5.532491,
        },
        "3761",
    ),
    (
        RgbColor {
            red: 144,
            green: 183,
            blue: 196,
        },
        Lab {
            l: 72.08166,
            a: -10.042399,
            b: -10.856462,
        },
        "519",
    ),
    (
        RgbColor {
            red: 86,
            green: 146,
            blue: 161,
        },
        Lab {
            l: 57.180817,
            a: -15.347063,
            b: -13.986074,
        },
        "518",
    ),
    (
        RgbColor {
            red: 66,
            green: 129,
            blue: 152,
        },
        Lab {
            l: 50.832863,
            a: -13.364643,
            b: -18.514788,
        },
        "3760",
    ),
    (
        RgbColor {
            red: 11,
            green: 94,
            blue: 129,
        },
        Lab {
            l: 37.226635,
            a: -9.373784,
            b: -25.948101,
        },
        "517",
    ),
    (
        RgbColor {
            red: 0,
            green: 72,
            blue: 104,
        },
        Lab {
            l: 28.468979,
            a: -6.6637545,
            b: -24.100584,
        },
        "3842",
    ),
    (
        RgbColor {
            red: 0,
            green: 56,
            blue: 84,
        },
        Lab {
            l: 21.828934,
            a: -4.7955513,
            b: -21.563387,
        },
        "311",
    ),
    (
        RgbColor {
            red: 205,
            green: 228,
            blue: 227,
        },
        Lab {
            l: 88.92777,
            a: -7.767886,
            b: -2.1004915,
        },
        "747",
    ),
    (
        RgbColor {
            red: 144,
            green: 190,
            blue: 193,
        },
        Lab {
            l: 73.93657,
            a: -14.409483,
            b: -6.458962,
        },
        "3766",
    ),
    (
        RgbColor {
            red: 85,
            green: 147,
            blue: 156,
        },
        Lab {
            l: 57.2678,
            a: -17.528116,
            b: -11.000788,
        },
        "807",
    ),
    (
        RgbColor {
            red: 49,
            green: 94,
            blue: 70,
        },
        Lab {
            l: 36.176624,
            a: -21.753996,
            b: 9.03207,
        },
        "806",
    ),
    (
        RgbColor {
            red: 0,
            green: 96,
            blue: 121,
        },
        Lab {
            l: 37.381744,
            a: -14.3729,
            b: -20.820248,
        },
        "3765",
    ),
    (
        RgbColor {
            red: 182,
            green: 210,
            blue: 206,
        },
        Lab {
            l: 82.10579,
            a: -10.096788,
            b: -1.1665463,
        },
        "3811",
    ),
    (
        RgbColor {
            red: 154,
            green: 191,
            blue: 187,
        },
        Lab {
            l: 74.647064,
            a: -13.159394,
            b: -2.078843,
        },
        "598",
    ),
    (
        RgbColor {
            red: 106,
            green: 156,
            blue: 161,
        },
        Lab {
            l: 61.1447,
            a: -15.336752,
            b: -7.9392314,
        },
        "597",
    ),
    (
        RgbColor {
            red: 70,
            green: 133,
            blue: 136,
        },
        Lab {
            l: 51.671562,
            a: -19.364565,
            b: -7.897067,
        },
        "3810",
    ),
    (
        RgbColor {
            red: 1,
            green: 97,
            blue: 99,
        },
        Lab {
            l: 36.848007,
            a: -22.918478,
            b: -8.073341,
        },
        "3809",
    ),
    (
        RgbColor {
            red: 25,
            green: 78,
            blue: 80,
        },
        Lab {
            l: 29.99606,
            a: -16.405851,
            b: -6.341505,
        },
        "3808",
    ),
    (
        RgbColor {
            red: 200,
            green: 205,
            blue: 198,
        },
        Lab {
            l: 81.848595,
            a: -2.9891133,
            b: 2.882278,
        },
        "928",
    ),
    (
        RgbColor {
            red: 159,
            green: 171,
            blue: 163,
        },
        Lab {
            l: 68.85318,
            a: -5.764842,
            b: 2.6668072,
        },
        "927",
    ),
    (
        RgbColor {
            red: 121,
            green: 137,
            blue: 133,
        },
        Lab {
            l: 55.7351,
            a: -6.626606,
            b: 0.17547607,
        },
        "926",
    ),
    (
        RgbColor {
            red: 73,
            green: 90,
            blue: 95,
        },
        Lab {
            l: 37.031284,
            a: -5.2600203,
            b: -5.044651,
        },
        "3768",
    ),
    (
        RgbColor {
            red: 31,
            green: 63,
            blue: 66,
        },
        Lab {
            l: 24.44786,
            a: -10.806263,
            b: -5.4983196,
        },
        "924",
    ),
    (
        RgbColor {
            red: 102,
            green: 158,
            blue: 148,
        },
        Lab {
            l: 61.146736,
            a: -20.72385,
            b: -0.6141901,
        },
        "3849",
    ),
    (
        RgbColor {
            red: 47,
            green: 121,
            blue: 108,
        },
        Lab {
            l: 46.12481,
            a: -26.234627,
            b: 0.30162334,
        },
        "3848",
    ),
    (
        RgbColor {
            red: 0,
            green: 82,
            blue: 69,
        },
        Lab {
            l: 30.553883,
            a: -25.43433,
            b: 1.4595151,
        },
        "3847",
    ),
    (
        RgbColor {
            red: 168,
            green: 212,
            blue: 199,
        },
        Lab {
            l: 81.58433,
            a: -16.998589,
            b: 1.6989946,
        },
        "964",
    ),
    (
        RgbColor {
            red: 117,
            green: 183,
            blue: 159,
        },
        Lab {
            l: 69.57684,
            a: -26.454628,
            b: 5.58877,
        },
        "959",
    ),
    (
        RgbColor {
            red: 102,
            green: 171,
            blue: 146,
        },
        Lab {
            l: 64.98225,
            a: -27.72513,
            b: 6.064534,
        },
        "958",
    ),
    (
        RgbColor {
            red: 0,
            green: 136,
            blue: 103,
        },
        Lab {
            l: 50.199966,
            a: -39.83894,
            b: 8.986866,
        },
        "3812",
    ),
    (
        RgbColor {
            red: 89,
            green: 169,
            blue: 132,
        },
        Lab {
            l: 63.47866,
            a: -33.40736,
            b: 11.615622,
        },
        "3851",
    ),
    (
        RgbColor {
            red: 18,
            green: 140,
            blue: 100,
        },
        Lab {
            l: 51.614708,
            a: -41.119186,
            b: 12.717438,
        },
        "943",
    ),
    (
        RgbColor {
            red: 0,
            green: 124,
            blue: 77,
        },
        Lab {
            l: 45.56614,
            a: -41.36649,
            b: 17.58356,
        },
        "3850",
    ),
    (
        RgbColor {
            red: 129,
            green: 177,
            blue: 151,
        },
        Lab {
            l: 68.27406,
            a: -21.461128,
            b: 8.200216,
        },
        "993",
    ),
    (
        RgbColor {
            red: 96,
            green: 153,
            blue: 125,
        },
        Lab {
            l: 58.7414,
            a: -25.183529,
            b: 8.876538,
        },
        "992",
    ),
    (
        RgbColor {
            red: 44,
            green: 120,
            blue: 98,
        },
        Lab {
            l: 45.427475,
            a: -29.052616,
            b: 5.218959,
        },
        "3814",
    ),
    (
        RgbColor {
            red: 0,
            green: 97,
            blue: 71,
        },
        Lab {
            l: 35.99168,
            a: -31.748205,
            b: 7.9760847,
        },
        "991",
    ),
    (
        RgbColor {
            red: 167,
            green: 188,
            blue: 156,
        },
        Lab {
            l: 73.921585,
            a: -13.150811,
            b: 13.814473,
        },
        "966",
    ),
    (
        RgbColor {
            red: 174,
            green: 199,
            blue: 167,
        },
        Lab {
            l: 77.65909,
            a: -14.430464,
            b: 13.228822,
        },
        "564",
    ),
    (
        RgbColor {
            red: 150,
            green: 183,
            blue: 149,
        },
        Lab {
            l: 71.22825,
            a: -17.823696,
            b: 13.65844,
        },
        "563",
    ),
    (
        RgbColor {
            red: 92,
            green: 134,
            blue: 94,
        },
        Lab {
            l: 51.98752,
            a: -23.096651,
            b: 17.03664,
        },
        "562",
    ),
    (
        RgbColor {
            red: 55,
            green: 99,
            blue: 61,
        },
        Lab {
            l: 37.972313,
            a: -24.265856,
            b: 16.966904,
        },
        "505",
    ),
    (
        RgbColor {
            red: 165,
            green: 187,
            blue: 168,
        },
        Lab {
            l: 73.78287,
            a: -11.1987,
            b: 7.089627,
        },
        "3817",
    ),
    (
        RgbColor {
            red: 118,
            green: 154,
            blue: 133,
        },
        Lab {
            l: 60.476532,
            a: -16.894669,
            b: 6.9996715,
        },
        "3816",
    ),
    (
        RgbColor {
            red: 85,
            green: 122,
            blue: 96,
        },
        Lab {
            l: 47.8728,
            a: -18.914997,
            b: 10.123873,
        },
        "163",
    ),
    (
        RgbColor {
            red: 76,
            green: 114,
            blue: 87,
        },
        Lab {
            l: 44.601963,
            a: -19.655586,
            b: 10.770071,
        },
        "3815",
    ),
    (
        RgbColor {
            red: 49,
            green: 94,
            blue: 70,
        },
        Lab {
            l: 36.176624,
            a: -21.753996,
            b: 9.03207,
        },
        "561",
    ),
    (
        RgbColor {
            red: 154,
            green: 115,
            blue: 30,
        },
        Lab {
            l: 50.950882,
            a: 7.278979,
            b: 49.49538,
        },
        "504",
    ),
    (
        RgbColor {
            red: 179,
            green: 196,
            blue: 182,
        },
        Lab {
            l: 77.543564,
            a: -8.456766,
            b: 5.0468087,
        },
        "3813",
    ),
    (
        RgbColor {
            red: 140,
            green: 167,
            blue: 154,
        },
        Lab {
            l: 66.171165,
            a: -12.001723,
            b: 3.6071658,
        },
        "503",
    ),
    (
        RgbColor {
            red: 116,
            green: 140,
            blue: 124,
        },
        Lab {
            l: 56.011703,
            a: -11.923849,
            b: 5.711043,
        },
        "502",
    ),
    (
        RgbColor {
            red: 83,
            green: 106,
            blue: 94,
        },
        Lab {
            l: 42.695354,
            a: -11.260525,
            b: 3.9385498,
        },
        "501",
    ),
    (
        RgbColor {
            red: 19,
            green: 37,
            blue: 25,
        },
        Lab {
            l: 12.8083515,
            a: -10.886013,
            b: 5.700988,
        },
        "500",
    ),
    (
        RgbColor {
            red: 190,
            green: 218,
            blue: 177,
        },
        Lab {
            l: 84.10295,
            a: -16.747147,
            b: 17.134548,
        },
        "955",
    ),
    (
        RgbColor {
            red: 157,
            green: 196,
            blue: 147,
        },
        Lab {
            l: 75.298004,
            a: -22.26624,
            b: 20.460976,
        },
        "954",
    ),
    (
        RgbColor {
            red: 127,
            green: 173,
            blue: 119,
        },
        Lab {
            l: 66.31348,
            a: -26.01391,
            b: 22.94383,
        },
        "913",
    ),
    (
        RgbColor {
            red: 97,
            green: 155,
            blue: 99,
        },
        Lab {
            l: 58.867058,
            a: -30.84916,
            b: 23.552345,
        },
        "912",
    ),
    (
        RgbColor {
            red: 57,
            green: 132,
            blue: 68,
        },
        Lab {
            l: 49.239326,
            a: -37.677364,
            b: 27.603685,
        },
        "911",
    ),
    (
        RgbColor {
            red: 30,
            green: 108,
            blue: 43,
        },
        Lab {
            l: 39.873863,
            a: -38.123608,
            b: 29.254501,
        },
        "910",
    ),
    (
        RgbColor {
            red: 0,
            green: 85,
            blue: 20,
        },
        Lab {
            l: 30.753067,
            a: -37.18528,
            b: 30.160301,
        },
        "909",
    ),
    (
        RgbColor {
            red: 0,
            green: 64,
            blue: 21,
        },
        Lab {
            l: 22.72594,
            a: -29.607862,
            b: 20.781353,
        },
        "3818",
    ),
    (
        RgbColor {
            red: 194,
            green: 204,
            blue: 168,
        },
        Lab {
            l: 80.450096,
            a: -9.677649,
            b: 16.821707,
        },
        "369",
    ),
    (
        RgbColor {
            red: 143,
            green: 159,
            blue: 116,
        },
        Lab {
            l: 63.257523,
            a: -13.440132,
            b: 20.622194,
        },
        "368",
    ),
    (
        RgbColor {
            red: 105,
            green: 127,
            blue: 91,
        },
        Lab {
            l: 50.613625,
            a: -15.1186285,
            b: 17.018599,
        },
        "320",
    ),
    (
        RgbColor {
            red: 72,
            green: 97,
            blue: 62,
        },
        Lab {
            l: 38.33112,
            a: -16.751453,
            b: 17.030233,
        },
        "367",
    ),
    (
        RgbColor {
            red: 23,
            green: 52,
            blue: 24,
        },
        Lab {
            l: 18.817215,
            a: -18.141598,
            b: 14.390307,
        },
        "319",
    ),
    (
        RgbColor {
            red: 13,
            green: 29,
            blue: 0,
        },
        Lab {
            l: 8.690233,
            a: -12.6605625,
            b: 12.777323,
        },
        "890",
    ),
    (
        RgbColor {
            red: 166,
            green: 183,
            blue: 132,
        },
        Lab {
            l: 71.987144,
            a: -14.753312,
            b: 24.053562,
        },
        "164",
    ),
    (
        RgbColor {
            red: 127,
            green: 143,
            blue: 80,
        },
        Lab {
            l: 56.81746,
            a: -16.36079,
            b: 31.53947,
        },
        "989",
    ),
    (
        RgbColor {
            red: 110,
            green: 126,
            blue: 64,
        },
        Lab {
            l: 50.22409,
            a: -16.352951,
            b: 31.61537,
        },
        "988",
    ),
    (
        RgbColor {
            red: 72,
            green: 92,
            blue: 44,
        },
        Lab {
            l: 36.392715,
            a: -16.705572,
            b: 25.00571,
        },
        "987",
    ),
    (
        RgbColor {
            red: 34,
            green: 66,
            blue: 20,
        },
        Lab {
            l: 24.598148,
            a: -21.79034,
            b: 23.763609,
        },
        "986",
    ),
    (
        RgbColor {
            red: 219,
            green: 219,
            blue: 178,
        },
        Lab {
            l: 86.47331,
            a: -6.7159834,
            b: 20.226992,
        },
        "772",
    ),
    (
        RgbColor {
            red: 189,
            green: 180,
            blue: 115,
        },
        Lab {
            l: 72.69386,
            a: -6.3557625,
            b: 34.30202,
        },
        "3348",
    ),
    (
        RgbColor {
            red: 115,
            green: 121,
            blue: 59,
        },
        Lab {
            l: 49.051018,
            a: -12.022495,
            b: 32.93778,
        },
        "3347",
    ),
    (
        RgbColor {
            red: 90,
            green: 97,
            blue: 43,
        },
        Lab {
            l: 39.40739,
            a: -11.544436,
            b: 29.500961,
        },
        "3346",
    ),
    (
        RgbColor {
            red: 43,
            green: 58,
            blue: 6,
        },
        Lab {
            l: 22.134354,
            a: -14.842719,
            b: 27.806164,
        },
        "3345",
    ),
    (
        RgbColor {
            red: 33,
            green: 58,
            blue: 22,
        },
        Lab {
            l: 21.606377,
            a: -17.789156,
            b: 19.172436,
        },
        "895",
    ),
    (
        RgbColor {
            red: 145,
            green: 161,
            blue: 57,
        },
        Lab {
            l: 63.18148,
            a: -20.255924,
            b: 50.432556,
        },
        "704",
    ),
    (
        RgbColor {
            red: 127,
            green: 153,
            blue: 64,
        },
        Lab {
            l: 59.584526,
            a: -23.193329,
            b: 42.942513,
        },
        "703",
    ),
    (
        RgbColor {
            red: 93,
            green: 135,
            blue: 56,
        },
        Lab {
            l: 51.76969,
            a: -28.734118,
            b: 37.271915,
        },
        "702",
    ),
    (
        RgbColor {
            red: 66,
            green: 120,
            blue: 41,
        },
        Lab {
            l: 45.291157,
            a: -33.262817,
            b: 36.77773,
        },
        "701",
    ),
    (
        RgbColor {
            red: 40,
            green: 105,
            blue: 35,
        },
        Lab {
            l: 39.028038,
            a: -35.59293,
            b: 32.381874,
        },
        "700",
    ),
    (
        RgbColor {
            red: 0,
            green: 79,
            blue: 17,
        },
        Lab {
            l: 28.464859,
            a: -35.47226,
            b: 29.099392,
        },
        "699",
    ),
    (
        RgbColor {
            red: 164,
            green: 174,
            blue: 40,
        },
        Lab {
            l: 68.25456,
            a: -19.625605,
            b: 62.278645,
        },
        "907",
    ),
    (
        RgbColor {
            red: 103,
            green: 135,
            blue: 0,
        },
        Lab {
            l: 52.075424,
            a: -27.760654,
            b: 55.975647,
        },
        "906",
    ),
    (
        RgbColor {
            red: 79,
            green: 102,
            blue: 17,
        },
        Lab {
            l: 39.924355,
            a: -21.229626,
            b: 41.397053,
        },
        "905",
    ),
    (
        RgbColor {
            red: 62,
            green: 81,
            blue: 15,
        },
        Lab {
            l: 31.677837,
            a: -17.970175,
            b: 33.964455,
        },
        "904",
    ),
    (
        RgbColor {
            red: 193,
            green: 179,
            blue: 98,
        },
        Lab {
            l: 72.53523,
            a: -5.935073,
            b: 42.859566,
        },
        "472",
    ),
    (
        RgbColor {
            red: 144,
            green: 140,
            blue: 72,
        },
        Lab {
            l: 57.224953,
            a: -8.399815,
            b: 36.549843,
        },
        "471",
    ),
    (
        RgbColor {
            red: 117,
            green: 123,
            blue: 35,
        },
        Lab {
            l: 49.580185,
            a: -14.147938,
            b: 44.95164,
        },
        "470",
    ),
    (
        RgbColor {
            red: 94,
            green: 92,
            blue: 23,
        },
        Lab {
            l: 38.014256,
            a: -8.368313,
            b: 37.792133,
        },
        "469",
    ),
    (
        RgbColor {
            red: 81,
            green: 85,
            blue: 19,
        },
        Lab {
            l: 34.586727,
            a: -10.891795,
            b: 35.704144,
        },
        "937",
    ),
    (
        RgbColor {
            red: 66,
            green: 66,
            blue: 28,
        },
        Lab {
            l: 27.126038,
            a: -6.293759,
            b: 22.863441,
        },
        "936",
    ),
    (
        RgbColor {
            red: 51,
            green: 55,
            blue: 28,
        },
        Lab {
            l: 22.016682,
            a: -6.892681,
            b: 16.326517,
        },
        "935",
    ),
    (
        RgbColor {
            red: 39,
            green: 39,
            blue: 22,
        },
        Lab {
            l: 15.170898,
            a: -3.4177752,
            b: 11.260906,
        },
        "934",
    ),
    (
        RgbColor {
            red: 162,
            green: 162,
            blue: 134,
        },
        Lab {
            l: 65.9378,
            a: -4.899144,
            b: 14.605999,
        },
        "523",
    ),
    (
        RgbColor {
            red: 157,
            green: 154,
            blue: 112,
        },
        Lab {
            l: 62.885284,
            a: -5.8138075,
            b: 22.50756,
        },
        "3053",
    ),
    (
        RgbColor {
            red: 132,
            green: 128,
            blue: 90,
        },
        Lab {
            l: 53.039497,
            a: -4.8196316,
            b: 21.225327,
        },
        "3052",
    ),
    (
        RgbColor {
            red: 72,
            green: 69,
            blue: 29,
        },
        Lab {
            l: 28.702011,
            a: -5.0543995,
            b: 24.28354,
        },
        "3051",
    ),
    (
        RgbColor {
            red: 179,
            green: 176,
            blue: 148,
        },
        Lab {
            l: 71.400505,
            a: -3.7246048,
            b: 14.754116,
        },
        "524",
    ),
    (
        RgbColor {
            red: 150,
            green: 154,
            blue: 126,
        },
        Lab {
            l: 62.5934,
            a: -6.4739285,
            b: 14.244819,
        },
        "522",
    ),
    (
        RgbColor {
            red: 68,
            green: 80,
            blue: 54,
        },
        Lab {
            l: 32.330616,
            a: -10.143176,
            b: 13.806015,
        },
        "520",
    ),
    (
        RgbColor {
            red: 135,
            green: 135,
            blue: 89,
        },
        Lab {
            l: 55.27813,
            a: -7.5818005,
            b: 24.79266,
        },
        "3364",
    ),
    (
        RgbColor {
            red: 108,
            green: 113,
            blue: 77,
        },
        Lab {
            l: 46.3575,
            a: -8.364618,
            b: 19.40067,
        },
        "3363",
    ),
    (
        RgbColor {
            red: 82,
            green: 90,
            blue: 59,
        },
        Lab {
            l: 36.784325,
            a: -9.081453,
            b: 16.906404,
        },
        "3362",
    ),
    (
        RgbColor {
            red: 218,
            green: 199,
            blue: 116,
        },
        Lab {
            l: 80.187805,
            a: -4.3846965,
            b: 43.76731,
        },
        "165",
    ),
    (
        RgbColor {
            red: 213,
            green: 194,
            blue: 90,
        },
        Lab {
            l: 78.151276,
            a: -6.059587,
            b: 53.730404,
        },
        "3819",
    ),
    (
        RgbColor {
            red: 185,
            green: 162,
            blue: 22,
        },
        Lab {
            l: 66.68321,
            a: -4.7112107,
            b: 66.422325,
        },
        "166",
    ),
    (
        RgbColor {
            red: 136,
            green: 132,
            blue: 32,
        },
        Lab {
            l: 53.85845,
            a: -10.485947,
            b: 50.795956,
        },
        "581",
    ),
    (
        RgbColor {
            red: 103,
            green: 94,
            blue: 23,
        },
        Lab {
            l: 39.49806,
            a: -5.14853,
            b: 39.528255,
        },
        "580",
    ),
    (
        RgbColor {
            red: 187,
            green: 156,
            blue: 74,
        },
        Lab {
            l: 65.6219,
            a: 1.7848313,
            b: 46.37965,
        },
        "734",
    ),
    (
        RgbColor {
            red: 165,
            green: 135,
            blue: 44,
        },
        Lab {
            l: 57.569176,
            a: 1.688689,
            b: 50.700043,
        },
        "733",
    ),
    (
        RgbColor {
            red: 116,
            green: 92,
            blue: 1,
        },
        Lab {
            l: 40.19681,
            a: 1.6952157,
            b: 47.478832,
        },
        "732",
    ),
    (
        RgbColor {
            red: 213,
            green: 198,
            blue: 178,
        },
        Lab {
            l: 80.59924,
            a: 1.9642711,
            b: 11.916721,
        },
        "731",
    ),
    (
        RgbColor {
            red: 90,
            green: 72,
            blue: 8,
        },
        Lab {
            l: 31.407974,
            a: 0.9370744,
            b: 37.22069,
        },
        "730",
    ),
    (
        RgbColor {
            red: 176,
            green: 161,
            blue: 113,
        },
        Lab {
            l: 66.45327,
            a: -1.758486,
            b: 27.01273,
        },
        "3013",
    ),
    (
        RgbColor {
            red: 134,
            green: 117,
            blue: 65,
        },
        Lab {
            l: 49.705376,
            a: -0.68992376,
            b: 30.922169,
        },
        "3012",
    ),
    (
        RgbColor {
            red: 102,
            green: 88,
            blue: 47,
        },
        Lab {
            l: 37.85995,
            a: -0.25738776,
            b: 25.710625,
        },
        "3011",
    ),
    (
        RgbColor {
            red: 179,
            green: 161,
            blue: 117,
        },
        Lab {
            l: 66.79301,
            a: -0.040352345,
            b: 25.34436,
        },
        "372",
    ),
    (
        RgbColor {
            red: 158,
            green: 133,
            blue: 82,
        },
        Lab {
            l: 56.736885,
            a: 2.5576353,
            b: 30.938852,
        },
        "371",
    ),
    (
        RgbColor {
            red: 159,
            green: 136,
            blue: 82,
        },
        Lab {
            l: 57.64508,
            a: 1.3152063,
            b: 32.099968,
        },
        "370",
    ),
    (
        RgbColor {
            red: 198,
            green: 163,
            blue: 87,
        },
        Lab {
            l: 68.671326,
            a: 3.7240386,
            b: 43.819214,
        },
        "834",
    ),
    (
        RgbColor {
            red: 174,
            green: 136,
            blue: 54,
        },
        Lab {
            l: 58.858063,
            a: 5.719572,
            b: 47.972073,
        },
        "833",
    ),
    (
        RgbColor {
            red: 154,
            green: 115,
            blue: 30,
        },
        Lab {
            l: 50.950882,
            a: 7.278979,
            b: 49.49538,
        },
        "832",
    ),
    (
        RgbColor {
            red: 127,
            green: 95,
            blue: 22,
        },
        Lab {
            l: 42.391823,
            a: 5.858481,
            b: 43.55477,
        },
        "831",
    ),
    (
        RgbColor {
            red: 103,
            green: 75,
            blue: 16,
        },
        Lab {
            l: 33.886,
            a: 5.9375167,
            b: 37.09279,
        },
        "830",
    ),
    (
        RgbColor {
            red: 95,
            green: 61,
            blue: 13,
        },
        Lab {
            l: 28.901794,
            a: 10.516151,
            b: 33.410892,
        },
        "829",
    ),
    (
        RgbColor {
            red: 214,
            green: 201,
            blue: 177,
        },
        Lab {
            l: 81.42578,
            a: 0.6030798,
            b: 13.631571,
        },
        "613",
    ),
    (
        RgbColor {
            red: 173,
            green: 148,
            blue: 109,
        },
        Lab {
            l: 62.65007,
            a: 3.6174655,
            b: 24.096752,
        },
        "612",
    ),
    (
        RgbColor {
            red: 112,
            green: 90,
            blue: 57,
        },
        Lab {
            l: 39.669407,
            a: 4.273906,
            b: 22.41018,
        },
        "611",
    ),
    (
        RgbColor {
            red: 107,
            green: 82,
            blue: 45,
        },
        Lab {
            l: 36.650112,
            a: 5.543679,
            b: 25.511414,
        },
        "610",
    ),
    (
        RgbColor {
            red: 225,
            green: 206,
            blue: 163,
        },
        Lab {
            l: 83.33474,
            a: 0.11381507,
            b: 23.83343,
        },
        "3047",
    ),
    (
        RgbColor {
            red: 208,
            green: 181,
            blue: 126,
        },
        Lab {
            l: 74.80303,
            a: 2.028823,
            b: 31.602669,
        },
        "3046",
    ),
    (
        RgbColor {
            red: 169,
            green: 131,
            blue: 82,
        },
        Lab {
            l: 57.3452,
            a: 8.439064,
            b: 31.944853,
        },
        "3045",
    ),
    (
        RgbColor {
            red: 149,
            green: 107,
            blue: 58,
        },
        Lab {
            l: 48.525284,
            a: 11.20153,
            b: 33.67389,
        },
        "167",
    ),
    (
        RgbColor {
            red: 245,
            green: 235,
            blue: 207,
        },
        Lab {
            l: 93.16087,
            a: -1.15484,
            b: 14.806902,
        },
        "746",
    ),
    (
        RgbColor {
            red: 233,
            green: 209,
            blue: 160,
        },
        Lab {
            l: 84.72228,
            a: 1.2244582,
            b: 27.402603,
        },
        "677",
    ),
    (
        RgbColor {
            red: 191,
            green: 157,
            blue: 109,
        },
        Lab {
            l: 66.745224,
            a: 6.116062,
            b: 29.817795,
        },
        "422",
    ),
    (
        RgbColor {
            red: 168,
            green: 130,
            blue: 79,
        },
        Lab {
            l: 56.934555,
            a: 8.28743,
            b: 33.032894,
        },
        "3828",
    ),
    (
        RgbColor {
            red: 135,
            green: 90,
            blue: 40,
        },
        Lab {
            l: 42.180805,
            a: 13.474703,
            b: 35.51158,
        },
        "420",
    ),
    (
        RgbColor {
            red: 112,
            green: 76,
            blue: 30,
        },
        Lab {
            l: 35.415737,
            a: 10.43047,
            b: 32.506226,
        },
        "869",
    ),
    (
        RgbColor {
            red: 226,
            green: 165,
            blue: 56,
        },
        Lab {
            l: 71.8034,
            a: 12.770444,
            b: 62.02227,
        },
        "728",
    ),
    (
        RgbColor {
            red: 196,
            green: 129,
            blue: 28,
        },
        Lab {
            l: 59.454536,
            a: 18.292397,
            b: 59.505947,
        },
        "783",
    ),
    (
        RgbColor {
            red: 153,
            green: 92,
            blue: 0,
        },
        Lab {
            l: 44.840355,
            a: 19.274176,
            b: 53.10095,
        },
        "782",
    ),
    (
        RgbColor {
            red: 212,
            green: 140,
            blue: 172,
        },
        Lab {
            l: 66.31575,
            a: 31.907648,
            b: -5.493486,
        },
        "781",
    ),
    (
        RgbColor {
            red: 132,
            green: 71,
            blue: 5,
        },
        Lab {
            l: 36.787975,
            a: 21.87419,
            b: 44.824062,
        },
        "780",
    ),
    (
        RgbColor {
            red: 219,
            green: 175,
            blue: 109,
        },
        Lab {
            l: 74.078354,
            a: 7.9699755,
            b: 39.732994,
        },
        "676",
    ),
    (
        RgbColor {
            red: 184,
            green: 135,
            blue: 67,
        },
        Lab {
            l: 59.82096,
            a: 11.50468,
            b: 43.129128,
        },
        "729",
    ),
    (
        RgbColor {
            red: 160,
            green: 114,
            blue: 47,
        },
        Lab {
            l: 51.512993,
            a: 11.401624,
            b: 42.982475,
        },
        "680",
    ),
    (
        RgbColor {
            red: 148,
            green: 98,
            blue: 25,
        },
        Lab {
            l: 45.774292,
            a: 14.092088,
            b: 46.42494,
        },
        "3829",
    ),
    (
        RgbColor {
            red: 236,
            green: 196,
            blue: 111,
        },
        Lab {
            l: 80.99528,
            a: 4.0607452,
            b: 47.594666,
        },
        "3822",
    ),
    (
        RgbColor {
            red: 221,
            green: 171,
            blue: 70,
        },
        Lab {
            l: 72.85393,
            a: 8.110582,
            b: 57.1165,
        },
        "3821",
    ),
    (
        RgbColor {
            red: 208,
            green: 154,
            blue: 46,
        },
        Lab {
            l: 67.06674,
            a: 10.496527,
            b: 60.630215,
        },
        "3820",
    ),
    (
        RgbColor {
            red: 192,
            green: 129,
            blue: 3,
        },
        Lab {
            l: 58.896004,
            a: 16.107141,
            b: 64.22311,
        },
        "3852",
    ),
    (
        RgbColor {
            red: 255,
            green: 237,
            blue: 148,
        },
        Lab {
            l: 93.436714,
            a: -5.864352,
            b: 45.242928,
        },
        "445",
    ),
    (
        RgbColor {
            red: 255,
            green: 212,
            blue: 56,
        },
        Lab {
            l: 86.32314,
            a: 0.5914569,
            b: 76.59673,
        },
        "307",
    ),
    (
        RgbColor {
            red: 250,
            green: 195,
            blue: 0,
        },
        Lab {
            l: 81.487564,
            a: 6.1743555,
            b: 83.10343,
        },
        "973",
    ),
    (
        RgbColor {
            red: 248,
            green: 189,
            blue: 0,
        },
        Lab {
            l: 79.81527,
            a: 8.42908,
            b: 81.85396,
        },
        "444",
    ),
    (
        RgbColor {
            red: 250,
            green: 227,
            blue: 160,
        },
        Lab {
            l: 90.68944,
            a: -1.4764667,
            b: 35.62622,
        },
        "3078",
    ),
    (
        RgbColor {
            red: 252,
            green: 221,
            blue: 135,
        },
        Lab {
            l: 88.99793,
            a: -0.23931265,
            b: 45.981026,
        },
        "727",
    ),
    (
        RgbColor {
            red: 248,
            green: 200,
            blue: 83,
        },
        Lab {
            l: 82.85607,
            a: 5.0144196,
            b: 62.91915,
        },
        "726",
    ),
    (
        RgbColor {
            red: 238,
            green: 175,
            blue: 66,
        },
        Lab {
            l: 75.59619,
            a: 13.008833,
            b: 62.196754,
        },
        "725",
    ),
    (
        RgbColor {
            red: 245,
            green: 158,
            blue: 0,
        },
        Lab {
            l: 72.14128,
            a: 23.397238,
            b: 76.54223,
        },
        "972",
    ),
    (
        RgbColor {
            red: 253,
            green: 226,
            blue: 172,
        },
        Lab {
            l: 90.89644,
            a: 1.5860498,
            b: 29.831768,
        },
        "745",
    ),
    (
        RgbColor {
            red: 250,
            green: 209,
            blue: 130,
        },
        Lab {
            l: 85.77539,
            a: 4.6449604,
            b: 44.384323,
        },
        "744",
    ),
    (
        RgbColor {
            red: 248,
            green: 192,
            blue: 88,
        },
        Lab {
            l: 80.94027,
            a: 9.453982,
            b: 58.666645,
        },
        "743",
    ),
    (
        RgbColor {
            red: 243,
            green: 162,
            blue: 48,
        },
        Lab {
            l: 72.965576,
            a: 21.274925,
            b: 66.762276,
        },
        "742",
    ),
    (
        RgbColor {
            red: 238,
            green: 135,
            blue: 0,
        },
        Lab {
            l: 66.1427,
            a: 32.50742,
            b: 72.271065,
        },
        "741",
    ),
    (
        RgbColor {
            red: 219,
            green: 100,
            blue: 0,
        },
        Lab {
            l: 56.265083,
            a: 42.653145,
            b: 64.94853,
        },
        "740",
    ),
    (
        RgbColor {
            red: 231,
            green: 110,
            blue: 32,
        },
        Lab {
            l: 60.11258,
            a: 42.815716,
            b: 60.696053,
        },
        "970",
    ),
    (
        RgbColor {
            red: 199,
            green: 113,
            blue: 138,
        },
        Lab {
            l: 57.836403,
            a: 36.834656,
            b: 1.3061523,
        },
        "971",
    ),
    (
        RgbColor {
            red: 207,
            green: 80,
            blue: 20,
        },
        Lab {
            l: 50.75271,
            a: 47.82492,
            b: 55.868477,
        },
        "947",
    ),
    (
        RgbColor {
            red: 193,
            green: 75,
            blue: 30,
        },
        Lab {
            l: 47.57969,
            a: 45.210243,
            b: 48.52865,
        },
        "946",
    ),
    (
        RgbColor {
            red: 166,
            green: 54,
            blue: 31,
        },
        Lab {
            l: 39.31987,
            a: 44.87477,
            b: 38.6947,
        },
        "900",
    ),
    (
        RgbColor {
            red: 252,
            green: 202,
            blue: 185,
        },
        Lab {
            l: 85.221054,
            a: 15.282601,
            b: 15.21337,
        },
        "967",
    ),
    (
        RgbColor {
            red: 246,
            green: 170,
            blue: 151,
        },
        Lab {
            l: 76.42352,
            a: 25.64037,
            b: 21.077955,
        },
        "3824",
    ),
    (
        RgbColor {
            red: 234,
            green: 142,
            blue: 119,
        },
        Lab {
            l: 68.148094,
            a: 32.49836,
            b: 27.112747,
        },
        "3341",
    ),
    (
        RgbColor {
            red: 223,
            green: 108,
            blue: 73,
        },
        Lab {
            l: 58.879044,
            a: 42.32037,
            b: 40.255535,
        },
        "3340",
    ),
    (
        RgbColor {
            red: 204,
            green: 63,
            blue: 24,
        },
        Lab {
            l: 47.573772,
            a: 54.03024,
            b: 51.813667,
        },
        "608",
    ),
    (
        RgbColor {
            red: 174,
            green: 13,
            blue: 19,
        },
        Lab {
            l: 36.62257,
            a: 58.80216,
            b: 42.944164,
        },
        "606",
    ),
    (
        RgbColor {
            red: 238,
            green: 210,
            blue: 185,
        },
        Lab {
            l: 85.89353,
            a: 5.930811,
            b: 15.975273,
        },
        "951",
    ),
    (
        RgbColor {
            red: 233,
            green: 187,
            blue: 147,
        },
        Lab {
            l: 79.025604,
            a: 11.166721,
            b: 26.540207,
        },
        "3856",
    ),
    (
        RgbColor {
            red: 231,
            green: 141,
            blue: 91,
        },
        Lab {
            l: 67.124405,
            a: 29.573381,
            b: 40.808105,
        },
        "722",
    ),
    (
        RgbColor {
            red: 209,
            green: 98,
            blue: 43,
        },
        Lab {
            l: 54.51989,
            a: 40.5758,
            b: 50.04295,
        },
        "721",
    ),
    (
        RgbColor {
            red: 177,
            green: 67,
            blue: 2,
        },
        Lab {
            l: 43.312283,
            a: 42.482346,
            b: 53.516037,
        },
        "720",
    ),
    (
        RgbColor {
            red: 242,
            green: 168,
            blue: 126,
        },
        Lab {
            l: 75.07129,
            a: 22.554218,
            b: 32.60585,
        },
        "3825",
    ),
    (
        RgbColor {
            red: 190,
            green: 103,
            blue: 58,
        },
        Lab {
            l: 52.900406,
            a: 31.089275,
            b: 40.099632,
        },
        "922",
    ),
    (
        RgbColor {
            red: 161,
            green: 74,
            blue: 33,
        },
        Lab {
            l: 42.132633,
            a: 33.45233,
            b: 40.157097,
        },
        "921",
    ),
    (
        RgbColor {
            red: 135,
            green: 49,
            blue: 19,
        },
        Lab {
            l: 32.691105,
            a: 35.312637,
            b: 36.508896,
        },
        "920",
    ),
    (
        RgbColor {
            red: 113,
            green: 28,
            blue: 2,
        },
        Lab {
            l: 24.785004,
            a: 36.12079,
            b: 35.362255,
        },
        "919",
    ),
    (
        RgbColor {
            red: 108,
            green: 36,
            blue: 17,
        },
        Lab {
            l: 25.23257,
            a: 31.021536,
            b: 28.753685,
        },
        "918",
    ),
    (
        RgbColor {
            red: 248,
            green: 230,
            blue: 213,
        },
        Lab {
            l: 92.28949,
            a: 3.37857,
            b: 10.486805,
        },
        "3770",
    ),
    (
        RgbColor {
            red: 227,
            green: 194,
            blue: 167,
        },
        Lab {
            l: 80.575005,
            a: 7.692337,
            b: 17.934359,
        },
        "945",
    ),
    (
        RgbColor {
            red: 211,
            green: 145,
            blue: 101,
        },
        Lab {
            l: 65.78086,
            a: 20.114809,
            b: 33.305107,
        },
        "402",
    ),
    (
        RgbColor {
            red: 177,
            green: 100,
            blue: 52,
        },
        Lab {
            l: 50.34748,
            a: 26.986807,
            b: 39.87326,
        },
        "3776",
    ),
    (
        RgbColor {
            red: 143,
            green: 74,
            blue: 28,
        },
        Lab {
            l: 39.279312,
            a: 25.700628,
            b: 38.841026,
        },
        "301",
    ),
    (
        RgbColor {
            red: 102,
            green: 39,
            blue: 0,
        },
        Lab {
            l: 24.564968,
            a: 26.481897,
            b: 35.220997,
        },
        "400",
    ),
    (
        RgbColor {
            red: 74,
            green: 27,
            blue: 0,
        },
        Lab {
            l: 16.699543,
            a: 20.774931,
            b: 25.030521,
        },
        "300",
    ),
    (
        RgbColor {
            red: 251,
            green: 234,
            blue: 197,
        },
        Lab {
            l: 93.18372,
            a: 0.052183867,
            b: 20.078182,
        },
        "3823",
    ),
    (
        RgbColor {
            red: 249,
            green: 203,
            blue: 140,
        },
        Lab {
            l: 84.35341,
            a: 8.312106,
            b: 37.50856,
        },
        "3855",
    ),
    (
        RgbColor {
            red: 226,
            green: 154,
            blue: 93,
        },
        Lab {
            l: 69.52136,
            a: 20.771473,
            b: 42.521538,
        },
        "3854",
    ),
    (
        RgbColor {
            red: 205,
            green: 114,
            blue: 45,
        },
        Lab {
            l: 57.27526,
            a: 30.789524,
            b: 51.60982,
        },
        "3853",
    ),
    (
        RgbColor {
            red: 222,
            green: 163,
            blue: 105,
        },
        Lab {
            l: 71.38231,
            a: 15.265197,
            b: 38.503086,
        },
        "3827",
    ),
    (
        RgbColor {
            red: 214,
            green: 146,
            blue: 84,
        },
        Lab {
            l: 66.133224,
            a: 19.454569,
            b: 42.747505,
        },
        "977",
    ),
    (
        RgbColor {
            red: 181,
            green: 108,
            blue: 42,
        },
        Lab {
            l: 52.639275,
            a: 23.803503,
            b: 47.297455,
        },
        "976",
    ),
    (
        RgbColor {
            red: 151,
            green: 84,
            blue: 35,
        },
        Lab {
            l: 42.827255,
            a: 23.865997,
            b: 39.373146,
        },
        "3826",
    ),
    (
        RgbColor {
            red: 101,
            green: 45,
            blue: 8,
        },
        Lab {
            l: 25.747948,
            a: 22.928507,
            b: 33.020725,
        },
        "975",
    ),
    (
        RgbColor {
            red: 237,
            green: 210,
            blue: 191,
        },
        Lab {
            l: 85.93665,
            a: 6.4341426,
            b: 12.857866,
        },
        "948",
    ),
    (
        RgbColor {
            red: 228,
            green: 182,
            blue: 160,
        },
        Lab {
            l: 77.57272,
            a: 13.496637,
            b: 17.520023,
        },
        "754",
    ),
    (
        RgbColor {
            red: 214,
            green: 161,
            blue: 132,
        },
        Lab {
            l: 70.49256,
            a: 15.847087,
            b: 22.741688,
        },
        "3771",
    ),
    (
        RgbColor {
            red: 206,
            green: 152,
            blue: 132,
        },
        Lab {
            l: 67.46109,
            a: 17.573685,
            b: 18.460846,
        },
        "758",
    ),
    (
        RgbColor {
            red: 177,
            green: 110,
            blue: 93,
        },
        Lab {
            l: 53.266266,
            a: 24.690897,
            b: 20.663929,
        },
        "3778",
    ),
    (
        RgbColor {
            red: 152,
            green: 82,
            blue: 69,
        },
        Lab {
            l: 42.974686,
            a: 27.736217,
            b: 20.55037,
        },
        "356",
    ),
    (
        RgbColor {
            red: 135,
            green: 54,
            blue: 43,
        },
        Lab {
            l: 33.909225,
            a: 33.933117,
            b: 24.306274,
        },
        "3830",
    ),
    (
        RgbColor {
            red: 109,
            green: 27,
            blue: 22,
        },
        Lab {
            l: 23.978851,
            a: 35.68883,
            b: 24.561388,
        },
        "355",
    ),
    (
        RgbColor {
            red: 95,
            green: 13,
            blue: 17,
        },
        Lab {
            l: 19.063328,
            a: 35.769196,
            b: 21.398357,
        },
        "3777",
    ),
    (
        RgbColor {
            red: 222,
            green: 173,
            blue: 158,
        },
        Lab {
            l: 74.753624,
            a: 15.754729,
            b: 14.575362,
        },
        "3779",
    ),
    (
        RgbColor {
            red: 161,
            green: 111,
            blue: 93,
        },
        Lab {
            l: 51.538918,
            a: 17.53664,
            b: 18.000048,
        },
        "3859",
    ),
    (
        RgbColor {
            red: 99,
            green: 47,
            blue: 43,
        },
        Lab {
            l: 26.333809,
            a: 23.021385,
            b: 13.428778,
        },
        "3858",
    ),
    (
        RgbColor {
            red: 68,
            green: 21,
            blue: 8,
        },
        Lab {
            l: 14.304951,
            a: 21.861195,
            b: 18.486408,
        },
        "3857",
    ),
    (
        RgbColor {
            red: 229,
            green: 201,
            blue: 182,
        },
        Lab {
            l: 82.81222,
            a: 6.8816843,
            b: 13.108551,
        },
        "3774",
    ),
    (
        RgbColor {
            red: 221,
            green: 190,
            blue: 168,
        },
        Lab {
            l: 79.05709,
            a: 7.6825914,
            b: 15.1966095,
        },
        "950",
    ),
    (
        RgbColor {
            red: 171,
            green: 122,
            blue: 99,
        },
        Lab {
            l: 55.562363,
            a: 16.143053,
            b: 20.172382,
        },
        "3064",
    ),
    (
        RgbColor {
            red: 173,
            green: 130,
            blue: 114,
        },
        Lab {
            l: 58.099136,
            a: 14.300526,
            b: 15.101814,
        },
        "407",
    ),
    (
        RgbColor {
            red: 143,
            green: 74,
            blue: 28,
        },
        Lab {
            l: 39.279312,
            a: 25.700628,
            b: 38.841026,
        },
        "3773",
    ),
    (
        RgbColor {
            red: 138,
            green: 90,
            blue: 74,
        },
        Lab {
            l: 43.07921,
            a: 17.774343,
            b: 17.341507,
        },
        "3772",
    ),
    (
        RgbColor {
            red: 112,
            green: 66,
            blue: 53,
        },
        Lab {
            l: 33.132923,
            a: 18.398539,
            b: 16.3629,
        },
        "632",
    ),
    (
        RgbColor {
            red: 204,
            green: 193,
            blue: 185,
        },
        Lab {
            l: 78.7411,
            a: 2.470255,
            b: 5.401659,
        },
        "453",
    ),
    (
        RgbColor {
            red: 177,
            green: 161,
            blue: 157,
        },
        Lab {
            l: 67.48031,
            a: 5.126357,
            b: 4.192078,
        },
        "452",
    ),
    (
        RgbColor {
            red: 134,
            green: 116,
            blue: 111,
        },
        Lab {
            l: 50.34281,
            a: 6.1149893,
            b: 5.337155,
        },
        "451",
    ),
    (
        RgbColor {
            red: 155,
            green: 131,
            blue: 126,
        },
        Lab {
            l: 56.78985,
            a: 8.306683,
            b: 6.1030746,
        },
        "3861",
    ),
    (
        RgbColor {
            red: 108,
            green: 81,
            blue: 75,
        },
        Lab {
            l: 37.08034,
            a: 10.329053,
            b: 7.901168,
        },
        "3860",
    ),
    (
        RgbColor {
            red: 88,
            green: 60,
            blue: 55,
        },
        Lab {
            l: 28.333809,
            a: 11.576518,
            b: 8.025539,
        },
        "779",
    ),
    (
        RgbColor {
            red: 237,
            green: 223,
            blue: 201,
        },
        Lab {
            l: 89.39194,
            a: 1.2094975,
            b: 12.502146,
        },
        "712",
    ),
    (
        RgbColor {
            red: 232,
            green: 212,
            blue: 180,
        },
        Lab {
            l: 85.7755,
            a: 1.9630492,
            b: 18.366348,
        },
        "739",
    ),
    (
        RgbColor {
            red: 212,
            green: 180,
            blue: 140,
        },
        Lab {
            l: 75.16073,
            a: 5.807132,
            b: 24.71,
        },
        "738",
    ),
    (
        RgbColor {
            red: 204,
            green: 159,
            blue: 113,
        },
        Lab {
            l: 68.62613,
            a: 10.801495,
            b: 30.367994,
        },
        "437",
    ),
    (
        RgbColor {
            red: 172,
            green: 121,
            blue: 72,
        },
        Lab {
            l: 54.98427,
            a: 14.481038,
            b: 34.514767,
        },
        "436",
    ),
    (
        RgbColor {
            red: 153,
            green: 98,
            blue: 50,
        },
        Lab {
            l: 46.63031,
            a: 17.601967,
            b: 35.876892,
        },
        "435",
    ),
    (
        RgbColor {
            red: 121,
            green: 68,
            blue: 18,
        },
        Lab {
            l: 34.48518,
            a: 18.919155,
            b: 37.717793,
        },
        "434",
    ),
    (
        RgbColor {
            red: 92,
            green: 49,
            blue: 13,
        },
        Lab {
            l: 25.264301,
            a: 16.51986,
            b: 29.89341,
        },
        "433",
    ),
    (
        RgbColor {
            red: 76,
            green: 40,
            blue: 15,
        },
        Lab {
            l: 20.396294,
            a: 14.472112,
            b: 23.057207,
        },
        "801",
    ),
    (
        RgbColor {
            red: 63,
            green: 33,
            blue: 8,
        },
        Lab {
            l: 16.316422,
            a: 12.083351,
            b: 20.8641,
        },
        "898",
    ),
    (
        RgbColor {
            red: 51,
            green: 24,
            blue: 6,
        },
        Lab {
            l: 11.758911,
            a: 11.741824,
            b: 15.326056,
        },
        "938",
    ),
    (
        RgbColor {
            red: 31,
            green: 5,
            blue: 0,
        },
        Lab {
            l: 3.6123772,
            a: 9.801388,
            b: 5.5906653,
        },
        "3371",
    ),
    (
        RgbColor {
            red: 222,
            green: 201,
            blue: 184,
        },
        Lab {
            l: 82.251976,
            a: 4.6101513,
            b: 11.164581,
        },
        "543",
    ),
    (
        RgbColor {
            red: 191,
            green: 162,
            blue: 139,
        },
        Lab {
            l: 68.59312,
            a: 7.1296988,
            b: 15.927291,
        },
        "3864",
    ),
    (
        RgbColor {
            red: 147,
            green: 112,
            blue: 85,
        },
        Lab {
            l: 50.035072,
            a: 10.093271,
            b: 20.345604,
        },
        "3863",
    ),
    (
        RgbColor {
            red: 124,
            green: 88,
            blue: 59,
        },
        Lab {
            l: 40.54214,
            a: 11.171788,
            b: 22.630245,
        },
        "3862",
    ),
    (
        RgbColor {
            red: 57,
            green: 37,
            blue: 21,
        },
        Lab {
            l: 16.734787,
            a: 7.2874875,
            b: 14.385712,
        },
        "3031",
    ),
    (
        RgbColor {
            red: 251,
            green: 249,
            blue: 240,
        },
        Lab {
            l: 97.85402,
            a: -0.88238716,
            b: 4.5462847,
        },
        "3865",
    ),
    (
        RgbColor {
            red: 223,
            green: 211,
            blue: 191,
        },
        Lab {
            l: 85.01618,
            a: 0.83217025,
            b: 11.389149,
        },
        "822",
    ),
    (
        RgbColor {
            red: 200,
            green: 190,
            blue: 168,
        },
        Lab {
            l: 77.24133,
            a: -0.16784668,
            b: 12.378132,
        },
        "644",
    ),
    (
        RgbColor {
            red: 146,
            green: 134,
            blue: 110,
        },
        Lab {
            l: 56.374367,
            a: 0.48997998,
            b: 14.582407,
        },
        "642",
    ),
    (
        RgbColor {
            red: 123,
            green: 111,
            blue: 83,
        },
        Lab {
            l: 47.241898,
            a: 0.07069111,
            b: 17.329538,
        },
        "640",
    ),
    (
        RgbColor {
            red: 78,
            green: 72,
            blue: 54,
        },
        Lab {
            l: 30.690304,
            a: -0.7916093,
            b: 11.737167,
        },
        "3787",
    ),
    (
        RgbColor {
            red: 47,
            green: 38,
            blue: 26,
        },
        Lab {
            l: 15.8257885,
            a: 2.0412803,
            b: 9.54343,
        },
        "3021",
    ),
    (
        RgbColor {
            red: 202,
            green: 199,
            blue: 186,
        },
        Lab {
            l: 80.1508,
            a: -1.2441278,
            b: 6.872499,
        },
        "3024",
    ),
    (
        RgbColor {
            red: 165,
            green: 157,
            blue: 137,
        },
        Lab {
            l: 64.89884,
            a: -0.54496527,
            b: 11.517107,
        },
        "3023",
    ),
    (
        RgbColor {
            red: 135,
            green: 132,
            blue: 108,
        },
        Lab {
            l: 54.799713,
            a: -3.1428933,
            b: 13.392067,
        },
        "3022",
    ),
    (
        RgbColor {
            red: 72,
            green: 70,
            blue: 68,
        },
        Lab {
            l: 29.850777,
            a: 0.4068613,
            b: 1.4994919,
        },
        "535",
    ),
    (
        RgbColor {
            red: 213,
            green: 198,
            blue: 178,
        },
        Lab {
            l: 80.59924,
            a: 1.9642711,
            b: 11.916721,
        },
        "3033",
    ),
    (
        RgbColor {
            red: 180,
            green: 163,
            blue: 138,
        },
        Lab {
            l: 67.82359,
            a: 2.14383,
            b: 15.255201,
        },
        "3782",
    ),
    (
        RgbColor {
            red: 145,
            green: 128,
            blue: 100,
        },
        Lab {
            l: 54.426506,
            a: 2.0135343,
            b: 17.604565,
        },
        "3032",
    ),
    (
        RgbColor {
            red: 123,
            green: 100,
            blue: 77,
        },
        Lab {
            l: 44.033737,
            a: 5.700797,
            b: 16.53269,
        },
        "3790",
    ),
    (
        RgbColor {
            red: 72,
            green: 52,
            blue: 32,
        },
        Lab {
            l: 23.46598,
            a: 6.1117706,
            b: 16.1043,
        },
        "3781",
    ),
    (
        RgbColor {
            red: 229,
            green: 219,
            blue: 207,
        },
        Lab {
            l: 87.88505,
            a: 1.3872683,
            b: 7.11416,
        },
        "3866",
    ),
    (
        RgbColor {
            red: 187,
            green: 165,
            blue: 146,
        },
        Lab {
            l: 69.148636,
            a: 4.9583616,
            b: 12.790239,
        },
        "842",
    ),
    (
        RgbColor {
            red: 165,
            green: 140,
            blue: 121,
        },
        Lab {
            l: 60.01201,
            a: 6.419688,
            b: 13.654757,
        },
        "841",
    ),
    (
        RgbColor {
            red: 131,
            green: 106,
            blue: 85,
        },
        Lab {
            l: 46.697426,
            a: 6.711185,
            b: 15.549302,
        },
        "840",
    ),
    (
        RgbColor {
            red: 78,
            green: 56,
            blue: 39,
        },
        Lab {
            l: 25.547531,
            a: 7.287517,
            b: 14.437556,
        },
        "839",
    ),
    (
        RgbColor {
            red: 57,
            green: 33,
            blue: 23,
        },
        Lab {
            l: 15.589615,
            a: 10.237679,
            b: 11.564768,
        },
        "838",
    ),
    (
        RgbColor {
            red: 208,
            green: 208,
            blue: 196,
        },
        Lab {
            l: 83.18247,
            a: -2.123177,
            b: 5.9539795,
        },
        "3072",
    ),
    (
        RgbColor {
            red: 176,
            green: 168,
            blue: 155,
        },
        Lab {
            l: 69.185295,
            a: 0.5906522,
            b: 7.740581,
        },
        "648",
    ),
    (
        RgbColor {
            red: 158,
            green: 156,
            blue: 138,
        },
        Lab {
            l: 64.0646,
            a: -2.5093257,
            b: 9.695541,
        },
        "647",
    ),
    (
        RgbColor {
            red: 128,
            green: 123,
            blue: 105,
        },
        Lab {
            l: 51.582115,
            a: -1.3558567,
            b: 10.537815,
        },
        "646",
    ),
    (
        RgbColor {
            red: 89,
            green: 86,
            blue: 76,
        },
        Lab {
            l: 36.558044,
            a: -0.7599145,
            b: 6.259918,
        },
        "645",
    ),
    (
        RgbColor {
            red: 64,
            green: 57,
            blue: 51,
        },
        Lab {
            l: 24.498184,
            a: 1.8560886,
            b: 4.853463,
        },
        "844",
    ),
    (
        RgbColor {
            red: 221,
            green: 222,
            blue: 220,
        },
        Lab {
            l: 88.34364,
            a: -0.6965697,
            b: 0.8592844,
        },
        "762",
    ),
    (
        RgbColor {
            red: 145,
            green: 148,
            blue: 152,
        },
        Lab {
            l: 61.1911,
            a: -0.2913773,
            b: -2.4859905,
        },
        "415",
    ),
    (
        RgbColor {
            red: 151,
            green: 154,
            blue: 162,
        },
        Lab {
            l: 63.58924,
            a: 0.5180836,
            b: -4.5517206,
        },
        "318",
    ),
    (
        RgbColor {
            red: 112,
            green: 116,
            blue: 125,
        },
        Lab {
            l: 48.780067,
            a: 0.42116642,
            b: -5.4623127,
        },
        "414",
    ),
    (
        RgbColor {
            red: 182,
            green: 189,
            blue: 189,
        },
        Lab {
            l: 76.07797,
            a: -2.4200678,
            b: -0.8430004,
        },
        "168",
    ),
    (
        RgbColor {
            red: 125,
            green: 132,
            blue: 130,
        },
        Lab {
            l: 54.52645,
            a: -2.9878914,
            b: 0.1829505,
        },
        "169",
    ),
    (
        RgbColor {
            red: 76,
            green: 82,
            blue: 90,
        },
        Lab {
            l: 34.618664,
            a: -0.50516427,
            b: -5.4820538,
        },
        "317",
    ),
    (
        RgbColor {
            red: 58,
            green: 65,
            blue: 67,
        },
        Lab {
            l: 26.976479,
            a: -2.398178,
            b: -2.2031367,
        },
        "413",
    ),
    (
        RgbColor {
            red: 44,
            green: 44,
            blue: 42,
        },
        Lab {
            l: 17.936722,
            a: -0.46747923,
            b: 1.2965977,
        },
        "3799",
    ),
    (
        RgbColor {
            red: 0,
            green: 0,
            blue: 0,
        },
        Lab {
            l: 0.0,
            a: 0.0,
            b: 0.0,
        },
        "310",
    ),
    (
        RgbColor {
            red: 209,
            green: 208,
            blue: 205,
        },
        Lab {
            l: 83.48177,
            a: -0.19747019,
            b: 1.6062021,
        },
        "01",
    ),
    (
        RgbColor {
            red: 190,
            green: 190,
            blue: 189,
        },
        Lab {
            l: 76.94971,
            a: -0.18447638,
            b: 0.5033612,
        },
        "02",
    ),
    (
        RgbColor {
            red: 151,
            green: 152,
            blue: 156,
        },
        Lab {
            l: 62.874,
            a: 0.4221797,
            b: -2.2219539,
        },
        "03",
    ),
    (
        RgbColor {
            red: 116,
            green: 114,
            blue: 117,
        },
        Lab {
            l: 48.300323,
            a: 1.3726652,
            b: -1.383841,
        },
        "04",
    ),
    (
        RgbColor {
            red: 197,
            green: 183,
            blue: 173,
        },
        Lab {
            l: 75.298615,
            a: 3.2598078,
            b: 6.8688154,
        },
        "05",
    ),
    (
        RgbColor {
            red: 189,
            green: 174,
            blue: 163,
        },
        Lab {
            l: 72.051544,
            a: 3.5061836,
            b: 7.5847626,
        },
        "06",
    ),
    (
        RgbColor {
            red: 143,
            green: 125,
            blue: 111,
        },
        Lab {
            l: 53.65901,
            a: 4.540026,
            b: 10.170484,
        },
        "07",
    ),
    (
        RgbColor {
            red: 88,
            green: 68,
            blue: 55,
        },
        Lab {
            l: 30.60456,
            a: 6.4947157,
            b: 11.162466,
        },
        "08",
    ),
    (
        RgbColor {
            red: 65,
            green: 46,
            blue: 48,
        },
        Lab {
            l: 21.174782,
            a: 9.073928,
            b: 2.1397173,
        },
        "09",
    ),
    (
        RgbColor {
            red: 232,
            green: 226,
            blue: 189,
        },
        Lab {
            l: 89.497505,
            a: -3.9311051,
            b: 18.85463,
        },
        "10",
    ),
    (
        RgbColor {
            red: 232,
            green: 215,
            blue: 124,
        },
        Lab {
            l: 85.61161,
            a: -6.1813593,
            b: 46.78986,
        },
        "11",
    ),
    (
        RgbColor {
            red: 217,
            green: 200,
            blue: 84,
        },
        Lab {
            l: 80.0116,
            a: -7.887304,
            b: 58.586563,
        },
        "12",
    ),
    (
        RgbColor {
            red: 185,
            green: 211,
            blue: 159,
        },
        Lab {
            l: 81.514114,
            a: -17.882734,
            b: 22.945763,
        },
        "13",
    ),
    (
        RgbColor {
            red: 223,
            green: 226,
            blue: 165,
        },
        Lab {
            l: 88.35124,
            a: -10.591179,
            b: 29.5766,
        },
        "14",
    ),
    (
        RgbColor {
            red: 210,
            green: 212,
            blue: 141,
        },
        Lab {
            l: 83.2908,
            a: -11.500001,
            b: 34.96827,
        },
        "15",
    ),
    (
        RgbColor {
            red: 194,
            green: 198,
            blue: 91,
        },
        Lab {
            l: 77.646286,
            a: -15.891493,
            b: 52.27517,
        },
        "16",
    ),
    (
        RgbColor {
            red: 233,
            green: 198,
            blue: 95,
        },
        Lab {
            l: 81.03814,
            a: 0.5605817,
            b: 55.19204,
        },
        "17",
    ),
    (
        RgbColor {
            red: 225,
            green: 186,
            blue: 68,
        },
        Lab {
            l: 77.00785,
            a: 1.7995536,
            b: 62.381172,
        },
        "18",
    ),
    (
        RgbColor {
            red: 251,
            green: 192,
            blue: 124,
        },
        Lab {
            l: 81.656586,
            a: 13.3315325,
            b: 42.257034,
        },
        "19",
    ),
    (
        RgbColor {
            red: 247,
            green: 192,
            blue: 175,
        },
        Lab {
            l: 82.14744,
            a: 17.34218,
            b: 16.14356,
        },
        "20",
    ),
    (
        RgbColor {
            red: 178,
            green: 101,
            blue: 86,
        },
        Lab {
            l: 51.204796,
            a: 29.370426,
            b: 22.007496,
        },
        "21",
    ),
    (
        RgbColor {
            red: 131,
            green: 39,
            blue: 37,
        },
        Lab {
            l: 30.426388,
            a: 39.18737,
            b: 23.669703,
        },
        "22",
    ),
    (
        RgbColor {
            red: 249,
            green: 234,
            blue: 234,
        },
        Lab {
            l: 93.85143,
            a: 5.0659776,
            b: 1.8215537,
        },
        "23",
    ),
    (
        RgbColor {
            red: 233,
            green: 222,
            blue: 228,
        },
        Lab {
            l: 89.47161,
            a: 4.78971,
            b: -1.5884161,
        },
        "24",
    ),
    (
        RgbColor {
            red: 219,
            green: 213,
            blue: 224,
        },
        Lab {
            l: 86.021675,
            a: 4.0411353,
            b: -4.6738863,
        },
        "25",
    ),
    (
        RgbColor {
            red: 201,
            green: 200,
            blue: 218,
        },
        Lab {
            l: 81.17483,
            a: 3.759861,
            b: -8.803582,
        },
        "26",
    ),
    (
        RgbColor {
            red: 226,
            green: 223,
            blue: 225,
        },
        Lab {
            l: 89.101974,
            a: 1.3708472,
            b: -0.6172776,
        },
        "27",
    ),
    (
        RgbColor {
            red: 121,
            green: 114,
            blue: 132,
        },
        Lab {
            l: 49.210457,
            a: 6.406009,
            b: -8.886946,
        },
        "28",
    ),
    (
        RgbColor {
            red: 60,
            green: 46,
            blue: 68,
        },
        Lab {
            l: 21.327408,
            a: 11.321365,
            b: -11.351585,
        },
        "29",
    ),
    (
        RgbColor {
            red: 126,
            green: 125,
            blue: 158,
        },
        Lab {
            l: 53.56909,
            a: 7.630616,
            b: -17.499506,
        },
        "30",
    ),
    (
        RgbColor {
            red: 92,
            green: 91,
            blue: 128,
        },
        Lab {
            l: 40.10735,
            a: 9.340182,
            b: -20.5643,
        },
        "31",
    ),
    (
        RgbColor {
            red: 62,
            green: 64,
            blue: 99,
        },
        Lab {
            l: 28.357868,
            a: 8.67787,
            b: -20.882929,
        },
        "32",
    ),
    (
        RgbColor {
            red: 129,
            green: 74,
            blue: 126,
        },
        Lab {
            l: 39.700684,
            a: 31.708628,
            b: -19.641626,
        },
        "33",
    ),
    (
        RgbColor {
            red: 104,
            green: 23,
            blue: 92,
        },
        Lab {
            l: 24.730614,
            a: 43.073715,
            b: -21.606768,
        },
        "34",
    ),
    (
        RgbColor {
            red: 80,
            green: 17,
            blue: 65,
        },
        Lab {
            l: 17.865635,
            a: 34.409092,
            b: -14.383369,
        },
        "35",
    ),
];
