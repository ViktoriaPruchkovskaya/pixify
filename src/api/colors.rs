use std::f32::consts::PI;
use lab::Lab;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RGBColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl RGBColor {
    pub fn find_dmc(&self) -> (RGBColor, &str) {
        let mut color_diffs: Vec<(i32, (RGBColor, &str))> = vec![];
        let lab = Lab::from_rgb(&[self.red, self.green, self.blue]);
        for &(rgb, dmc) in RGB_TO_DMC.iter() {
            if rgb == *self {
                return (rgb, dmc);
            }
            let lab2 = Lab::from_rgb(&[rgb.red, rgb.green, rgb.blue]);
            let diff = Self::calculate_diff(lab, lab2) as i32;
            color_diffs.push((diff, (rgb, dmc)));
        }
        color_diffs.iter().min_by_key(|x| x.0).unwrap().1
    }

    fn calculate_diff(lab: Lab, lab2: Lab) -> f32 {
        let c1 = (lab.a.powi(2) + lab.b.powi(2)).sqrt();
        let c2 = (lab2.a.powi(2) + lab2.b.powi(2)).sqrt();
        let c_bar = (c1 + c2) / 2.0;

        let delta_l_prime = lab2.l - lab.l;
        let l_bar = (lab.l + lab2.l) / 2.0;
        let a_prime_1 = Self::get_a_prime(lab.a, c_bar);
        let a_prime_2 = Self::get_a_prime(lab2.a, c_bar);
        let c_prime_1 = (a_prime_1.powf(2.0) + lab.b.powf(2.0)).sqrt();
        let c_prime_2 = (a_prime_2.powf(2.0) + lab2.b.powf(2.0)).sqrt();
        let c_bar_prime = (c_prime_1 + c_prime_2) / 2.0;
        let delta_c_prime = c_prime_2 - c_prime_1;
        let s_sub_l = 1.0 + (0.015 * (l_bar - 50.0).powf(2.0)) / (20.0 + (l_bar - 50.0).powf(2.0)).sqrt();
        let s_sub_c = 1.0 + 0.045 * c_bar_prime;

        let h_prime_1 = Self::get_h_prime_fn(lab.b, a_prime_1);
        let h_prime_2 = Self::get_h_prime_fn(lab2.b, a_prime_2);

        let delta_h_prime = Self::get_delta_h_prime(c1, c2, h_prime_1, h_prime_2);

        let delta_uppercase_h_prime = 2.0 * (c_prime_1 * c_prime_2).sqrt() *
            (Self::degrees_to_radians(delta_h_prime) / 2.0).sin();

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
        1.0 - 0.17 * Self::degrees_to_radians(uppercase_h_bar_prime - 30.0).cos() +
            0.24 * Self::degrees_to_radians(2.0 * uppercase_h_bar_prime).cos() +
            0.32 * Self::degrees_to_radians(3.0 * uppercase_h_bar_prime + 6.0).cos() -
            0.20 * Self::degrees_to_radians(4.0 * uppercase_h_bar_prime - 63.0).cos()
    }

    fn get_r_sub_t(c_bar_prime: f32, uppercase_h_bar_prime: f32) -> f32 {
        -2.0 * (c_bar_prime.powi(7) / (c_bar_prime.powi(7) + 25f32.powi(7))).sqrt() *
            Self::degrees_to_radians(60.0 * (-((uppercase_h_bar_prime - 275.0) / 25.0).powi(2)).exp()).sin()
    }

    fn radians_to_degrees(radians: f32) -> f32 {
        radians * (180.0 / PI)
    }

    fn degrees_to_radians(degrees: f32) -> f32 {
        degrees * (PI / 180.0)
    }
}

mod tests {
    #[test]
    fn it_gets_dmc_color() {
        let color = super::RGBColor { red: 255, green: 29, blue: 30 };
        let (rgb, ..) = color.find_dmc();
        assert_eq!(rgb.red, 204);
        assert_eq!(rgb.green, 63);
        assert_eq!(rgb.blue, 24);
    }

    #[test]
    fn it_gets_existing_color() {
        let color = super::RGBColor { red: 255, green: 255, blue: 255 };
        let (rgb, ..) = color.find_dmc();
        assert_eq!(rgb.red, 255);
        assert_eq!(rgb.green, 255);
        assert_eq!(rgb.blue, 255);
    }
}

static RGB_TO_DMC: [(RGBColor, &str); 487] = [
    (RGBColor { red: 255, green: 255, blue: 255 }, "B5200"),
    (RGBColor { red: 243, green: 206, blue: 205 }, "3713"),
    (RGBColor { red: 234, green: 184, blue: 185 }, "761"),
    (RGBColor { red: 210, green: 140, blue: 142 }, "760"),
    (RGBColor { red: 189, green: 104, blue: 104 }, "3712"),
    (RGBColor { red: 167, green: 74, blue: 74 }, "3328"),
    (RGBColor { red: 146, green: 32, blue: 42 }, "347"),
    (RGBColor { red: 242, green: 193, blue: 176 }, "353"),
    (RGBColor { red: 217, green: 134, blue: 118 }, "352"),
    (RGBColor { red: 181, green: 73, blue: 61 }, "351"),
    (RGBColor { red: 166, green: 50, blue: 42 }, "350"),
    (RGBColor { red: 149, green: 10, blue: 30 }, "349"),
    (RGBColor { red: 143, green: 1, blue: 21 }, "817"),
    (RGBColor { red: 234, green: 155, blue: 175 }, "3708"),
    (RGBColor { red: 224, green: 123, blue: 141 }, "3706"),
    (RGBColor { red: 193, green: 68, blue: 80 }, "3705"),
    (RGBColor { red: 182, green: 46, blue: 59 }, "3801"),
    (RGBColor { red: 152, green: 0, blue: 21 }, "666"),
    (RGBColor { red: 126, green: 0, blue: 24 }, "321"),
    (RGBColor { red: 127, green: 0, blue: 45 }, "304"),
    (RGBColor { red: 103, green: 0, blue: 26 }, "498"),
    (RGBColor { red: 110, green: 0, blue: 29 }, "816"),
    (RGBColor { red: 81, green: 0, blue: 28 }, "815"),
    (RGBColor { red: 70, green: 4, blue: 27 }, "814"),
    (RGBColor { red: 232, green: 152, blue: 173 }, "894"),
    (RGBColor { red: 210, green: 99, blue: 124 }, "893"),
    (RGBColor { red: 203, green: 78, blue: 93 }, "892"),
    (RGBColor { red: 193, green: 57, blue: 85 }, "891"),
    (RGBColor { red: 247, green: 212, blue: 211 }, "818"),
    (RGBColor { red: 233, green: 153, blue: 178 }, "957"),
    (RGBColor { red: 211, green: 93, blue: 134 }, "956"),
    (RGBColor { red: 145, green: 50, blue: 68 }, "309"),
    (RGBColor { red: 244, green: 203, blue: 208 }, "963"),
    (RGBColor { red: 228, green: 170, blue: 186 }, "3716"),
    (RGBColor { red: 199, green: 128, blue: 147 }, "962"),
    (RGBColor { red: 191, green: 100, blue: 127 }, "961"),
    (RGBColor { red: 194, green: 111, blue: 128 }, "3833"),
    (RGBColor { red: 177, green: 71, blue: 93 }, "3832"),
    (RGBColor { red: 144, green: 37, blue: 59 }, "3831"),
    (RGBColor { red: 95, green: 2, blue: 36 }, "777"),
    (RGBColor { red: 252, green: 230, blue: 222 }, "819"),
    (RGBColor { red: 227, green: 165, blue: 175 }, "3326"),
    (RGBColor { red: 129, green: 74, blue: 126 }, "776"),
    (RGBColor { red: 199, green: 113, blue: 138 }, "899"),
    (RGBColor { red: 183, green: 84, blue: 112 }, "335"),
    (RGBColor { red: 122, green: 0, blue: 46 }, "326"),
    (RGBColor { red: 232, green: 187, blue: 196 }, "151"),
    (RGBColor { red: 215, green: 159, blue: 172 }, "3354"),
    (RGBColor { red: 193, green: 122, blue: 141 }, "3733"),
    (RGBColor { red: 170, green: 78, blue: 107 }, "3731"),
    (RGBColor { red: 136, green: 44, blue: 74 }, "3350"),
    (RGBColor { red: 117, green: 19, blue: 51 }, "150"),
    (RGBColor { red: 231, green: 191, blue: 205 }, "3689"),
    (RGBColor { red: 186, green: 129, blue: 152 }, "3688"),
    (RGBColor { red: 162, green: 90, blue: 118 }, "3687"),
    (RGBColor { red: 108, green: 42, blue: 71 }, "3803"),
    (RGBColor { red: 83, green: 13, blue: 43 }, "3685"),
    (RGBColor { red: 233, green: 189, blue: 206 }, "605"),
    (RGBColor { red: 226, green: 166, blue: 190 }, "604"),
    (RGBColor { red: 210, green: 132, blue: 165 }, "603"),
    (RGBColor { red: 187, green: 84, blue: 129 }, "602"),
    (RGBColor { red: 165, green: 48, blue: 97 }, "601"),
    (RGBColor { red: 147, green: 25, blue: 80 }, "600"),
    (RGBColor { red: 212, green: 140, blue: 172 }, "3806"),
    (RGBColor { red: 176, green: 73, blue: 126 }, "3805"),
    (RGBColor { red: 167, green: 57, blue: 112 }, "3804"),
    (RGBColor { red: 221, green: 181, blue: 210 }, "3609"),
    (RGBColor { red: 201, green: 148, blue: 183 }, "3608"),
    (RGBColor { red: 156, green: 76, blue: 141 }, "3607"),
    (RGBColor { red: 132, green: 27, blue: 97 }, "718"),
    (RGBColor { red: 137, green: 40, blue: 103 }, "917"),
    (RGBColor { red: 86, green: 0, blue: 43 }, "915"),
    (RGBColor { red: 238, green: 207, blue: 201 }, "225"),
    (RGBColor { red: 211, green: 169, blue: 163 }, "224"),
    (RGBColor { red: 195, green: 149, blue: 148 }, "152"),
    (RGBColor { red: 153, green: 94, blue: 101 }, "223"),
    (RGBColor { red: 140, green: 78, blue: 82 }, "3722"),
    (RGBColor { red: 121, green: 53, blue: 55 }, "3721"),
    (RGBColor { red: 99, green: 28, blue: 33 }, "221"),
    (RGBColor { red: 204, green: 168, blue: 170 }, "778"),
    (RGBColor { red: 196, green: 158, blue: 169 }, "3727"),
    (RGBColor { red: 172, green: 131, blue: 150 }, "316"),
    (RGBColor { red: 135, green: 86, blue: 94 }, "3726"),
    (RGBColor { red: 106, green: 56, blue: 65 }, "315"),
    (RGBColor { red: 85, green: 35, blue: 49 }, "3802"),
    (RGBColor { red: 94, green: 13, blue: 34 }, "902"),
    (RGBColor { red: 195, green: 188, blue: 196 }, "3743"),
    (RGBColor { red: 163, green: 153, blue: 165 }, "3042"),
    (RGBColor { red: 114, green: 92, blue: 106 }, "3041"),
    (RGBColor { red: 85, green: 60, blue: 72 }, "3740"),
    (RGBColor { red: 163, green: 138, blue: 164 }, "3836"),
    (RGBColor { red: 119, green: 87, blue: 121 }, "3835"),
    (RGBColor { red: 74, green: 39, blue: 72 }, "3834"),
    (RGBColor { red: 37, green: 3, blue: 31 }, "154"),
    (RGBColor { red: 204, green: 194, blue: 222 }, "211"),
    (RGBColor { red: 178, green: 165, blue: 205 }, "210"),
    (RGBColor { red: 149, green: 131, blue: 180 }, "209"),
    (RGBColor { red: 111, green: 84, blue: 151 }, "208"),
    (RGBColor { red: 84, green: 58, blue: 130 }, "3837"),
    (RGBColor { red: 89, green: 62, blue: 99 }, "327"),
    (RGBColor { red: 208, green: 187, blue: 211 }, "153"),
    (RGBColor { red: 182, green: 158, blue: 190 }, "554"),
    (RGBColor { red: 122, green: 95, blue: 150 }, "553"),
    (RGBColor { red: 101, green: 70, blue: 127 }, "552"),
    (RGBColor { red: 53, green: 13, blue: 71 }, "550"),
    (RGBColor { red: 200, green: 206, blue: 218 }, "3747"),
    (RGBColor { red: 156, green: 175, blue: 201 }, "341"),
    (RGBColor { red: 129, green: 151, blue: 186 }, "156"),
    (RGBColor { red: 119, green: 131, blue: 180 }, "340"),
    (RGBColor { red: 120, green: 127, blue: 178 }, "155"),
    (RGBColor { red: 85, green: 95, blue: 154 }, "3746"),
    (RGBColor { red: 60, green: 62, blue: 129 }, "333"),
    (RGBColor { red: 165, green: 187, blue: 207 }, "157"),
    (RGBColor { red: 132, green: 160, blue: 186 }, "794"),
    (RGBColor { red: 102, green: 124, blue: 156 }, "793"),
    (RGBColor { red: 73, green: 93, blue: 137 }, "3807"),
    (RGBColor { red: 49, green: 77, blue: 130 }, "792"),
    (RGBColor { red: 43, green: 64, blue: 108 }, "158"),
    (RGBColor { red: 12, green: 37, blue: 87 }, "791"),
    (RGBColor { red: 167, green: 193, blue: 217 }, "3840"),
    (RGBColor { red: 116, green: 144, blue: 187 }, "3839"),
    (RGBColor { red: 91, green: 121, blue: 170 }, "3838"),
    (RGBColor { red: 167, green: 194, blue: 213 }, "800"),
    (RGBColor { red: 119, green: 135, blue: 160 }, "809"),
    (RGBColor { red: 102, green: 140, blue: 175 }, "799"),
    (RGBColor { red: 48, green: 101, blue: 152 }, "798"),
    (RGBColor { red: 29, green: 65, blue: 121 }, "797"),
    (RGBColor { red: 0, green: 42, blue: 101 }, "796"),
    (RGBColor { red: 0, green: 26, blue: 83 }, "820"),
    (RGBColor { red: 200, green: 219, blue: 223 }, "162"),
    (RGBColor { red: 172, green: 200, blue: 213 }, "827"),
    (RGBColor { red: 124, green: 163, blue: 186 }, "813"),
    (RGBColor { red: 78, green: 127, blue: 162 }, "826"),
    (RGBColor { red: 6, green: 83, blue: 123 }, "825"),
    (RGBColor { red: 0, green: 60, blue: 103 }, "824"),
    (RGBColor { red: 58, green: 171, blue: 209 }, "996"),
    (RGBColor { red: 0, green: 136, blue: 183 }, "3843"),
    (RGBColor { red: 0, green: 123, blue: 175 }, "995"),
    (RGBColor { red: 56, green: 196, blue: 211 }, "3846"),
    (RGBColor { red: 0, green: 171, blue: 189 }, "3845"),
    (RGBColor { red: 0, green: 136, blue: 161 }, "3844"),
    (RGBColor { red: 166, green: 173, blue: 190 }, "159"),
    (RGBColor { red: 119, green: 135, blue: 160 }, "160"),
    (RGBColor { red: 81, green: 96, blue: 124 }, "161"),
    (RGBColor { red: 229, green: 232, blue: 226 }, "3756"),
    (RGBColor { red: 199, green: 214, blue: 217 }, "775"),
    (RGBColor { red: 191, green: 210, blue: 217 }, "3841"),
    (RGBColor { red: 161, green: 189, blue: 205 }, "3325"),
    (RGBColor { red: 134, green: 171, blue: 194 }, "3755"),
    (RGBColor { red: 91, green: 136, blue: 164 }, "334"),
    (RGBColor { red: 87, green: 126, blue: 162 }, "322"),
    (RGBColor { red: 14, green: 68, blue: 105 }, "312"),
    (RGBColor { red: 0, green: 53, blue: 88 }, "803"),
    (RGBColor { red: 15, green: 42, blue: 71 }, "336"),
    (RGBColor { red: 9, green: 13, blue: 37 }, "823"),
    (RGBColor { red: 11, green: 12, blue: 28 }, "939"),
    (RGBColor { red: 199, green: 211, blue: 215 }, "3753"),
    (RGBColor { red: 173, green: 191, blue: 199 }, "3752"),
    (RGBColor { red: 139, green: 161, blue: 175 }, "932"),
    (RGBColor { red: 91, green: 122, blue: 142 }, "931"),
    (RGBColor { red: 42, green: 69, blue: 81 }, "930"),
    (RGBColor { red: 13, green: 51, blue: 70 }, "3750"),
    (RGBColor { red: 195, green: 217, blue: 217 }, "828"),
    (RGBColor { red: 177, green: 209, blue: 213 }, "3761"),
    (RGBColor { red: 144, green: 183, blue: 196 }, "519"),
    (RGBColor { red: 86, green: 146, blue: 161 }, "518"),
    (RGBColor { red: 66, green: 129, blue: 152 }, "3760"),
    (RGBColor { red: 11, green: 94, blue: 129 }, "517"),
    (RGBColor { red: 0, green: 72, blue: 104 }, "3842"),
    (RGBColor { red: 0, green: 56, blue: 84 }, "311"),
    (RGBColor { red: 205, green: 228, blue: 227 }, "747"),
    (RGBColor { red: 144, green: 190, blue: 193 }, "3766"),
    (RGBColor { red: 85, green: 147, blue: 156 }, "807"),
    (RGBColor { red: 49, green: 94, blue: 70 }, "806"),
    (RGBColor { red: 0, green: 96, blue: 121 }, "3765"),
    (RGBColor { red: 182, green: 210, blue: 206 }, "3811"),
    (RGBColor { red: 154, green: 191, blue: 187 }, "598"),
    (RGBColor { red: 106, green: 156, blue: 161 }, "597"),
    (RGBColor { red: 70, green: 133, blue: 136 }, "3810"),
    (RGBColor { red: 1, green: 97, blue: 99 }, "3809"),
    (RGBColor { red: 25, green: 78, blue: 80 }, "3808"),
    (RGBColor { red: 200, green: 205, blue: 198 }, "928"),
    (RGBColor { red: 159, green: 171, blue: 163 }, "927"),
    (RGBColor { red: 121, green: 137, blue: 133 }, "926"),
    (RGBColor { red: 73, green: 90, blue: 95 }, "3768"),
    (RGBColor { red: 31, green: 63, blue: 66 }, "924"),
    (RGBColor { red: 102, green: 158, blue: 148 }, "3849"),
    (RGBColor { red: 47, green: 121, blue: 108 }, "3848"),
    (RGBColor { red: 0, green: 82, blue: 69 }, "3847"),
    (RGBColor { red: 168, green: 212, blue: 199 }, "964"),
    (RGBColor { red: 117, green: 183, blue: 159 }, "959"),
    (RGBColor { red: 102, green: 171, blue: 146 }, "958"),
    (RGBColor { red: 0, green: 136, blue: 103 }, "3812"),
    (RGBColor { red: 89, green: 169, blue: 132 }, "3851"),
    (RGBColor { red: 18, green: 140, blue: 100 }, "943"),
    (RGBColor { red: 0, green: 124, blue: 77 }, "3850"),
    (RGBColor { red: 129, green: 177, blue: 151 }, "993"),
    (RGBColor { red: 96, green: 153, blue: 125 }, "992"),
    (RGBColor { red: 44, green: 120, blue: 98 }, "3814"),
    (RGBColor { red: 0, green: 97, blue: 71 }, "991"),
    (RGBColor { red: 167, green: 188, blue: 156 }, "966"),
    (RGBColor { red: 174, green: 199, blue: 167 }, "564"),
    (RGBColor { red: 150, green: 183, blue: 149 }, "563"),
    (RGBColor { red: 92, green: 134, blue: 94 }, "562"),
    (RGBColor { red: 55, green: 99, blue: 61 }, "505"),
    (RGBColor { red: 165, green: 187, blue: 168 }, "3817"),
    (RGBColor { red: 118, green: 154, blue: 133 }, "3816"),
    (RGBColor { red: 85, green: 122, blue: 96 }, "163"),
    (RGBColor { red: 76, green: 114, blue: 87 }, "3815"),
    (RGBColor { red: 49, green: 94, blue: 70 }, "561"),
    (RGBColor { red: 154, green: 115, blue: 30 }, "504"),
    (RGBColor { red: 179, green: 196, blue: 182 }, "3813"),
    (RGBColor { red: 140, green: 167, blue: 154 }, "503"),
    (RGBColor { red: 116, green: 140, blue: 124 }, "502"),
    (RGBColor { red: 83, green: 106, blue: 94 }, "501"),
    (RGBColor { red: 19, green: 37, blue: 25 }, "500"),
    (RGBColor { red: 190, green: 218, blue: 177 }, "955"),
    (RGBColor { red: 157, green: 196, blue: 147 }, "954"),
    (RGBColor { red: 127, green: 173, blue: 119 }, "913"),
    (RGBColor { red: 97, green: 155, blue: 99 }, "912"),
    (RGBColor { red: 57, green: 132, blue: 68 }, "911"),
    (RGBColor { red: 30, green: 108, blue: 43 }, "910"),
    (RGBColor { red: 0, green: 85, blue: 20 }, "909"),
    (RGBColor { red: 0, green: 64, blue: 21 }, "3818"),
    (RGBColor { red: 194, green: 204, blue: 168 }, "369"),
    (RGBColor { red: 143, green: 159, blue: 116 }, "368"),
    (RGBColor { red: 105, green: 127, blue: 91 }, "320"),
    (RGBColor { red: 72, green: 97, blue: 62 }, "367"),
    (RGBColor { red: 23, green: 52, blue: 24 }, "319"),
    (RGBColor { red: 13, green: 29, blue: 0 }, "890"),
    (RGBColor { red: 166, green: 183, blue: 132 }, "164"),
    (RGBColor { red: 127, green: 143, blue: 80 }, "989"),
    (RGBColor { red: 110, green: 126, blue: 64 }, "988"),
    (RGBColor { red: 72, green: 92, blue: 44 }, "987"),
    (RGBColor { red: 34, green: 66, blue: 20 }, "986"),
    (RGBColor { red: 219, green: 219, blue: 178 }, "772"),
    (RGBColor { red: 189, green: 180, blue: 115 }, "3348"),
    (RGBColor { red: 115, green: 121, blue: 59 }, "3347"),
    (RGBColor { red: 90, green: 97, blue: 43 }, "3346"),
    (RGBColor { red: 43, green: 58, blue: 6 }, "3345"),
    (RGBColor { red: 33, green: 58, blue: 22 }, "895"),
    (RGBColor { red: 145, green: 161, blue: 57 }, "704"),
    (RGBColor { red: 127, green: 153, blue: 64 }, "703"),
    (RGBColor { red: 93, green: 135, blue: 56 }, "702"),
    (RGBColor { red: 66, green: 120, blue: 41 }, "701"),
    (RGBColor { red: 40, green: 105, blue: 35 }, "700"),
    (RGBColor { red: 0, green: 79, blue: 17 }, "699"),
    (RGBColor { red: 164, green: 174, blue: 40 }, "907"),
    (RGBColor { red: 103, green: 135, blue: 0 }, "906"),
    (RGBColor { red: 79, green: 102, blue: 17 }, "905"),
    (RGBColor { red: 62, green: 81, blue: 15 }, "904"),
    (RGBColor { red: 193, green: 179, blue: 98 }, "472"),
    (RGBColor { red: 144, green: 140, blue: 72 }, "471"),
    (RGBColor { red: 117, green: 123, blue: 35 }, "470"),
    (RGBColor { red: 94, green: 92, blue: 23 }, "469"),
    (RGBColor { red: 81, green: 85, blue: 19 }, "937"),
    (RGBColor { red: 66, green: 66, blue: 28 }, "936"),
    (RGBColor { red: 51, green: 55, blue: 28 }, "935"),
    (RGBColor { red: 39, green: 39, blue: 22 }, "934"),
    (RGBColor { red: 162, green: 162, blue: 134 }, "523"),
    (RGBColor { red: 157, green: 154, blue: 112 }, "3053"),
    (RGBColor { red: 132, green: 128, blue: 90 }, "3052"),
    (RGBColor { red: 72, green: 69, blue: 29 }, "3051"),
    (RGBColor { red: 179, green: 176, blue: 148 }, "524"),
    (RGBColor { red: 150, green: 154, blue: 126 }, "522"),
    (RGBColor { red: 68, green: 80, blue: 54 }, "520"),
    (RGBColor { red: 135, green: 135, blue: 89 }, "3364"),
    (RGBColor { red: 108, green: 113, blue: 77 }, "3363"),
    (RGBColor { red: 82, green: 90, blue: 59 }, "3362"),
    (RGBColor { red: 218, green: 199, blue: 116 }, "165"),
    (RGBColor { red: 213, green: 194, blue: 90 }, "3819"),
    (RGBColor { red: 185, green: 162, blue: 22 }, "166"),
    (RGBColor { red: 136, green: 132, blue: 32 }, "581"),
    (RGBColor { red: 103, green: 94, blue: 23 }, "580"),
    (RGBColor { red: 187, green: 156, blue: 74 }, "734"),
    (RGBColor { red: 165, green: 135, blue: 44 }, "733"),
    (RGBColor { red: 116, green: 92, blue: 1 }, "732"),
    (RGBColor { red: 213, green: 198, blue: 178 }, "731"),
    (RGBColor { red: 90, green: 72, blue: 8 }, "730"),
    (RGBColor { red: 176, green: 161, blue: 113 }, "3013"),
    (RGBColor { red: 134, green: 117, blue: 65 }, "3012"),
    (RGBColor { red: 102, green: 88, blue: 47 }, "3011"),
    (RGBColor { red: 179, green: 161, blue: 117 }, "372"),
    (RGBColor { red: 158, green: 133, blue: 82 }, "371"),
    (RGBColor { red: 159, green: 136, blue: 82 }, "370"),
    (RGBColor { red: 198, green: 163, blue: 87 }, "834"),
    (RGBColor { red: 174, green: 136, blue: 54 }, "833"),
    (RGBColor { red: 154, green: 115, blue: 30 }, "832"),
    (RGBColor { red: 127, green: 95, blue: 22 }, "831"),
    (RGBColor { red: 103, green: 75, blue: 16 }, "830"),
    (RGBColor { red: 95, green: 61, blue: 13 }, "829"),
    (RGBColor { red: 214, green: 201, blue: 177 }, "613"),
    (RGBColor { red: 173, green: 148, blue: 109 }, "612"),
    (RGBColor { red: 112, green: 90, blue: 57 }, "611"),
    (RGBColor { red: 107, green: 82, blue: 45 }, "610"),
    (RGBColor { red: 225, green: 206, blue: 163 }, "3047"),
    (RGBColor { red: 208, green: 181, blue: 126 }, "3046"),
    (RGBColor { red: 169, green: 131, blue: 82 }, "3045"),
    (RGBColor { red: 149, green: 107, blue: 58 }, "167"),
    (RGBColor { red: 245, green: 235, blue: 207 }, "746"),
    (RGBColor { red: 233, green: 209, blue: 160 }, "677"),
    (RGBColor { red: 191, green: 157, blue: 109 }, "422"),
    (RGBColor { red: 168, green: 130, blue: 79 }, "3828"),
    (RGBColor { red: 135, green: 90, blue: 40 }, "420"),
    (RGBColor { red: 112, green: 76, blue: 30 }, "869"),
    (RGBColor { red: 226, green: 165, blue: 56 }, "728"),
    (RGBColor { red: 196, green: 129, blue: 28 }, "783"),
    (RGBColor { red: 153, green: 92, blue: 0 }, "782"),
    (RGBColor { red: 212, green: 140, blue: 172 }, "781"),
    (RGBColor { red: 132, green: 71, blue: 5 }, "780"),
    (RGBColor { red: 219, green: 175, blue: 109 }, "676"),
    (RGBColor { red: 184, green: 135, blue: 67 }, "729"),
    (RGBColor { red: 160, green: 114, blue: 47 }, "680"),
    (RGBColor { red: 148, green: 98, blue: 25 }, "3829"),
    (RGBColor { red: 236, green: 196, blue: 111 }, "3822"),
    (RGBColor { red: 221, green: 171, blue: 70 }, "3821"),
    (RGBColor { red: 208, green: 154, blue: 46 }, "3820"),
    (RGBColor { red: 192, green: 129, blue: 3 }, "3852"),
    (RGBColor { red: 255, green: 237, blue: 148 }, "445"),
    (RGBColor { red: 255, green: 212, blue: 56 }, "307"),
    (RGBColor { red: 250, green: 195, blue: 0 }, "973"),
    (RGBColor { red: 248, green: 189, blue: 0 }, "444"),
    (RGBColor { red: 250, green: 227, blue: 160 }, "3078"),
    (RGBColor { red: 252, green: 221, blue: 135 }, "727"),
    (RGBColor { red: 248, green: 200, blue: 83 }, "726"),
    (RGBColor { red: 238, green: 175, blue: 66 }, "725"),
    (RGBColor { red: 245, green: 158, blue: 0 }, "972"),
    (RGBColor { red: 253, green: 226, blue: 172 }, "745"),
    (RGBColor { red: 250, green: 209, blue: 130 }, "744"),
    (RGBColor { red: 248, green: 192, blue: 88 }, "743"),
    (RGBColor { red: 243, green: 162, blue: 48 }, "742"),
    (RGBColor { red: 238, green: 135, blue: 0 }, "741"),
    (RGBColor { red: 219, green: 100, blue: 0 }, "740"),
    (RGBColor { red: 231, green: 110, blue: 32 }, "970"),
    (RGBColor { red: 199, green: 113, blue: 138 }, "971"),
    (RGBColor { red: 207, green: 80, blue: 20 }, "947"),
    (RGBColor { red: 193, green: 75, blue: 30 }, "946"),
    (RGBColor { red: 166, green: 54, blue: 31 }, "900"),
    (RGBColor { red: 252, green: 202, blue: 185 }, "967"),
    (RGBColor { red: 246, green: 170, blue: 151 }, "3824"),
    (RGBColor { red: 234, green: 142, blue: 119 }, "3341"),
    (RGBColor { red: 223, green: 108, blue: 73 }, "3340"),
    (RGBColor { red: 204, green: 63, blue: 24 }, "608"),
    (RGBColor { red: 174, green: 13, blue: 19 }, "606"),
    (RGBColor { red: 238, green: 210, blue: 185 }, "951"),
    (RGBColor { red: 233, green: 187, blue: 147 }, "3856"),
    (RGBColor { red: 231, green: 141, blue: 91 }, "722"),
    (RGBColor { red: 209, green: 98, blue: 43 }, "721"),
    (RGBColor { red: 177, green: 67, blue: 2 }, "720"),
    (RGBColor { red: 242, green: 168, blue: 126 }, "3825"),
    (RGBColor { red: 190, green: 103, blue: 58 }, "922"),
    (RGBColor { red: 161, green: 74, blue: 33 }, "921"),
    (RGBColor { red: 135, green: 49, blue: 19 }, "920"),
    (RGBColor { red: 113, green: 28, blue: 2 }, "919"),
    (RGBColor { red: 108, green: 36, blue: 17 }, "918"),
    (RGBColor { red: 248, green: 230, blue: 213 }, "3770"),
    (RGBColor { red: 227, green: 194, blue: 167 }, "945"),
    (RGBColor { red: 211, green: 145, blue: 101 }, "402"),
    (RGBColor { red: 177, green: 100, blue: 52 }, "3776"),
    (RGBColor { red: 143, green: 74, blue: 28 }, "301"),
    (RGBColor { red: 102, green: 39, blue: 0 }, "400"),
    (RGBColor { red: 74, green: 27, blue: 0 }, "300"),
    (RGBColor { red: 251, green: 234, blue: 197 }, "3823"),
    (RGBColor { red: 249, green: 203, blue: 140 }, "3855"),
    (RGBColor { red: 226, green: 154, blue: 93 }, "3854"),
    (RGBColor { red: 205, green: 114, blue: 45 }, "3853"),
    (RGBColor { red: 222, green: 163, blue: 105 }, "3827"),
    (RGBColor { red: 214, green: 146, blue: 84 }, "977"),
    (RGBColor { red: 181, green: 108, blue: 42 }, "976"),
    (RGBColor { red: 151, green: 84, blue: 35 }, "3826"),
    (RGBColor { red: 101, green: 45, blue: 8 }, "975"),
    (RGBColor { red: 237, green: 210, blue: 191 }, "948"),
    (RGBColor { red: 228, green: 182, blue: 160 }, "754"),
    (RGBColor { red: 214, green: 161, blue: 132 }, "3771"),
    (RGBColor { red: 206, green: 152, blue: 132 }, "758"),
    (RGBColor { red: 177, green: 110, blue: 93 }, "3778"),
    (RGBColor { red: 152, green: 82, blue: 69 }, "356"),
    (RGBColor { red: 135, green: 54, blue: 43 }, "3830"),
    (RGBColor { red: 109, green: 27, blue: 22 }, "355"),
    (RGBColor { red: 95, green: 13, blue: 17 }, "3777"),
    (RGBColor { red: 222, green: 173, blue: 158 }, "3779"),
    (RGBColor { red: 161, green: 111, blue: 93 }, "3859"),
    (RGBColor { red: 99, green: 47, blue: 43 }, "3858"),
    (RGBColor { red: 68, green: 21, blue: 8 }, "3857"),
    (RGBColor { red: 229, green: 201, blue: 182 }, "3774"),
    (RGBColor { red: 221, green: 190, blue: 168 }, "950"),
    (RGBColor { red: 171, green: 122, blue: 99 }, "3064"),
    (RGBColor { red: 173, green: 130, blue: 114 }, "407"),
    (RGBColor { red: 143, green: 74, blue: 28 }, "3773"),
    (RGBColor { red: 138, green: 90, blue: 74 }, "3772"),
    (RGBColor { red: 112, green: 66, blue: 53 }, "632"),
    (RGBColor { red: 204, green: 193, blue: 185 }, "453"),
    (RGBColor { red: 177, green: 161, blue: 157 }, "452"),
    (RGBColor { red: 134, green: 116, blue: 111 }, "451"),
    (RGBColor { red: 155, green: 131, blue: 126 }, "3861"),
    (RGBColor { red: 108, green: 81, blue: 75 }, "3860"),
    (RGBColor { red: 88, green: 60, blue: 55 }, "779"),
    (RGBColor { red: 237, green: 223, blue: 201 }, "712"),
    (RGBColor { red: 232, green: 212, blue: 180 }, "739"),
    (RGBColor { red: 212, green: 180, blue: 140 }, "738"),
    (RGBColor { red: 204, green: 159, blue: 113 }, "437"),
    (RGBColor { red: 172, green: 121, blue: 72 }, "436"),
    (RGBColor { red: 153, green: 98, blue: 50 }, "435"),
    (RGBColor { red: 121, green: 68, blue: 18 }, "434"),
    (RGBColor { red: 92, green: 49, blue: 13 }, "433"),
    (RGBColor { red: 76, green: 40, blue: 15 }, "801"),
    (RGBColor { red: 63, green: 33, blue: 8 }, "898"),
    (RGBColor { red: 51, green: 24, blue: 6 }, "938"),
    (RGBColor { red: 31, green: 5, blue: 0 }, "3371"),
    (RGBColor { red: 222, green: 201, blue: 184 }, "543"),
    (RGBColor { red: 191, green: 162, blue: 139 }, "3864"),
    (RGBColor { red: 147, green: 112, blue: 85 }, "3863"),
    (RGBColor { red: 124, green: 88, blue: 59 }, "3862"),
    (RGBColor { red: 57, green: 37, blue: 21 }, "3031"),
    (RGBColor { red: 251, green: 249, blue: 240 }, "3865"),
    (RGBColor { red: 223, green: 211, blue: 191 }, "822"),
    (RGBColor { red: 200, green: 190, blue: 168 }, "644"),
    (RGBColor { red: 146, green: 134, blue: 110 }, "642"),
    (RGBColor { red: 123, green: 111, blue: 83 }, "640"),
    (RGBColor { red: 78, green: 72, blue: 54 }, "3787"),
    (RGBColor { red: 47, green: 38, blue: 26 }, "3021"),
    (RGBColor { red: 202, green: 199, blue: 186 }, "3024"),
    (RGBColor { red: 165, green: 157, blue: 137 }, "3023"),
    (RGBColor { red: 135, green: 132, blue: 108 }, "3022"),
    (RGBColor { red: 72, green: 70, blue: 68 }, "535"),
    (RGBColor { red: 213, green: 198, blue: 178 }, "3033"),
    (RGBColor { red: 180, green: 163, blue: 138 }, "3782"),
    (RGBColor { red: 145, green: 128, blue: 100 }, "3032"),
    (RGBColor { red: 123, green: 100, blue: 77 }, "3790"),
    (RGBColor { red: 72, green: 52, blue: 32 }, "3781"),
    (RGBColor { red: 229, green: 219, blue: 207 }, "3866"),
    (RGBColor { red: 187, green: 165, blue: 146 }, "842"),
    (RGBColor { red: 165, green: 140, blue: 121 }, "841"),
    (RGBColor { red: 131, green: 106, blue: 85 }, "840"),
    (RGBColor { red: 78, green: 56, blue: 39 }, "839"),
    (RGBColor { red: 57, green: 33, blue: 23 }, "838"),
    (RGBColor { red: 208, green: 208, blue: 196 }, "3072"),
    (RGBColor { red: 176, green: 168, blue: 155 }, "648"),
    (RGBColor { red: 158, green: 156, blue: 138 }, "647"),
    (RGBColor { red: 128, green: 123, blue: 105 }, "646"),
    (RGBColor { red: 89, green: 86, blue: 76 }, "645"),
    (RGBColor { red: 64, green: 57, blue: 51 }, "844"),
    (RGBColor { red: 221, green: 222, blue: 220 }, "762"),
    (RGBColor { red: 145, green: 148, blue: 152 }, "415"),
    (RGBColor { red: 151, green: 154, blue: 162 }, "318"),
    (RGBColor { red: 112, green: 116, blue: 125 }, "414"),
    (RGBColor { red: 182, green: 189, blue: 189 }, "168"),
    (RGBColor { red: 125, green: 132, blue: 130 }, "169"),
    (RGBColor { red: 76, green: 82, blue: 90 }, "317"),
    (RGBColor { red: 58, green: 65, blue: 67 }, "413"),
    (RGBColor { red: 44, green: 44, blue: 42 }, "3799"),
    (RGBColor { red: 0, green: 0, blue: 0 }, "310"),
    (RGBColor { red: 209, green: 208, blue: 205 }, "01"),
    (RGBColor { red: 190, green: 190, blue: 189 }, "02"),
    (RGBColor { red: 151, green: 152, blue: 156 }, "03"),
    (RGBColor { red: 116, green: 114, blue: 117 }, "04"),
    (RGBColor { red: 197, green: 183, blue: 173 }, "05"),
    (RGBColor { red: 189, green: 174, blue: 163 }, "06"),
    (RGBColor { red: 143, green: 125, blue: 111 }, "07"),
    (RGBColor { red: 88, green: 68, blue: 55 }, "08"),
    (RGBColor { red: 65, green: 46, blue: 48 }, "09"),
    (RGBColor { red: 232, green: 226, blue: 189 }, "10"),
    (RGBColor { red: 232, green: 215, blue: 124 }, "11"),
    (RGBColor { red: 217, green: 200, blue: 84 }, "12"),
    (RGBColor { red: 185, green: 211, blue: 159 }, "13"),
    (RGBColor { red: 223, green: 226, blue: 165 }, "14"),
    (RGBColor { red: 210, green: 212, blue: 141 }, "15"),
    (RGBColor { red: 194, green: 198, blue: 91 }, "16"),
    (RGBColor { red: 233, green: 198, blue: 95 }, "17"),
    (RGBColor { red: 225, green: 186, blue: 68 }, "18"),
    (RGBColor { red: 251, green: 192, blue: 124 }, "19"),
    (RGBColor { red: 247, green: 192, blue: 175 }, "20"),
    (RGBColor { red: 178, green: 101, blue: 86 }, "21"),
    (RGBColor { red: 131, green: 39, blue: 37 }, "22"),
    (RGBColor { red: 249, green: 234, blue: 234 }, "23"),
    (RGBColor { red: 233, green: 222, blue: 228 }, "24"),
    (RGBColor { red: 219, green: 213, blue: 224 }, "25"),
    (RGBColor { red: 201, green: 200, blue: 218 }, "26"),
    (RGBColor { red: 226, green: 223, blue: 225 }, "27"),
    (RGBColor { red: 121, green: 114, blue: 132 }, "28"),
    (RGBColor { red: 60, green: 46, blue: 68 }, "29"),
    (RGBColor { red: 126, green: 125, blue: 158 }, "30"),
    (RGBColor { red: 92, green: 91, blue: 128 }, "31"),
    (RGBColor { red: 62, green: 64, blue: 99 }, "32"),
    (RGBColor { red: 129, green: 74, blue: 126 }, "33"),
    (RGBColor { red: 104, green: 23, blue: 92 }, "34"),
    (RGBColor { red: 80, green: 17, blue: 65 }, "35")]
;

